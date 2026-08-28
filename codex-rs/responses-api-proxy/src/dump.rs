use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::sha256::Sha256;
use reqwest::header::HeaderMap;
use serde::Serialize;
use tiny_http::Header;
use tiny_http::Method;

const REDACTED_HEADER_VALUE: &str = "[REDACTED]";
const DIGEST_SCHEMA: &str = "sha256_digest_v1";
const DUMP_RETENTION: Duration = Duration::from_secs(24 * 60 * 60);
const MAX_DUMP_FILES: usize = 256;

pub(crate) struct ExchangeDumper {
    dump_dir: PathBuf,
    next_sequence: AtomicU64,
}

impl ExchangeDumper {
    pub(crate) fn new(dump_dir: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(&dump_dir)?;
        secure_directory(&dump_dir)?;
        prune_expired_dumps(&dump_dir, SystemTime::now())?;
        ensure_capacity(&dump_dir, 2)?;

        Ok(Self {
            dump_dir,
            next_sequence: AtomicU64::new(1),
        })
    }

    pub(crate) fn dump_request(
        &self,
        method: &Method,
        url: &str,
        headers: &[Header],
        body: &[u8],
    ) -> io::Result<ExchangeDump> {
        ensure_capacity(&self.dump_dir, 2)?;
        let sequence = self.next_sequence.fetch_add(1, Ordering::Relaxed);
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_millis());
        let prefix = format!(
            "{timestamp_ms}-{}-{sequence:06}",
            std::process::id()
        );
        let request_path = self.dump_dir.join(format!("{prefix}-request.json"));
        let response_path = self.dump_dir.join(format!("{prefix}-response.json"));

        let request_dump = RequestDump {
            schema: "responses_api_proxy_request_digest_v1",
            method: method.as_str().to_string(),
            url: url.to_string(),
            headers: headers.iter().map(HeaderDump::from).collect(),
            body: BodyDigest::complete(body),
        };
        write_json_dump(&request_path, &request_dump)?;
        Ok(ExchangeDump { response_path })
    }
}

pub(crate) struct ExchangeDump {
    response_path: PathBuf,
}

impl ExchangeDump {
    pub(crate) fn tee_response_body<R: Read>(
        self,
        status: u16,
        headers: &HeaderMap,
        response_body: R,
    ) -> ResponseBodyDump<R> {
        ResponseBodyDump {
            response_body,
            response_path: self.response_path,
            status,
            headers: headers.iter().map(HeaderDump::from).collect(),
            hasher: Sha256::new(),
            byte_length: 0,
            complete: false,
            dump_written: false,
        }
    }
}

pub(crate) struct ResponseBodyDump<R> {
    response_body: R,
    response_path: PathBuf,
    status: u16,
    headers: Vec<HeaderDump>,
    hasher: Sha256,
    byte_length: u64,
    complete: bool,
    dump_written: bool,
}

impl<R> ResponseBodyDump<R> {
    fn write_dump_if_needed(&mut self) {
        if self.dump_written {
            return;
        }
        self.dump_written = true;
        let response_dump = ResponseDump {
            schema: "responses_api_proxy_response_digest_v1",
            status: self.status,
            headers: std::mem::take(&mut self.headers),
            body: BodyDigest {
                schema: DIGEST_SCHEMA,
                sha256: self.hasher.clone().finalize_hex(),
                byte_length: self.byte_length,
                complete: self.complete,
            },
        };
        if let Err(error) = write_json_dump(&self.response_path, &response_dump) {
            eprintln!(
                "responses-api-proxy failed to write {}: {error}",
                self.response_path.display()
            );
        }
    }
}

impl<R: Read> Read for ResponseBodyDump<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self.response_body.read(buffer) {
            Ok(0) => {
                self.complete = true;
                self.write_dump_if_needed();
                Ok(0)
            }
            Ok(bytes_read) => {
                self.hasher.update(&buffer[..bytes_read]);
                self.byte_length = match self.byte_length.checked_add(bytes_read as u64) {
                    Some(byte_length) => byte_length,
                    None => {
                        self.write_dump_if_needed();
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "response dump byte length overflow",
                        ));
                    }
                };
                Ok(bytes_read)
            }
            Err(error) => {
                self.write_dump_if_needed();
                Err(error)
            }
        }
    }
}

impl<R> Drop for ResponseBodyDump<R> {
    fn drop(&mut self) {
        self.write_dump_if_needed();
    }
}

#[derive(Serialize)]
struct RequestDump {
    schema: &'static str,
    method: String,
    url: String,
    headers: Vec<HeaderDump>,
    body: BodyDigest,
}

#[derive(Serialize)]
struct ResponseDump {
    schema: &'static str,
    status: u16,
    headers: Vec<HeaderDump>,
    body: BodyDigest,
}

#[derive(Serialize)]
struct BodyDigest {
    schema: &'static str,
    sha256: String,
    byte_length: u64,
    complete: bool,
}

impl BodyDigest {
    fn complete(body: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(body);
        Self {
            schema: DIGEST_SCHEMA,
            sha256: hasher.finalize_hex(),
            byte_length: body.len() as u64,
            complete: true,
        }
    }
}

#[derive(Debug, Serialize)]
struct HeaderDump {
    name: String,
    value: String,
}

impl From<&Header> for HeaderDump {
    fn from(header: &Header) -> Self {
        let name = header.field.as_str().to_string();
        let value = if should_redact_header(&name) {
            REDACTED_HEADER_VALUE.to_string()
        } else {
            header.value.as_str().to_string()
        };
        Self { name, value }
    }
}

impl From<(&reqwest::header::HeaderName, &reqwest::header::HeaderValue)> for HeaderDump {
    fn from(header: (&reqwest::header::HeaderName, &reqwest::header::HeaderValue)) -> Self {
        let name = header.0.as_str();
        let value = if should_redact_header(name) {
            REDACTED_HEADER_VALUE.to_string()
        } else {
            String::from_utf8_lossy(header.1.as_bytes()).into_owned()
        };
        Self {
            name: name.to_string(),
            value,
        }
    }
}

fn should_redact_header(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    [
        "authorization",
        "cookie",
        "token",
        "secret",
        "api-key",
        "apikey",
    ]
    .iter()
    .any(|fragment| name.contains(fragment))
}

fn write_json_dump(path: &Path, dump: &impl Serialize) -> io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(path)?;
    serde_json::to_writer_pretty(&mut file, dump)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    file.write_all(b"\n")?;
    file.sync_all()
}

fn ensure_capacity(directory: &Path, additional_files: usize) -> io::Result<()> {
    let count = fs::read_dir(directory)?
        .filter_map(Result::ok)
        .filter(|entry| is_json_path(&entry.path()))
        .count();
    if count.saturating_add(additional_files) > MAX_DUMP_FILES {
        return Err(io::Error::other(format!(
            "response dump file limit exceeded: maximum={MAX_DUMP_FILES}"
        )));
    }
    Ok(())
}

fn is_json_path(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension == std::ffi::OsStr::new("json"))
}

fn prune_expired_dumps(directory: &Path, now: SystemTime) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if !is_json_path(&path) {
            continue;
        }
        let metadata = entry.metadata()?;
        let Ok(modified) = metadata.modified() else {
            continue;
        };
        let Ok(age) = now.duration_since(modified) else {
            continue;
        };
        if age > DUMP_RETENTION {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn secure_directory(directory: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(directory)?.permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(directory, permissions)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use crate::sha256::digest_bytes;
    use pretty_assertions::assert_eq;
    use reqwest::header::AUTHORIZATION;
    use reqwest::header::CONTENT_TYPE;
    use reqwest::header::HeaderValue;
    use std::io::Cursor;
    use std::sync::atomic::AtomicU64;

    static NEXT_TEST_DIR: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn request_dump_contains_digest_only_and_redacts_secret_headers() {
        let dump_dir = test_dump_dir();
        let dumper = ExchangeDumper::new(dump_dir.clone()).expect("dumper");
        let headers = vec![
            Header::from_bytes(&b"Authorization"[..], &b"Bearer secret"[..])
                .expect("authorization"),
            Header::from_bytes(&b"X-Api-Key"[..], &b"api-secret"[..]).expect("api key"),
            Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                .expect("content type"),
        ];
        let raw = br#"{"input":"private prompt","model":"fixture"}"#;
        dumper
            .dump_request(&Method::Post, "/v1/responses", &headers, raw)
            .expect("request dump");

        let text = fs::read_to_string(dump_file_with_suffix(&dump_dir, "-request.json"))
            .expect("read dump");
        assert!(!text.contains("private prompt"));
        assert!(!text.contains("api-secret"));
        let value: serde_json::Value = serde_json::from_str(&text).expect("parse dump");
        assert_eq!(value["body"]["sha256"], digest_bytes(raw));
        assert_eq!(value["body"]["byte_length"].as_u64(), Some(raw.len() as u64));
        assert_eq!(value["body"]["complete"], true);
        let header_values = value["headers"]
            .as_array()
            .expect("header array")
            .iter()
            .map(|header| {
                (
                    header["name"].as_str().expect("header name"),
                    header["value"].as_str().expect("header value"),
                )
            })
            .collect::<Vec<_>>();
        assert!(header_values.contains(&("Authorization", REDACTED_HEADER_VALUE)));
        assert!(header_values.contains(&("X-Api-Key", REDACTED_HEADER_VALUE)));
        fs::remove_dir_all(dump_dir).expect("remove dump dir");
    }

    #[test]
    fn response_dump_streams_original_body_but_persists_only_digest() {
        let dump_dir = test_dump_dir();
        let dumper = ExchangeDumper::new(dump_dir.clone()).expect("dumper");
        let exchange = dumper
            .dump_request(&Method::Post, "/v1/responses", &[], b"{}")
            .expect("request dump");
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer secret"));
        let raw = b"data: private model output\n\n";
        let mut received = Vec::new();
        exchange
            .tee_response_body(200, &headers, std::io::Cursor::new(raw.to_vec()))
            .read_to_end(&mut received)
            .expect("read response");
        assert_eq!(received, raw);

        let text = fs::read_to_string(dump_file_with_suffix(&dump_dir, "-response.json"))
            .expect("read dump");
        assert!(!text.contains("private model output"));
        let value: serde_json::Value = serde_json::from_str(&text).expect("parse dump");
        assert_eq!(value["body"]["sha256"], digest_bytes(raw));
        assert_eq!(value["body"]["byte_length"].as_u64(), Some(raw.len() as u64));
        assert_eq!(value["body"]["complete"], true);
        let authorization = value["headers"]
            .as_array()
            .expect("header array")
            .iter()
            .find(|header| header["name"] == "authorization")
            .expect("authorization header");
        assert_eq!(authorization["value"], REDACTED_HEADER_VALUE);
        fs::remove_dir_all(dump_dir).expect("remove dump dir");
    }

    #[test]
    fn early_drop_records_incomplete_partial_digest() {
        let dump_dir = test_dump_dir();
        let dumper = ExchangeDumper::new(dump_dir.clone()).expect("dumper");
        let exchange = dumper
            .dump_request(&Method::Post, "/v1/responses", &[], b"{}")
            .expect("request dump");
        let mut body = exchange.tee_response_body(
            200,
            &HeaderMap::new(),
            Cursor::new(b"partial-secret".to_vec()),
        );
        let mut first = [0u8; 7];
        body.read_exact(&mut first).expect("partial read");
        drop(body);

        let text = fs::read_to_string(dump_file_with_suffix(&dump_dir, "-response.json"))
            .expect("read dump");
        assert!(!text.contains("partial"));
        let value: serde_json::Value = serde_json::from_str(&text).expect("parse dump");
        assert_eq!(value["body"]["sha256"], digest_bytes(&first));
        assert_eq!(value["body"]["byte_length"].as_u64(), Some(first.len() as u64));
        assert_eq!(value["body"]["complete"], false);
        fs::remove_dir_all(dump_dir).expect("remove dump dir");
    }

    #[cfg(unix)]
    #[test]
    fn dump_directory_and_files_are_owner_only() {
        use std::os::unix::fs::PermissionsExt as _;
        let dump_dir = test_dump_dir();
        let dumper = ExchangeDumper::new(dump_dir.clone()).expect("dumper");
        dumper
            .dump_request(&Method::Post, "/v1/responses", &[], b"{}")
            .expect("request dump");
        let directory_mode = fs::metadata(&dump_dir)
            .expect("directory metadata")
            .permissions()
            .mode()
            & 0o777;
        let file_mode = fs::metadata(dump_file_with_suffix(&dump_dir, "-request.json"))
            .expect("file metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(directory_mode, 0o700);
        assert_eq!(file_mode, 0o600);
        fs::remove_dir_all(dump_dir).expect("remove dump dir");
    }

    fn test_dump_dir() -> PathBuf {
        let id = NEXT_TEST_DIR.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "codex-responses-api-proxy-inf0c-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("create test directory");
        directory
    }

    fn dump_file_with_suffix(directory: &Path, suffix: &str) -> PathBuf {
        let mut matches = fs::read_dir(directory)
            .expect("read directory")
            .map(|entry| entry.expect("entry").path())
            .filter(|path| path.to_string_lossy().ends_with(suffix))
            .collect::<Vec<_>>();
        matches.sort();
        assert_eq!(matches.len(), 1);
        matches.pop().expect("single dump")
    }
}

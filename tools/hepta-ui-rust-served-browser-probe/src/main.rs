use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::time::Duration;

use hepta_core::control_ui::{
    CONTROL_UI_HEPTA_AGENT_LOGO_PNG, CONTROL_UI_INDEX_HTML, CONTROL_UI_JS,
    CONTROL_UI_STYLES_CSS,
};

const MAX_REQUEST_BYTES: usize = 32 * 1024;
const API_BODY: &[u8] = br#"{"status":"ready","source":"hepta-core-rust-served","fixture":false,"data":{}}"#;

fn main() -> io::Result<()> {
    let port_file = parse_port_file()?;
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let address = listener.local_addr()?;
    let ready = format!(
        "{{\"schema\":\"hepta.ui.v4.rust-served-browser-server.v1\",\"host\":\"127.0.0.1\",\"port\":{},\"fixture\":false,\"networkAuthority\":false}}\n",
        address.port()
    );
    if let Some(parent) = port_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&port_file, ready)?;

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                if let Err(error) = handle(stream) {
                    eprintln!("hepta-ui-rust-served-browser-probe: {error}");
                }
            }
            Err(error) => eprintln!("hepta-ui-rust-served-browser-probe accept: {error}"),
        }
    }
    Ok(())
}

fn parse_port_file() -> io::Result<PathBuf> {
    let mut args = env::args_os();
    let _program = args.next();
    match (args.next(), args.next(), args.next()) {
        (Some(flag), Some(value), None) if flag == "--port-file" => Ok(PathBuf::from(value)),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: hepta-ui-rust-served-browser-probe --port-file <path>",
        )),
    }
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    let mut buffer = vec![0_u8; MAX_REQUEST_BYTES];
    let read = stream.read(&mut buffer)?;
    if read == 0 {
        return Ok(());
    }
    let request = String::from_utf8_lossy(&buffer[..read]);
    let first_line = request.lines().next().unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let raw_target = parts.next().unwrap_or_default();
    if !matches!(method, "GET" | "HEAD") {
        return respond(
            &mut stream,
            method,
            405,
            "application/json; charset=utf-8",
            br#"{"status":"denied","reason":"get_head_only"}"#,
        );
    }

    let target = raw_target
        .split('?')
        .next()
        .unwrap_or("/")
        .split('#')
        .next()
        .unwrap_or("/");
    match target {
        "/" | "/index.html" => respond(
            &mut stream,
            method,
            200,
            "text/html; charset=utf-8",
            CONTROL_UI_INDEX_HTML.as_bytes(),
        ),
        "/styles.css" => respond(
            &mut stream,
            method,
            200,
            "text/css; charset=utf-8",
            CONTROL_UI_STYLES_CSS.as_bytes(),
        ),
        "/control-ui.js" => respond(
            &mut stream,
            method,
            200,
            "text/javascript; charset=utf-8",
            CONTROL_UI_JS,
        ),
        "/assets/hepta-agent-logo.png" => respond(
            &mut stream,
            method,
            200,
            "image/png",
            CONTROL_UI_HEPTA_AGENT_LOGO_PNG,
        ),
        value if value.starts_with("/api/") => respond(
            &mut stream,
            method,
            200,
            "application/json; charset=utf-8",
            API_BODY,
        ),
        _ => respond(
            &mut stream,
            method,
            404,
            "application/json; charset=utf-8",
            br#"{"status":"missing"}"#,
        ),
    }
}

fn respond(
    stream: &mut TcpStream,
    method: &str,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; object-src 'none'; form-action 'none'; base-uri 'none'\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    if method != "HEAD" {
        stream.write_all(body)?;
    }
    stream.flush()
}

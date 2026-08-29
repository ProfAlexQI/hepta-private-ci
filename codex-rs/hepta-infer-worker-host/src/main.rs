use std::io;
use std::time::Duration;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_worker_host::MAX_PRIVATE_WORKER_FRAME_BYTES;
use codex_hepta_infer_worker_host::WorkerFrame;
use codex_hepta_infer_worker_host::read_frame;
use codex_hepta_infer_worker_host::write_frame;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mode = fixture_mode()?;
    let mut input = tokio::io::stdin();
    let mut output = tokio::io::stdout();
    let (backend_generation, session_digest) = match read_frame(
        &mut input,
        MAX_PRIVATE_WORKER_FRAME_BYTES,
    )
    .await
    .map_err(protocol_to_io)?
    {
        WorkerFrame::Hello {
            backend_generation,
            session_digest,
        } => (backend_generation, session_digest),
        _ => return Err(io::Error::other("INF_WORKER_HOST_HELLO_REQUIRED")),
    };
    write_frame(
        &mut output,
        &WorkerFrame::Ready {
            backend_generation,
            session_digest: session_digest.clone(),
        },
        MAX_PRIVATE_WORKER_FRAME_BYTES,
    )
    .await
    .map_err(protocol_to_io)?;

    loop {
        let frame = read_frame(&mut input, MAX_PRIVATE_WORKER_FRAME_BYTES)
            .await
            .map_err(protocol_to_io)?;
        match frame {
            WorkerFrame::Submit {
                request_id,
                request_generation,
                backend_generation: request_backend_generation,
                sequence,
                grant_digest: _,
                prompt_digest: _,
                output_token_limit: _,
            } => match mode.as_str() {
                "success" => {
                    let token_digest = fixed_digest('d')?;
                    write_frame(
                        &mut output,
                        &WorkerFrame::Token {
                            request_id: request_id.clone(),
                            request_generation,
                            backend_generation: request_backend_generation,
                            sequence: sequence
                                .checked_add(1)
                                .ok_or_else(|| io::Error::other("INF_WORKER_SEQUENCE_OVERFLOW"))?,
                            token_digest: token_digest.clone(),
                            token_bytes: 7,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                    write_frame(
                        &mut output,
                        &WorkerFrame::Complete {
                            request_id,
                            request_generation,
                            backend_generation: request_backend_generation,
                            sequence: sequence
                                .checked_add(2)
                                .ok_or_else(|| io::Error::other("INF_WORKER_SEQUENCE_OVERFLOW"))?,
                            result_digest: token_digest,
                            output_tokens: 1,
                            fixture: true,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                "stale" => {
                    write_frame(
                        &mut output,
                        &WorkerFrame::Token {
                            request_id,
                            request_generation,
                            backend_generation: request_backend_generation
                                .checked_add(1)
                                .ok_or_else(|| io::Error::other("INF_WORKER_GENERATION_OVERFLOW"))?,
                            sequence: sequence
                                .checked_add(1)
                                .ok_or_else(|| io::Error::other("INF_WORKER_SEQUENCE_OVERFLOW"))?,
                            token_digest: fixed_digest('e')?,
                            token_bytes: 1,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                "oom" => {
                    write_frame(
                        &mut output,
                        &WorkerFrame::Failure {
                            request_id,
                            request_generation,
                            backend_generation: request_backend_generation,
                            code: "INF_WORKER_OOM".to_owned(),
                            forced_worker_termination: false,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                "crash" => std::process::exit(33),
                "hang" => tokio::time::sleep(Duration::from_secs(60)).await,
                "cancel-ack" => {
                    write_frame(
                        &mut output,
                        &WorkerFrame::Failure {
                            request_id,
                            request_generation,
                            backend_generation: request_backend_generation,
                            code: "INF_WORKER_FIXTURE_MODE".to_owned(),
                            forced_worker_termination: false,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                _ => return Err(io::Error::other("INF_WORKER_FIXTURE_MODE_INVALID")),
            },
            WorkerFrame::Cancel {
                request_id,
                request_generation,
                cancel_generation,
                backend_generation: request_backend_generation,
            } => match mode.as_str() {
                "cancel-ack" | "success" => {
                    write_frame(
                        &mut output,
                        &WorkerFrame::CancelAck {
                            request_id,
                            request_generation,
                            cancel_generation,
                            backend_generation: request_backend_generation,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                "hang" => tokio::time::sleep(Duration::from_secs(60)).await,
                "crash" => std::process::exit(34),
                "stale" => {
                    write_frame(
                        &mut output,
                        &WorkerFrame::CancelAck {
                            request_id,
                            request_generation,
                            cancel_generation,
                            backend_generation: request_backend_generation
                                .checked_add(1)
                                .ok_or_else(|| io::Error::other("INF_WORKER_GENERATION_OVERFLOW"))?,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                "oom" => {
                    write_frame(
                        &mut output,
                        &WorkerFrame::Failure {
                            request_id,
                            request_generation,
                            backend_generation: request_backend_generation,
                            code: "INF_WORKER_OOM".to_owned(),
                            forced_worker_termination: false,
                        },
                        MAX_PRIVATE_WORKER_FRAME_BYTES,
                    )
                    .await
                    .map_err(protocol_to_io)?;
                }
                _ => return Err(io::Error::other("INF_WORKER_FIXTURE_MODE_INVALID")),
            },
            WorkerFrame::Shutdown => break,
            _ => return Err(io::Error::other("INF_WORKER_HOST_FRAME_NOT_ALLOWED")),
        }
    }
    if backend_generation == 0 || session_digest.as_str().is_empty() {
        return Err(io::Error::other("INF_WORKER_HOST_SESSION_INVALID"));
    }
    Ok(())
}

fn fixture_mode() -> io::Result<String> {
    let mut arguments = std::env::args().skip(1);
    if arguments.next().as_deref() != Some("--fixture-mode") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "INF_WORKER_FIXTURE_MODE_MISSING",
        ));
    }
    let mode = arguments.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "INF_WORKER_FIXTURE_MODE_MISSING",
        )
    })?;
    if arguments.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "INF_WORKER_FIXTURE_ARGUMENTS_INVALID",
        ));
    }
    Ok(mode)
}

fn fixed_digest(fill: char) -> io::Result<Digest> {
    Digest::parse(&format!("sha256:{}", fill.to_string().repeat(64)))
        .map_err(|_| io::Error::other("INF_WORKER_FIXTURE_DIGEST_INVALID"))
}

fn protocol_to_io(error: codex_hepta_infer_worker_host::ProtocolError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

impl OllamaClient {
    /// Start an explicit model pull and emit bounded streaming events.
    ///
    /// Product readiness never calls this method implicitly. Every transport,
    /// UTF-8, JSON, server, and frame-size failure becomes a terminal
    /// [`PullEvent::Error`].
    pub async fn pull_model_stream(
        &self,
        model: &str,
    ) -> io::Result<BoxStream<'static, PullEvent>> {
        validate_model_identifier(model)?;
        let response = tokio::time::timeout(
            OLLAMA_REQUEST_TIMEOUT,
            self.client
                .post(self.endpoint("/api/pull"))
                .json(&serde_json::json!({"model": model, "stream": true}))
                .send(),
        )
        .await
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::TimedOut,
                "OLLAMA_REQUEST_TIMEOUT operation=pull",
            )
        })?
        .map_err(|error| request_error("pull", error))?;
        if !response.status().is_success() {
            return Err(status_error("pull", response.status().as_u16()));
        }

        let mut stream = response.bytes_stream();
        let mut buffer = LineBuffer::default();
        let events = async_stream::stream! {
            loop {
                let next_chunk = match tokio::time::timeout(
                    OLLAMA_PULL_IDLE_TIMEOUT,
                    stream.next(),
                )
                .await
                {
                    Ok(next_chunk) => next_chunk,
                    Err(_) => {
                        yield PullEvent::Error(
                            "OLLAMA_PULL_IDLE_TIMEOUT".to_string(),
                        );
                        return;
                    }
                };
                let Some(chunk) = next_chunk else {
                    break;
                };
                let bytes = match chunk {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        yield PullEvent::Error(format!(
                            "OLLAMA_PULL_TRANSPORT_ERROR: {}",
                            sanitize_remote_message(&error.to_string())
                        ));
                        return;
                    }
                };
                let Some(next_buffer_len) = buffer.len().checked_add(bytes.len()) else {
                    yield PullEvent::Error(format!(
                        "OLLAMA_PULL_FRAME_TOO_LARGE: maximum={MAX_PULL_FRAME_BYTES}"
                    ));
                    return;
                };
                if next_buffer_len > MAX_PULL_FRAME_BYTES {
                    yield PullEvent::Error(format!(
                        "OLLAMA_PULL_FRAME_TOO_LARGE: maximum={MAX_PULL_FRAME_BYTES}"
                    ));
                    return;
                }
                buffer.extend_from_slice(&bytes);

                while let Some(line) = buffer.take_line() {
                    match decode_pull_frame(&line) {
                        Ok(decoded) => {
                            let terminal = decoded
                                .iter()
                                .any(|event| matches!(event, PullEvent::Success));
                            for event in decoded {
                                yield event;
                            }
                            if terminal {
                                return;
                            }
                        }
                        Err(error) => {
                            yield PullEvent::Error(error);
                            return;
                        }
                    }
                }

            }

            if let Some(frame) = buffer.take_remaining() {
                match decode_pull_frame(&frame) {
                    Ok(decoded) => {
                        let terminal = decoded
                            .iter()
                            .any(|event| matches!(event, PullEvent::Success));
                        for event in decoded {
                            yield event;
                        }
                        if terminal {
                            return;
                        }
                    }
                    Err(error) => {
                        yield PullEvent::Error(error);
                        return;
                    }
                }
            }
            yield PullEvent::Error("OLLAMA_PULL_UNEXPECTED_EOF".to_string());
        };

        Ok(Box::pin(events))
    }

    /// Explicit operator helper to pull a model and drive a progress reporter.
    pub async fn pull_with_reporter(
        &self,
        model: &str,
        reporter: &mut dyn PullProgressReporter,
    ) -> io::Result<()> {
        validate_model_identifier(model)?;
        reporter.on_event(&PullEvent::Status(format!("Pulling model {model}...")))?;
        let mut stream = self.pull_model_stream(model).await?;
        while let Some(event) = stream.next().await {
            reporter.on_event(&event)?;
            match event {
                PullEvent::Success => return Ok(()),
                PullEvent::Error(error) => {
                    return Err(io::Error::other(format!("Pull failed: {error}")));
                }
                PullEvent::ChunkProgress { .. } | PullEvent::Status(_) => {}
            }
        }
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "OLLAMA_PULL_UNEXPECTED_EOF: stream ended without success",
        ))
    }

    fn endpoint(&self, endpoint: &str) -> String {
        format!("{}{}", self.host_root.trim_end_matches('/'), endpoint)
    }

    #[cfg(test)]
    fn from_host_root(host_root: impl Into<String>) -> Self {
        let client = HttpClientBuilder::new()
            .without_redirects()
            .without_request_logging()
            .connect_timeout(OLLAMA_CONNECTION_TIMEOUT)
            .build_direct()
            .expect("direct test client");
        Self {
            client,
            host_root: host_root.into(),
            uses_openai_compat: false,
        }
    }
}

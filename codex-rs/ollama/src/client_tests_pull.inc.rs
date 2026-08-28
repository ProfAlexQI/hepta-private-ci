#[tokio::test]
async fn pull_stream_emits_one_success_for_trailing_frame_without_newline() {
    if network_disabled() {
        return;
    }
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/pull"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_raw(r#"{"status":"success"}"#, "application/x-ndjson"),
        )
        .mount(&server)
        .await;

    let events = OllamaClient::from_host_root(server.uri())
        .pull_model_stream("fixture")
        .await
        .expect("start stream")
        .collect::<Vec<_>>()
        .await;
    assert_matches!(
        events.as_slice(),
        [PullEvent::Status(status), PullEvent::Success] if status == "success"
    );
}

#[tokio::test]
async fn pull_stream_turns_invalid_json_into_terminal_error() {
    if network_disabled() {
        return;
    }
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/pull"))
        .respond_with(
            wiremock::ResponseTemplate::new(200).set_body_raw("not-json\n", "application/x-ndjson"),
        )
        .mount(&server)
        .await;

    let events = OllamaClient::from_host_root(server.uri())
        .pull_model_stream("fixture")
        .await
        .expect("start stream")
        .collect::<Vec<_>>()
        .await;
    assert_matches!(
        events.as_slice(),
        [PullEvent::Error(error)] if error == "OLLAMA_PULL_INVALID_JSON"
    );
}

#[tokio::test]
async fn pull_stream_emits_terminal_error_on_unexpected_eof() {
    if network_disabled() {
        return;
    }
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/pull"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(
            "{\"status\":\"pulling manifest\"}\n",
            "application/x-ndjson",
        ))
        .mount(&server)
        .await;

    let events = OllamaClient::from_host_root(server.uri())
        .pull_model_stream("fixture")
        .await
        .expect("start stream")
        .collect::<Vec<_>>()
        .await;
    assert_matches!(
        events.as_slice(),
        [PullEvent::Status(status), PullEvent::Error(error)]
            if status == "pulling manifest" && error == "OLLAMA_PULL_UNEXPECTED_EOF"
    );
}

#[tokio::test]
async fn pull_stream_rejects_oversized_frame() {
    if network_disabled() {
        return;
    }
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/pull"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_raw("x".repeat(MAX_PULL_FRAME_BYTES + 1), "application/x-ndjson"),
        )
        .mount(&server)
        .await;

    let events = OllamaClient::from_host_root(server.uri())
        .pull_model_stream("fixture")
        .await
        .expect("start stream")
        .collect::<Vec<_>>()
        .await;
    assert_matches!(
        events.as_slice(),
        [PullEvent::Error(error)] if error.contains("FRAME_TOO_LARGE")
    );
}

#[test]
fn base_url_requires_loopback_http_without_credentials() {
    assert!(validate_loopback_http_base_url("http://localhost:11434/v1").is_ok());
    assert!(validate_loopback_http_base_url("http://127.0.0.1:11434").is_ok());
    assert!(validate_loopback_http_base_url("http://[::1]:11434/v1").is_ok());
    assert!(validate_loopback_http_base_url("https://127.0.0.1:11434/v1").is_err());
    assert!(validate_loopback_http_base_url("http://example.com:11434/v1").is_err());
    assert!(validate_loopback_http_base_url("http://user@127.0.0.1:11434/v1").is_err());
    assert!(validate_loopback_http_base_url("http://127.0.0.1:11434/other").is_err());
}

#[tokio::test]
async fn fetch_models_rejects_padded_identifier() {
    if network_disabled() {
        return;
    }
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/api/tags"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"models": [{"name": " padded"}]})),
        )
        .mount(&server)
        .await;
    let error = OllamaClient::from_host_root(server.uri())
        .fetch_models()
        .await
        .expect_err("padded identifier must fail");
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[tokio::test]
async fn probe_supports_native_and_openai_compatible_endpoints() {
    if network_disabled() {
        return;
    }
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/api/tags"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&server)
        .await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/v1/models"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&server)
        .await;

    OllamaClient::from_host_root(server.uri())
        .probe_server()
        .await
        .expect("native probe");
    OllamaClient::try_from_provider_with_base_url(&format!("{}/v1", server.uri()))
        .await
        .expect("OpenAI-compatible probe");
}

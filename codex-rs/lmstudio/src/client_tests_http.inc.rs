    fn network_disabled() -> bool {
        std::env::var(codex_core::spawn::CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR).is_ok()
    }

    fn client_from_host_root(host_root: impl Into<String>) -> LMStudioClient {
        let client = HttpClientBuilder::new()
            .without_redirects()
            .without_request_logging()
            .connect_timeout(LMSTUDIO_CONNECTION_TIMEOUT)
            .build_direct()
            .expect("direct test client");
        LMStudioClient {
            client,
            base_url: host_root.into(),
        }
    }

    #[tokio::test]
    async fn fetch_models_happy_path() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/models"))
            .respond_with(
                wiremock::ResponseTemplate::new(200).set_body_json(
                    serde_json::json!({"data": [{"id": "openai/gpt-oss-20b"}]}),
                ),
            )
            .mount(&server)
            .await;
        assert_eq!(
            client_from_host_root(server.uri())
                .fetch_models()
                .await
                .expect("models"),
            vec!["openai/gpt-oss-20b"]
        );
    }

    #[tokio::test]
    async fn fetch_models_rejects_non_success_and_malformed_entries() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/models"))
            .respond_with(wiremock::ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let error = client_from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("500 must fail");
        assert!(error.to_string().contains("LMSTUDIO_MODELS_HTTP_STATUS"));
    }

    #[tokio::test]
    async fn fetch_models_rejects_oversized_control_response() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/models"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(
                "x".repeat(MAX_CONTROL_RESPONSE_BYTES + 1),
                "application/json",
            ))
            .mount(&server)
            .await;
        let error = client_from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("oversized body must fail");
        assert!(error.to_string().contains("CONTROL_RESPONSE_TOO_LARGE"));
    }

    #[tokio::test]
    async fn fetch_models_does_not_follow_redirects() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/models"))
            .respond_with(
                wiremock::ResponseTemplate::new(307)
                    .insert_header("Location", "http://example.com/models"),
            )
            .mount(&server)
            .await;
        let error = client_from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("redirect must fail closed");
        assert!(error.to_string().contains("status=307"));
    }


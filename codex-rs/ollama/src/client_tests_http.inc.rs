    fn network_disabled() -> bool {
        std::env::var(codex_core::spawn::CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR).is_ok()
    }

    #[tokio::test]
    async fn fetch_models_native_happy_path() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"models": [{"name": "llama3.2:3b"}]}),
            ))
            .mount(&server)
            .await;

        let client = OllamaClient::from_host_root(server.uri());
        assert_eq!(
            client.fetch_models().await.expect("models"),
            vec!["llama3.2:3b"]
        );
    }

    #[tokio::test]
    async fn fetch_models_non_success_is_not_empty_list() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(wiremock::ResponseTemplate::new(503))
            .mount(&server)
            .await;

        let error = OllamaClient::from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("503 must fail");
        assert!(error.to_string().contains("OLLAMA_HTTP_STATUS"));
        assert!(error.to_string().contains("status=503"));
    }

    #[tokio::test]
    async fn fetch_models_does_not_follow_redirects() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(
                wiremock::ResponseTemplate::new(307)
                    .insert_header("Location", "http://example.com/models"),
            )
            .mount(&server)
            .await;
        let error = OllamaClient::from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("redirect must fail closed");
        assert!(error.to_string().contains("status=307"));
    }

    #[tokio::test]
    async fn fetch_models_rejects_oversized_control_response() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(
                "x".repeat(MAX_CONTROL_RESPONSE_BYTES + 1),
                "application/json",
            ))
            .mount(&server)
            .await;
        let error = OllamaClient::from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("oversized body must fail");
        assert!(error.to_string().contains("CONTROL_RESPONSE_TOO_LARGE"));
    }

    #[tokio::test]
    async fn fetch_models_rejects_malformed_entry() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"models": [{}]})),
            )
            .mount(&server)
            .await;

        let error = OllamaClient::from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("malformed entry must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[tokio::test]
    async fn fetch_version_rejects_non_success_and_bad_semver() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/version"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"version": "not-semver"}),
            ))
            .mount(&server)
            .await;

        let error = OllamaClient::from_host_root(server.uri())
            .fetch_version()
            .await
            .expect_err("bad version must fail");
        assert!(error.to_string().contains("invalid semantic version"));
    }


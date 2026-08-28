    #[test]
    fn identical_paths_have_identical_receipts_and_domain_separated_operations() {
        let first = happy_path_events();
        let second = happy_path_events();
        assert_eq!(first, second);

        let correct = IntelligenceMutationBinding::derive(
            "agent:test",
            "workspace:test",
            "correct",
            "turn:7:memory:aurora",
            "lease:1",
            7,
            Some(3),
            11,
        )
        .expect("binding");
        assert_ne!(binding().operation_id, correct.operation_id);
        assert_ne!(binding().operation_id, binding().causal_root_sha256);
    }

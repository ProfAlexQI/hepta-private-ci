fn run_drill() -> Result<String, String> {
    let windows = WindowPair::create()?;
    let identity = HeptaWindowsQualificationWindowIdentity {
        root_window_index: 1,
        root_window_generation: 1,
        root_hwnd: windows.root as u64,
        transient_window_index: 2,
        transient_window_generation: 1,
        transient_hwnd: windows.transient as u64,
    };
    if !identity.is_valid() {
        return Err("window identity invalid".to_string());
    }
    let request = request(identity)?;
    let backend = DwmBackend {
        root: windows.root,
        transient: windows.transient,
    };

    let mut explicit = HeptaWindowsQualificationHost::new(backend);
    let active = explicit
        .begin_qualification(&request)
        .map_err(|error| format!("explicit activation failed: {error:?}"))?;
    let rollback = explicit
        .rollback_to_solid()
        .map_err(|error| format!("explicit rollback failed: {error:?}"))?;
    let evidence = explicit
        .qualification_evidence()
        .map_err(|error| format!("qualification evidence failed: {error:?}"))?;
    if !active.root_mica_exact
        || !active.transient_acrylic_exact
        || !rollback.root_none_exact
        || !rollback.transient_none_exact
        || !evidence.qualified_unbound
    {
        return Err("explicit material transaction was not exact".to_string());
    }

    let mut high_contrast = HeptaWindowsQualificationHost::new(backend);
    high_contrast
        .begin_qualification(&request)
        .map_err(|error| format!("high-contrast setup failed: {error:?}"))?;
    let high_contrast_receipt = high_contrast
        .enforce_preferences(HeptaWindowsQualificationPreferences {
            transparency_allowed: true,
            high_contrast: true,
        })
        .map_err(|error| format!("high-contrast rollback failed: {error:?}"))?;

    let mut transparency_disabled = HeptaWindowsQualificationHost::new(backend);
    transparency_disabled
        .begin_qualification(&request)
        .map_err(|error| format!("transparency setup failed: {error:?}"))?;
    let transparency_receipt = transparency_disabled
        .enforce_preferences(HeptaWindowsQualificationPreferences {
            transparency_allowed: false,
            high_contrast: false,
        })
        .map_err(|error| format!("transparency rollback failed: {error:?}"))?;

    let mut suspended = HeptaWindowsQualificationHost::new(backend);
    suspended
        .begin_qualification(&request)
        .map_err(|error| format!("suspend setup failed: {error:?}"))?;
    let suspend_receipt = suspended
        .suspend()
        .map_err(|error| format!("suspend rollback failed: {error:?}"))?;

    let mut shutdown = HeptaWindowsQualificationHost::new(backend);
    shutdown
        .begin_qualification(&request)
        .map_err(|error| format!("shutdown setup failed: {error:?}"))?;
    let shutdown_receipt = shutdown
        .shutdown()
        .map_err(|error| format!("shutdown rollback failed: {error:?}"))?;

    for receipt in [
        high_contrast_receipt,
        transparency_receipt,
        suspend_receipt,
        shutdown_receipt,
    ] {
        if receipt.rollback_required
            || !receipt.root_none_exact
            || !receipt.transient_none_exact
            || !receipt.remains_non_product()
            || !receipt.grants_no_authority()
        {
            return Err("a governed fallback path did not finish safely unbound".to_string());
        }
    }
    if shutdown.phase() != HeptaWindowsQualificationPhase::Shutdown
        || shutdown.requires_rollback()
        || shutdown.active_identity().is_some()
    {
        return Err("final shutdown state is not safely unbound".to_string());
    }

    let candidate_commit = required("HEPTA_CANDIDATE_COMMIT")?;
    let candidate_tree = required("HEPTA_CANDIDATE_TREE")?;
    let review_digest = required("HEPTA_REVIEW_BINDING_DIGEST")?;
    Ok(format!(
        concat!(
            "{{\n",
            "  \"schema\": \"hepta.ui.v4.windows-product-host-device-drill.v1\",\n",
            "  \"status\": \"PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_DEVICE_DRILL\",\n",
            "  \"candidate\": {{\"commit\": \"{}\", \"tree\": \"{}\"}},\n",
            "  \"reviewBindingDigest\": \"{}\",\n",
            "  \"feature\": {{\"name\": \"hepta_ui_windows_system_material_v4\", \"enabled\": true}},\n",
            "  \"device\": {{\"physical\": true, \"os\": \"Windows\", \"architecture\": \"X64\", \"runnerLabels\": [\"self-hosted\", \"Windows\", \"X64\", \"hepta-ui-dwm\"]}},\n",
            "  \"identity\": {{\n",
            "    \"root\": {{\"index\": 1, \"generation\": 1, \"nativeHandle\": \"{}\"}},\n",
            "    \"transient\": {{\"index\": 2, \"generation\": 1, \"nativeHandle\": \"{}\"}}\n",
            "  }},\n",
            "  \"checks\": {{\"activationObserved\": true, \"rootMicaExact\": true, \"transientAcrylicExact\": true, \"explicitRollback\": true, \"rootNoneExact\": true, \"transientNoneExact\": true, \"highContrastFallback\": true, \"transparencyDisabledFallback\": true, \"suspendRollback\": true, \"shutdownRollback\": true, \"rollbackDrillValidated\": true, \"physicalDeviceValidated\": true, \"finalState\": \"Unbound\"}},\n",
            "  \"qualification\": {{\"isolatedCandidate\": true, \"physicalDeviceValidated\": true, \"rollbackDrillValidated\": true, \"productCargoFeatureDeclared\": false, \"productModuleRegistered\": false, \"productLifecycleWired\": false, \"automaticBindingAllowed\": false, \"implementationApproved\": false, \"productHostMayBind\": false, \"productBound\": false, \"transientSystemMaterialBound\": false, \"completeProfileBound\": false, \"systemMaterialBound\": false, \"nativeProductRuntime\": false, \"deviceValidation\": false}},\n",
            "  \"authority\": {{\"network\": false, \"mutation\": false, \"effect\": false, \"liveAdapter\": false, \"production\": false, \"operatorAcceptance\": false, \"promotion\": false, \"release\": false}}\n",
            "}}\n"
        ),
        candidate_commit,
        candidate_tree,
        review_digest,
        identity.root_hwnd,
        identity.transient_hwnd,
    ))
}

fn write_receipt(payload: &str) -> Result<(), String> {
    let path = PathBuf::from(required("HEPTA_DEVICE_DRILL_RECEIPT_PATH")?);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, payload).map_err(|error| error.to_string())?;
    fs::rename(temporary, path).map_err(|error| error.to_string())
}

pub fn main() {
    match run_drill() {
        Ok(payload) => {
            if let Err(error) = write_receipt(&payload) {
                eprintln!("device receipt write failed: {error}");
                std::process::exit(1);
            }
            println!("{payload}");
        }
        Err(error) => {
            let bounded: String = error.chars().take(2048).collect();
            let candidate_commit = env::var("HEPTA_CANDIDATE_COMMIT").unwrap_or_default();
            let candidate_tree = env::var("HEPTA_CANDIDATE_TREE").unwrap_or_default();
            let payload = format!(
                "{{\n  \"schema\": \"hepta.ui.v4.windows-product-host-device-drill-failure.v1\",\n  \"status\": \"FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_DEVICE_DRILL\",\n  \"candidate\": {{\"commit\": \"{}\", \"tree\": \"{}\"}},\n  \"failures\": [\"{}\"],\n  \"authority\": {{\"network\": false, \"mutation\": false, \"effect\": false, \"liveAdapter\": false, \"production\": false, \"operatorAcceptance\": false, \"promotion\": false, \"release\": false}}\n}}\n",
                json_escape(&candidate_commit),
                json_escape(&candidate_tree),
                json_escape(&bounded),
            );
            let _ = write_receipt(&payload);
            eprintln!("{payload}");
            std::process::exit(1);
        }
    }
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

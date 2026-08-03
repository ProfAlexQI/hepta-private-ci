def hepta_android_emulator_receipt_v3_ready($head; $tree; $fingerprint; $manifest; $manifest_sha):
  def sha: type == "string" and test("^[0-9a-f]{64}$");
  def uuid: type == "string" and test("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$");
  def absolute_evidence_path:
    type == "string"
    and startswith("/")
    and (contains("/../") | not)
    and (endswith("/..") | not);
  .schema_version == 3
  and .kind == "hepta-native-android-emulator-smoke-receipt"
  and .producer == "scripts/hepta-native-android-emulator-smoke.sh"
  and .status == "ready"
  and .ready == true
  and .scope == "unauthenticated_android_login_surface_on_arm64_emulator"
  and .source_binding.head == $head
  and .source_binding.head_tree == $tree
  and .source_binding.source_fingerprint == $fingerprint
  and .source_binding.worktree_clean == true
  and .source_binding.repository_worktree_clean == true
  and (.artifact.path | absolute_evidence_path)
  and (.artifact.sha256 | sha)
  and .artifact.stale_artifact_accepted == false
  and .artifact.full_head_embedded == true
  and .artifact.artifact_source_bound == true
  and (.host_toolchain.emulator_binary_sha256 | sha)
  and (.host_toolchain.qemu_binary_sha256 | sha)
  and (.host_toolchain.adb_binary_path | absolute_evidence_path)
  and (.host_toolchain.adb_binary_sha256 | sha)
  and .host_toolchain.ndk.directory_version == "28.2.13676358"
  and .host_toolchain.ndk.release_name == "r28b"
  and .host_toolchain.ndk.host_prebuilt == "darwin-x86_64"
  and (.host_toolchain.ndk.root_path | absolute_evidence_path and endswith("/ndk/28.2.13676358"))
  and (.host_toolchain.ndk.source_properties_path | absolute_evidence_path and endswith("/ndk/28.2.13676358/source.properties"))
  and (.host_toolchain.ndk.source_properties_sha256 | sha)
  and (.host_toolchain.ndk.clang_binary_path | absolute_evidence_path and endswith("/ndk/28.2.13676358/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang"))
  and (.host_toolchain.ndk.clang_binary_sha256 | sha)
  and .host_toolchain.makepad_android_sdk.platform == "android-33-ext4"
  and .host_toolchain.makepad_android_sdk.build_tools_version == "33.0.1"
  and (.host_toolchain.makepad_android_sdk.android_jar_path | absolute_evidence_path and endswith("/platforms/android-33-ext4/android.jar"))
  and (.host_toolchain.makepad_android_sdk.android_jar_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.aapt_path | absolute_evidence_path and endswith("/build-tools/33.0.1/aapt"))
  and (.host_toolchain.makepad_android_sdk.aapt_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.aapt2_path | absolute_evidence_path and endswith("/build-tools/33.0.1/aapt2"))
  and (.host_toolchain.makepad_android_sdk.aapt2_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.d8_jar_path | absolute_evidence_path and endswith("/build-tools/33.0.1/lib/d8.jar"))
  and (.host_toolchain.makepad_android_sdk.d8_jar_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.zipalign_path | absolute_evidence_path and endswith("/build-tools/33.0.1/zipalign"))
  and (.host_toolchain.makepad_android_sdk.zipalign_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.apksigner_jar_path | absolute_evidence_path and endswith("/build-tools/33.0.1/lib/apksigner.jar"))
  and (.host_toolchain.makepad_android_sdk.apksigner_jar_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.java_path | absolute_evidence_path and endswith("/openjdk/bin/java"))
  and (.host_toolchain.makepad_android_sdk.java_sha256 | sha)
  and (.host_toolchain.makepad_android_sdk.javac_path | absolute_evidence_path and endswith("/openjdk/bin/javac"))
  and (.host_toolchain.makepad_android_sdk.javac_sha256 | sha)
  and (.device.adb_serial | type == "string" and test("^emulator-[0-9]+$"))
  and .device.avd_name == .device.qemu_avd_name
  and .device.avd_name == .avd.name
  and (.device.boot_id | uuid)
  and (.runtime.installed_package_path | type == "string" and test("^/data/app/[0-9A-Za-z._~=/+-]+/base\\.apk$"))
  and (.runtime.pid | type == "number" and . == floor and . > 0)
  and (.runtime.process_start_time_ticks | type == "number" and . == floor and . > 0)
  and (.session_probe.path | type == "string" and test("^/data/local/tmp/hepta-native-smoke-[0-9a-f]{24}$"))
  and (.session_probe.nonce | uuid)
  and (.session_probe.sha256 | sha)
  and .session_probe.boot_id == .device.boot_id
  and .session_probe.created_by_producer == true
  and .session_probe.readback_matched == true
  and .session_probe.no_credentials_supplied == true
  and .login_surface_template.manifest_path == $manifest
  and .login_surface_template.manifest_sha256 == $manifest_sha
  and .login_surface_template.all_states_ready == true
  and (. as $receipt | ["portrait", "landscape", "ime"] | all(. as $key |
    ($receipt.visual_inspection[$key].path | absolute_evidence_path)
    and ($receipt.visual_inspection[$key].sha256 | sha)
    and ($receipt.visual_inspection[$key].content_probe.ready == true)
    and ($receipt.visual_inspection[$key].login_template_probe.ready == true)
    and ($receipt.visual_inspection[$key].login_surface_template_ready == true)
  ))
  and (.uiautomator.path | absolute_evidence_path)
  and (.uiautomator.sha256 | sha)
  and .uiautomator.xml_ready == true
  and .uiautomator.semantic_accessibility_ready == false
  and .uiautomator.talkback_ready == false
  and .claims.android_emulator_login_surface_visual_ready == true
  and .claims.android_login_rotation_ready == true
  and .claims.android_login_ime_ready == true
  and .claims.android_rotation_ready == false
  and .claims.android_ime_ready == false
  and .claims.android_accessibility_ready == false
  and .claims.talkback_ready == false
  and .claims.android_real_device_ready == false
  and .claims.android_secure_credential_backend_ready == false
  and .claims.authenticated_matrix_workflow_ready == false
  and .claims.release_signed == false
  and .claims.public_distribution_ready == false
  and .claims.full_product_ready == false
  and .claims.public_ga_ready == false
  and ((.extended_lab // {requested:false}) as $lab |
    if $lab.requested == true then
      $lab.status == "executed_with_product_claim_blockers"
      and $lab.execution_ready == true
      and $lab.ready == false
      and $lab.state_restore_verified == true
      and $lab.modes.rtl.executed == true
      and $lab.modes.rtl.force_rtl_readback == true
      and $lab.modes.rtl.ready == false
      and $lab.modes.font_scale.executed == true
      and $lab.modes.font_scale.setting_readback_ready == true
      and $lab.modes.font_scale.ready == false
      and $lab.modes.rotation_ime.executed == true
      and $lab.modes.rotation_ime.scope == "unauthenticated_login_surface"
      and $lab.modes.rotation_ime.generic_app_wide_ready == false
      and $lab.modes.startup_performance.executed == true
      and $lab.modes.startup_performance.ready == true
      and $lab.modes.low_power.executed == true
      and $lab.modes.low_power.emulator_only == true
      and $lab.modes.low_power.real_low_power_qualification == false
      and $lab.modes.low_power.ready == false
      and ($lab.claims | to_entries | length > 0 and all(.value == false))
      and ($lab.blockers | map(.code) | index("android_real_device_low_power_performance_receipt_missing") != null)
      and ($lab.blockers | map(.code) | index("android_real_device_receipt_missing") != null)
      and ($lab.blockers | map(.code) | index("talkback_receipt_missing") != null)
      and ($lab.forbidden_actions_performed | to_entries | length > 0 and all(.value == false))
    else
      $lab.requested == false
    end)
  and (.hard_boundaries | to_entries | length > 0 and all(.value == false))
  and (.forbidden_actions_performed | to_entries | length > 0 and all(.value == false));

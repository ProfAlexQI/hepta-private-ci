def hepta_android_login_template_v1_ready:
  .schema_version == 1
  and .kind == "hepta-native-android-emulator-login-template-manifest"
  and .status == "ready"
  and .scope == "unauthenticated_android_login_surface_on_hepta_pixel_api_34_arm64"
  and (has("source_receipt") | not)
  and .source_evidence.head == "3e93658fef1bffdd73a9ade5d668f88e4fe1ff3c"
  and .source_evidence.avd == "Hepta_Pixel_API_34_arm64"
  and .source_evidence.receipt.sha256 == "c6ec76341505bcc20e10fd53714c374897f8ec97b07a8f4e4488f463e418c759"
  and .source_evidence.receipt.external_not_committed == true
  and (.source_evidence.receipt | keys | sort) == ["external_not_committed","sha256"]
  and .source_evidence.captured_frames.portrait_sha256 == "2dfaf3948b4a11af4514e2093d77cc0dc929516932dcf9dfacd501fe48adedb1"
  and .source_evidence.captured_frames.landscape_sha256 == "1e354535bffdcdea2d740f0aed189a755731aee3afa0495b6813fbf2106f000b"
  and .source_evidence.captured_frames.ime_sha256 == "48ece820b88dd0478e698cae8c86a6ad2a81d746c25a25b6e0a7e023f1b75636"
  and (.source_evidence.captured_frames | keys | sort) == ["ime_sha256","landscape_sha256","portrait_sha256"]
  and .source_evidence.claim_scope == "visual_template_seed_only"
  and (.source_evidence | keys | sort) == ["avd","captured_frames","claim_scope","head","receipt"]
  and (.templates | keys | sort) == ["ime","landscape","portrait"]
  and .templates.portrait.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/portrait.png"
  and .templates.portrait.sha256 == "19c36f0e2feb33f41d5b0ba3a5fbeff3a0d9e98ccd70123cd33a4e246fe767e6"
  and .templates.landscape.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/landscape.png"
  and .templates.landscape.sha256 == "7fcd567a6b5b4c9fa9caa945bcb1e74cac12e4d1a20f6696f310a1141c11c085"
  and .templates.ime.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/ime.png"
  and .templates.ime.sha256 == "8e10400d49a85b2e2c343af869c992ad0ac07d90e2bacd10f57e998c6f1a1f20"
  and .claim_boundary == "Visual template identity only. No semantic accessibility, authentication, real-device, secure-credential, or release claim.";

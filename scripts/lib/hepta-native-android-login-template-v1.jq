def hepta_android_login_template_v1_ready:
  .schema_version == 1
  and .kind == "hepta-native-android-emulator-login-template-manifest"
  and .status == "ready"
  and .scope == "unauthenticated_android_login_surface_on_hepta_pixel_api_34_arm64"
  and (has("source_receipt") | not)
  and .source_evidence.head == "d4b0f2e15c0bc82fd881a9ea4fa5df6cd3f37ede"
  and .source_evidence.avd == "Hepta_Pixel_API_34_arm64"
  and .source_evidence.receipt.sha256 == "3ad0330ac8d907dbbae92f6dd92081b0d34b03216446689556190d64d1d6b2c8"
  and .source_evidence.receipt.external_not_committed == true
  and (.source_evidence.receipt | keys | sort) == ["external_not_committed","sha256"]
  and .source_evidence.captured_frames.portrait_sha256 == "eb6570732c153e570ae06efb77c8fbaa3fa8fe5131e5d4f782b6ff29fa23bf23"
  and .source_evidence.captured_frames.landscape_sha256 == "adf5674f61fb6526d93b101d3cb6ba5857646ad038bc7489f76ac95c823829dd"
  and .source_evidence.captured_frames.ime_sha256 == "c7013aa153ec36233bad45f20a064d80eca2316c0806469d9f6371fdd749015a"
  and (.source_evidence.captured_frames | keys | sort) == ["ime_sha256","landscape_sha256","portrait_sha256"]
  and .source_evidence.claim_scope == "visual_template_seed_only"
  and (.source_evidence | keys | sort) == ["avd","captured_frames","claim_scope","head","receipt"]
  and (.templates | keys | sort) == ["ime","landscape","portrait"]
  and .templates.portrait.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/portrait.png"
  and .templates.portrait.sha256 == "27b37d8efe07dbf34db407498db3fbbc2a04e1ba9e0e29f92f225c56a689b8b0"
  and .templates.landscape.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/landscape.png"
  and .templates.landscape.sha256 == "c6f1173ee5af04474615a1956c343d85fbd292baa1a906be41693bdee9293220"
  and .templates.ime.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/ime.png"
  and .templates.ime.sha256 == "3e987c658cd4559c2b5ba969961a0265781f130400f12d2f0a4758d8d9250a05"
  and .claim_boundary == "Visual template identity only. No semantic accessibility, authentication, real-device, secure-credential, or release claim.";

def hepta_android_login_template_v1_ready:
  .schema_version == 1
  and .kind == "hepta-native-android-emulator-login-template-manifest"
  and .status == "ready"
  and .scope == "unauthenticated_android_login_surface_on_hepta_pixel_api_34_arm64"
  and (has("source_receipt") | not)
  and .source_evidence.head == "008ea89bda55ad0c0ce2973c1a963aa5ded94142"
  and .source_evidence.avd == "Hepta_Pixel_API_34_arm64"
  and .source_evidence.receipt.sha256 == "5a2eca42e63b16fda215c50f1d308054c53a5d81235cf2345cbbc6608d4da57b"
  and .source_evidence.receipt.external_not_committed == true
  and (.source_evidence.receipt | keys | sort) == ["external_not_committed","sha256"]
  and .source_evidence.captured_frames.portrait_sha256 == "6993c374462c50c6e7a8de5c8cd2d268ff62d71946ec8867aba315c21905cdbf"
  and .source_evidence.captured_frames.landscape_sha256 == "b939f1ed6365d576b03bbc43a59c7e606caa322076fd0698264f1572aee18549"
  and .source_evidence.captured_frames.ime_sha256 == "4e5ea0b8ddcc4eaea69fe5578b2da14dfe07e5746ff2c50ab736963d3477f617"
  and (.source_evidence.captured_frames | keys | sort) == ["ime_sha256","landscape_sha256","portrait_sha256"]
  and .source_evidence.claim_scope == "visual_template_seed_only"
  and (.source_evidence | keys | sort) == ["avd","captured_frames","claim_scope","head","receipt"]
  and (.templates | keys | sort) == ["ime","landscape","portrait"]
  and .templates.portrait.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/portrait.png"
  and .templates.portrait.sha256 == "4fbb29889ea08ef42d5238b0e38c4b72fc9824df9c1361aca03c2cf89fe02826"
  and .templates.landscape.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/landscape.png"
  and .templates.landscape.sha256 == "e7cc31e2ba00ec8e29400db006930466099b3487fcfe4e76542d7b14a4d7dfb5"
  and .templates.ime.path == "apps/hepta-native/packaging/android-emulator-login-template-v1/ime.png"
  and .templates.ime.sha256 == "447bb191a70e321ebabcfe7ad6a8ba541c69e5a85ce493fee675e8f2c5434b78"
  and .claim_boundary == "Visual template identity only. No semantic accessibility, authentication, real-device, secure-credential, or release claim.";

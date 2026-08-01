# Robrix Copy Manifest for Hepta Native

Date: 2026-05-14

## Source

- Source repo: `https://github.com/project-robius/robrix`
- Source commit: `b2bb6cf`
- Source license: MIT (`apps/hepta-native/LICENSE-MIT`)
- Destination root: `apps/hepta-native/`

## Policy

This manifest records the initial direct transplant of Robrix into Hepta for the Matrix-heart fast path. Files are copied first to preserve a working baseline; later patches should update `modification_summary` for heavily modified files.

## File manifest

| source_repo | source_commit | source_license | copied_file | hepta_destination | modification_summary | status |
| --- | --- | --- | --- | --- | --- | --- |
| project-robius/robrix | b2bb6cf | MIT | `.cargo/config.toml` | `apps/hepta-native/.cargo/config.toml` | copied then Makepad bundle identifier changed to ai.hepta.native; Android CLI package override uses Java-safe ai.hepta.nativeapp | renamed |
| project-robius/robrix | b2bb6cf | MIT | `.github/actions/setup-cargo-makepad/action.yml` | `apps/hepta-native/.github/actions/setup-cargo-makepad/action.yml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `.github/actions/setup-cargo-makepad/scripts/resolve-makepad-rev.sh` | `apps/hepta-native/.github/actions/setup-cargo-makepad/scripts/resolve-makepad-rev.sh` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `.github/workflows/builds.yml` | `apps/hepta-native/.github/workflows/builds.yml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `.github/workflows/main.yml` | `apps/hepta-native/.github/workflows/main.yml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `.github/workflows/release.yml` | `apps/hepta-native/.github/workflows/release.yml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `.gitignore` | `apps/hepta-native/.gitignore` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `AGENTS.md` | `apps/hepta-native/AGENTS.md` | excluded from the reviewable transplant because it is local agent guidance, not runtime source or attribution material | excluded |
| project-robius/robrix | b2bb6cf | MIT | `Cargo.lock` | `apps/hepta-native/Cargo.lock` | Cargo lock updated by cargo metadata/check after package rename | renamed |
| project-robius/robrix | b2bb6cf | MIT | `Cargo.toml` | `apps/hepta-native/Cargo.toml` | copied then package/product metadata renamed for Hepta Native | renamed |
| project-robius/robrix | b2bb6cf | MIT | `LICENSE-MIT` | `apps/hepta-native/LICENSE-MIT` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `License Attributions.md` | `apps/hepta-native/License Attributions.md` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `README.md` | `apps/hepta-native/README.md` | rewritten as a Hepta Native product README with upstream Robrix attribution only; obsolete upstream Robrix app commands removed | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `SPLASH.md` | `apps/hepta-native/SPLASH.md` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `build.rs` | `apps/hepta-native/build.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/Entitlements.plist` | `apps/hepta-native/packaging/Entitlements.plist` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/Info.plist` | `apps/hepta-native/packaging/Info.plist` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/Robrix macOS dmg background.png` | `apps/hepta-native/packaging/Hepta Native macOS dmg background.png` | replaced with Hepta Native DMG background during productization cleanup; original source remains attributed | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `packaging/build-macos-dmg.sh` | `apps/hepta-native/packaging/build-macos-dmg.sh` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/fix-dmg-applications-icon.sh` | `apps/hepta-native/packaging/fix-dmg-applications-icon.sh` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/icon_google_play_512.png` | `apps/hepta-native/packaging/icon_google_play_512.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon1024x1024.png` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon1024x1024.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon120x120.png` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon120x120.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon152x152.png` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon152x152.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon167x167.png` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon167x167.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon180x180.png` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/AppIcon180x180.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/Contents.json` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/Contents.json` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/Contents.json` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/Contents.json` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/ios/icons/Assets.xcassets/LaunchScreenBackground.colorset/Contents.json` | `apps/hepta-native/packaging/ios/icons/Assets.xcassets/LaunchScreenBackground.colorset/Contents.json` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `packaging/robrix.desktop` | `apps/hepta-native/packaging/hepta-native.desktop` | renamed and rewritten as Hepta Native desktop metadata | renamed |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-anydpi-v26/ic_launcher.xml` | `apps/hepta-native/resources/android/res/mipmap-anydpi-v26/ic_launcher.xml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-anydpi-v26/ic_launcher_round.xml` | `apps/hepta-native/resources/android/res/mipmap-anydpi-v26/ic_launcher_round.xml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-hdpi/ic_launcher.png` | `apps/hepta-native/resources/android/res/mipmap-hdpi/ic_launcher.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-hdpi/ic_launcher_foreground.png` | `apps/hepta-native/resources/android/res/mipmap-hdpi/ic_launcher_foreground.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-mdpi/ic_launcher.png` | `apps/hepta-native/resources/android/res/mipmap-mdpi/ic_launcher.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-mdpi/ic_launcher_foreground.png` | `apps/hepta-native/resources/android/res/mipmap-mdpi/ic_launcher_foreground.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-xhdpi/ic_launcher.png` | `apps/hepta-native/resources/android/res/mipmap-xhdpi/ic_launcher.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-xhdpi/ic_launcher_foreground.png` | `apps/hepta-native/resources/android/res/mipmap-xhdpi/ic_launcher_foreground.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-xxhdpi/ic_launcher.png` | `apps/hepta-native/resources/android/res/mipmap-xxhdpi/ic_launcher.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-xxhdpi/ic_launcher_foreground.png` | `apps/hepta-native/resources/android/res/mipmap-xxhdpi/ic_launcher_foreground.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-xxxhdpi/ic_launcher.png` | `apps/hepta-native/resources/android/res/mipmap-xxxhdpi/ic_launcher.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/mipmap-xxxhdpi/ic_launcher_foreground.png` | `apps/hepta-native/resources/android/res/mipmap-xxxhdpi/ic_launcher_foreground.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/android/res/values/ic_launcher_background.xml` | `apps/hepta-native/resources/android/res/values/ic_launcher_background.xml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon.ico` | `apps/hepta-native/resources/icon.ico` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_1024.png` | `apps/hepta-native/resources/icon_1024.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_128.png` | `apps/hepta-native/resources/icon_128.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_256.png` | `apps/hepta-native/resources/icon_256.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_32.png` | `apps/hepta-native/resources/icon_32.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_48.png` | `apps/hepta-native/resources/icon_48.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_512.png` | `apps/hepta-native/resources/icon_512.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_64.png` | `apps/hepta-native/resources/icon_64.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_comment.svg` | `apps/hepta-native/resources/icon_comment.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_favorite.svg` | `apps/hepta-native/resources/icon_favorite.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_find.svg` | `apps/hepta-native/resources/icon_find.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_jump_to_bottom.svg` | `apps/hepta-native/resources/icon_jump_to_bottom.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_likes.svg` | `apps/hepta-native/resources/icon_likes.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_send.svg` | `apps/hepta-native/resources/icon_send.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icon_user.svg` | `apps/hepta-native/resources/icon_user.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/add.svg` | `apps/hepta-native/resources/icons/add.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/add_image.svg` | `apps/hepta-native/resources/icons/add_image.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/add_reaction.svg` | `apps/hepta-native/resources/icons/add_reaction.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/add_user.svg` | `apps/hepta-native/resources/icons/add_user.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/add_wallet.svg` | `apps/hepta-native/resources/icons/add_wallet.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/checkmark.svg` | `apps/hepta-native/resources/icons/checkmark.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/close.svg` | `apps/hepta-native/resources/icons/close.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/cloud_checkmark.svg` | `apps/hepta-native/resources/icons/cloud_checkmark.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/cloud_offline.svg` | `apps/hepta-native/resources/icons/cloud_offline.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/collapse.svg` | `apps/hepta-native/resources/icons/collapse.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/copy.svg` | `apps/hepta-native/resources/icons/copy.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/double_chat.svg` | `apps/hepta-native/resources/icons/double_chat.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/edit.svg` | `apps/hepta-native/resources/icons/edit.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/edit2.svg` | `apps/hepta-native/resources/icons/edit2.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/external_link.svg` | `apps/hepta-native/resources/icons/external_link.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/eye_closed.svg` | `apps/hepta-native/resources/icons/eye_closed.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/eye_open.svg` | `apps/hepta-native/resources/icons/eye_open.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/forbidden.svg` | `apps/hepta-native/resources/icons/forbidden.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/go_back.svg` | `apps/hepta-native/resources/icons/go_back.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/hierarchy.svg` | `apps/hepta-native/resources/icons/hierarchy.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/home.svg` | `apps/hepta-native/resources/icons/home.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/html_file.svg` | `apps/hepta-native/resources/icons/html_file.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/import.svg` | `apps/hepta-native/resources/icons/import.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/info.svg` | `apps/hepta-native/resources/icons/info.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/invite.svg` | `apps/hepta-native/resources/icons/invite.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/join_room.svg` | `apps/hepta-native/resources/icons/join_room.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/link.svg` | `apps/hepta-native/resources/icons/link.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/location-person.svg` | `apps/hepta-native/resources/icons/location-person.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/logout.svg` | `apps/hepta-native/resources/icons/logout.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/menu.svg` | `apps/hepta-native/resources/icons/menu.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/money.svg` | `apps/hepta-native/resources/icons/money.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/pin.svg` | `apps/hepta-native/resources/icons/pin.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/pin2.svg` | `apps/hepta-native/resources/icons/pin2.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/reply.svg` | `apps/hepta-native/resources/icons/reply.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/rotate_left_fa.svg` | `apps/hepta-native/resources/icons/rotate_left_fa.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/rotate_right_fa.svg` | `apps/hepta-native/resources/icons/rotate_right_fa.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/scan.svg` | `apps/hepta-native/resources/icons/scan.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/search.svg` | `apps/hepta-native/resources/icons/search.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/settings.svg` | `apps/hepta-native/resources/icons/settings.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/squares_filled.svg` | `apps/hepta-native/resources/icons/squares_filled.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/tombstone.svg` | `apps/hepta-native/resources/icons/tombstone.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/trash.svg` | `apps/hepta-native/resources/icons/trash.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/triangle_down_fill.svg` | `apps/hepta-native/resources/icons/triangle_down_fill.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/triangle_up_fill.svg` | `apps/hepta-native/resources/icons/triangle_up_fill.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/upload.svg` | `apps/hepta-native/resources/icons/upload.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/verification_no.svg` | `apps/hepta-native/resources/icons/verification_no.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/verification_no_bg.svg` | `apps/hepta-native/resources/icons/verification_no_bg.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/verification_unk.svg` | `apps/hepta-native/resources/icons/verification_unk.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/verification_unk_bg.svg` | `apps/hepta-native/resources/icons/verification_unk_bg.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/verification_yes.svg` | `apps/hepta-native/resources/icons/verification_yes.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/verification_yes_bg.svg` | `apps/hepta-native/resources/icons/verification_yes_bg.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/view_source.svg` | `apps/hepta-native/resources/icons/view_source.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/view_source2.svg` | `apps/hepta-native/resources/icons/view_source2.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/view_source3.svg` | `apps/hepta-native/resources/icons/view_source3.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/warning.svg` | `apps/hepta-native/resources/icons/warning.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/zoom_in.svg` | `apps/hepta-native/resources/icons/zoom_in.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/icons/zoom_out.svg` | `apps/hepta-native/resources/icons/zoom_out.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/apple.png` | `apps/hepta-native/resources/img/apple.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/default_image.png` | `apps/hepta-native/resources/img/default_image.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/facebook.png` | `apps/hepta-native/resources/img/facebook.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/github.png` | `apps/hepta-native/resources/img/github.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/gitlab.png` | `apps/hepta-native/resources/img/gitlab.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/google.png` | `apps/hepta-native/resources/img/google.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/qr_icon.png` | `apps/hepta-native/resources/img/qr_icon.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/img/x.png` | `apps/hepta-native/resources/img/x.png` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/menu.svg` | `apps/hepta-native/resources/menu.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `resources/search.svg` | `apps/hepta-native/resources/search.svg` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `rust-toolchain.toml` | `apps/hepta-native/rust-toolchain.toml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `rustfmt.toml` | `apps/hepta-native/rustfmt.toml` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/app.rs` | `apps/hepta-native/src/app.rs` | copied then visible app/window/menu labels changed to Hepta Native and fixture mode added to boot the shell without Matrix login | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/avatar_cache.rs` | `apps/hepta-native/src/avatar_cache.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/event_preview.rs` | `apps/hepta-native/src/event_preview.rs` | copied then Hepta custom Matrix-style event preview labels added for m.hepta.* events | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/home/add_room.rs` | `apps/hepta-native/src/home/add_room.rs` | copied then public alias example changed to #hepta:matrix.org | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/home/edited_indicator.rs` | `apps/hepta-native/src/home/edited_indicator.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/editing_pane.rs` | `apps/hepta-native/src/home/editing_pane.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/event_reaction_list.rs` | `apps/hepta-native/src/home/event_reaction_list.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/event_source_modal.rs` | `apps/hepta-native/src/home/event_source_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/home_screen.rs` | `apps/hepta-native/src/home/home_screen.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/invite_modal.rs` | `apps/hepta-native/src/home/invite_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/invite_screen.rs` | `apps/hepta-native/src/home/invite_screen.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/light_themed_dock.rs` | `apps/hepta-native/src/home/light_themed_dock.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/link_preview.rs` | `apps/hepta-native/src/home/link_preview.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/loading_pane.rs` | `apps/hepta-native/src/home/loading_pane.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/location_preview.rs` | `apps/hepta-native/src/home/location_preview.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/main_desktop_ui.rs` | `apps/hepta-native/src/home/main_desktop_ui.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/main_mobile_ui.rs` | `apps/hepta-native/src/home/main_mobile_ui.rs` | copied then mobile welcome/cockpit surface wired to Hepta fixture/detail panes while Matrix room stack remains intact | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/home/mod.rs` | `apps/hepta-native/src/home/mod.rs` | copied then Hepta-owned fixture cockpit, inspector, and mobile detail modules registered | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/home/navigation_tab_bar.rs` | `apps/hepta-native/src/home/navigation_tab_bar.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/new_message_context_menu.rs` | `apps/hepta-native/src/home/new_message_context_menu.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/room_context_menu.rs` | `apps/hepta-native/src/home/room_context_menu.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/room_image_viewer.rs` | `apps/hepta-native/src/home/room_image_viewer.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/room_read_receipt.rs` | `apps/hepta-native/src/home/room_read_receipt.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/room_screen.rs` | `apps/hepta-native/src/home/room_screen.rs` | copied then heavily modified with Hepta custom event cards, inspect-payload action, local approval buttons, fixture timeline, and exact-payload preview confirmation flow | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/home/rooms_list.rs` | `apps/hepta-native/src/home/rooms_list.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/rooms_list_entry.rs` | `apps/hepta-native/src/home/rooms_list_entry.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/rooms_list_header.rs` | `apps/hepta-native/src/home/rooms_list_header.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/rooms_sidebar.rs` | `apps/hepta-native/src/home/rooms_sidebar.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/search_messages.rs` | `apps/hepta-native/src/home/search_messages.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/space_lobby.rs` | `apps/hepta-native/src/home/space_lobby.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/spaces_bar.rs` | `apps/hepta-native/src/home/spaces_bar.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/tombstone_footer.rs` | `apps/hepta-native/src/home/tombstone_footer.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/home/welcome_screen.rs` | `apps/hepta-native/src/home/welcome_screen.rs` | copied then welcome copy changed to Hepta Native / Matrix-heart mode | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/join_leave_room_modal.rs` | `apps/hepta-native/src/join_leave_room_modal.rs` | copied then restart copy changed to Hepta Native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/lib.rs` | `apps/hepta-native/src/lib.rs` | copied then app constants renamed to ai.hepta.hepta-native and Hepta custom/fixture modules exported | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/location.rs` | `apps/hepta-native/src/location.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/login/login_screen.rs` | `apps/hepta-native/src/login/login_screen.rs` | copied then login title changed to Hepta Native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/login/login_status_modal.rs` | `apps/hepta-native/src/login/login_status_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/login/mod.rs` | `apps/hepta-native/src/login/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/logout/logout_confirm_modal.rs` | `apps/hepta-native/src/logout/logout_confirm_modal.rs` | copied then logout/restart copy changed to Hepta Native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/logout/logout_errors.rs` | `apps/hepta-native/src/logout/logout_errors.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/logout/logout_state_machine.rs` | `apps/hepta-native/src/logout/logout_state_machine.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/logout/mod.rs` | `apps/hepta-native/src/logout/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/main.rs` | `apps/hepta-native/src/main.rs` | copied then crate call changed from robrix to hepta_native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/media_cache.rs` | `apps/hepta-native/src/media_cache.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/persistence/app_state.rs` | `apps/hepta-native/src/persistence/app_state.rs` | copied then default window title changed to Hepta Native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/persistence/matrix_state.rs` | `apps/hepta-native/src/persistence/matrix_state.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/persistence/mod.rs` | `apps/hepta-native/src/persistence/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/persistence/tsp_state.rs` | `apps/hepta-native/src/persistence/tsp_state.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/profile/mod.rs` | `apps/hepta-native/src/profile/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/profile/user_profile.rs` | `apps/hepta-native/src/profile/user_profile.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/profile/user_profile_cache.rs` | `apps/hepta-native/src/profile/user_profile_cache.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/room/mod.rs` | `apps/hepta-native/src/room/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/room/reply_preview.rs` | `apps/hepta-native/src/room/reply_preview.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/room/room_display_filter.rs` | `apps/hepta-native/src/room/room_display_filter.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/room/room_input_bar.rs` | `apps/hepta-native/src/room/room_input_bar.rs` | copied then Hepta slash/action bridge dry-run preview commands, inline composer preview pane, and Matrix typing-notice suppression for reserved Hepta commands added without Matrix/Gateway mutation | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/room/typing_notice.rs` | `apps/hepta-native/src/room/typing_notice.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/room_preview_cache.rs` | `apps/hepta-native/src/room_preview_cache.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/settings/account_settings.rs` | `apps/hepta-native/src/settings/account_settings.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/settings/app_preferences.rs` | `apps/hepta-native/src/settings/app_preferences.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/settings/app_settings.rs` | `apps/hepta-native/src/settings/app_settings.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/settings/mod.rs` | `apps/hepta-native/src/settings/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/settings/settings_screen.rs` | `apps/hepta-native/src/settings/settings_screen.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/avatar.rs` | `apps/hepta-native/src/shared/avatar.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/bouncing_dots.rs` | `apps/hepta-native/src/shared/bouncing_dots.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/collapsible_header.rs` | `apps/hepta-native/src/shared/collapsible_header.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/command_text_input.rs` | `apps/hepta-native/src/shared/command_text_input.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/confirmation_modal.rs` | `apps/hepta-native/src/shared/confirmation_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/expand_arrow.rs` | `apps/hepta-native/src/shared/expand_arrow.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/helpers.rs` | `apps/hepta-native/src/shared/helpers.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/html_or_plaintext.rs` | `apps/hepta-native/src/shared/html_or_plaintext.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/icon_button.rs` | `apps/hepta-native/src/shared/icon_button.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/image_viewer.rs` | `apps/hepta-native/src/shared/image_viewer.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/jump_to_bottom_button.rs` | `apps/hepta-native/src/shared/jump_to_bottom_button.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/mentionable_text_input.rs` | `apps/hepta-native/src/shared/mentionable_text_input.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/mod.rs` | `apps/hepta-native/src/shared/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/navigation_bar_button.rs` | `apps/hepta-native/src/shared/navigation_bar_button.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/popup_list.rs` | `apps/hepta-native/src/shared/popup_list.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/restore_status_view.rs` | `apps/hepta-native/src/shared/restore_status_view.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/room_filter_input_bar.rs` | `apps/hepta-native/src/shared/room_filter_input_bar.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/styles.rs` | `apps/hepta-native/src/shared/styles.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/text_or_image.rs` | `apps/hepta-native/src/shared/text_or_image.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/timestamp.rs` | `apps/hepta-native/src/shared/timestamp.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/unread_badge.rs` | `apps/hepta-native/src/shared/unread_badge.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/shared/verification_badge.rs` | `apps/hepta-native/src/shared/verification_badge.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/sliding_sync.rs` | `apps/hepta-native/src/sliding_sync.rs` | copied then device names, restart copy, and SSO callback scheme changed to Hepta Native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/space_service_sync.rs` | `apps/hepta-native/src/space_service_sync.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/temp_storage.rs` | `apps/hepta-native/src/temp_storage.rs` | copied then temp dir name changed to hepta_native_temp | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/create_did_modal.rs` | `apps/hepta-native/src/tsp/create_did_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/create_wallet_modal.rs` | `apps/hepta-native/src/tsp/create_wallet_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/mod.rs` | `apps/hepta-native/src/tsp/mod.rs` | copied then TSP user agent changed to Hepta Native | heavily_modified |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/sign_anycast_checkbox.rs` | `apps/hepta-native/src/tsp/sign_anycast_checkbox.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/tsp_settings_screen.rs` | `apps/hepta-native/src/tsp/tsp_settings_screen.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/tsp_sign_indicator.rs` | `apps/hepta-native/src/tsp/tsp_sign_indicator.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/tsp_verification_modal.rs` | `apps/hepta-native/src/tsp/tsp_verification_modal.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/verify_user.rs` | `apps/hepta-native/src/tsp/verify_user.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp/wallet_entry/mod.rs` | `apps/hepta-native/src/tsp/wallet_entry/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/tsp_dummy/mod.rs` | `apps/hepta-native/src/tsp_dummy/mod.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/utils.rs` | `apps/hepta-native/src/utils.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/verification.rs` | `apps/hepta-native/src/verification.rs` | initial direct copy from Robrix baseline | copied |
| project-robius/robrix | b2bb6cf | MIT | `src/verification_modal.rs` | `apps/hepta-native/src/verification_modal.rs` | initial direct copy from Robrix baseline | copied |
| Hepta | n/a | MIT | n/a | `apps/hepta-native/src/hepta_event.rs` | new Hepta custom Matrix-style event helper module for m.hepta.* event types | added |
| Hepta | n/a | MIT | n/a | `apps/hepta-native/src/hepta_fixture.rs` | new local fixture-mode conversation for desktop/mobile UI bootstrapping before Hepta's native OpenClaw-parity runtime is live | added |

## Hepta-owned Matrix-heart additions after initial import

These files were added after the Robrix baseline transplant and are Hepta-owned unless otherwise noted:

| hepta_file | ownership | purpose | status |
| --- | --- | --- | --- |
| `apps/hepta-native/src/hepta_event.rs` | Hepta | typed `m.hepta.*` custom event constants, envelopes, validation, and card text helpers | added |
| `apps/hepta-native/src/hepta_fixture.rs` | Hepta | no-homeserver fixture mode, sample runtime cockpit conversation, selectable local workspace injection | added |
| `apps/hepta-native/src/hepta_bridge.rs` | Hepta | side-effect-free Hepta-runtime-to-Matrix event shape seam for native OpenClaw-parity capability replication | added |
| `apps/hepta-native/src/hepta_action_bridge.rs` | Hepta | side-effect-free controlled-mutation policy gate for local/read-only/draft/blocked action classes | added |
| `apps/hepta-native/src/hepta_action_queue.rs` | Hepta | side-effect-free staged action outbox model and sample queue generation for composer drafts, approval previews, blocked policy classes, and redacted evidence | added |
| `apps/hepta-native/src/hepta_composer.rs` | Hepta | bounded `/hepta ...` composer command parser and dry-run planner for staged local actions | added |
| `apps/hepta-native/src/hepta_command_templates.rs` | Hepta | validated quick-command template model for status/task/agent/tool/approval dry-run composer actions | added |
| `apps/hepta-native/src/hepta_context_snapshot.rs` | Hepta | read-only context chip snapshot model for agent/task/session/memory/artifact composer context classes | added |
| `apps/hepta-native/src/hepta_runtime_status.rs` | Hepta | read-only local runtime status snapshot model for fixture readiness, preview-only action paths, and packaging blockers | added |
| `apps/hepta-native/src/home/hepta_action_outbox.rs` | Hepta | shared desktop/mobile action outbox pane dynamically populated from staged preview, exact confirmation, policy-blocked, and evidence queue lanes | added |
| `apps/hepta-native/src/home/hepta_command_templates.rs` | Hepta | shared desktop/mobile quick-command template pane populated from dry-run-validated command templates | added |
| `apps/hepta-native/src/home/hepta_context_snapshot.rs` | Hepta | shared desktop/mobile context snapshot pane populated from the read-only context chip model | added |
| `apps/hepta-native/src/home/hepta_runtime_status.rs` | Hepta | shared desktop/mobile runtime status pane populated from the read-only status snapshot model | added |
| `apps/hepta-native/src/home/hepta_fixture_cockpit.rs` | Hepta | local fixture cockpit card surface for desktop/mobile UI validation | added |
| `apps/hepta-native/src/home/hepta_inspector.rs` | Hepta | desktop right-side inspector/control pane for runtime, task, approval, context, and mobile policy status | added |
| `apps/hepta-native/src/home/hepta_mobile_detail.rs` | Hepta | mobile drill-down detail pane documenting shared Matrix-heart substrate and draft-first mutation policy | added |
| `apps/hepta-native/src/home/hepta_mobile_safety.rs` | Hepta | persistent mobile room safety bar for inspect-payload, confirm-preview, and live-blocked status | added |

## Hepta modification notes after M2/M3 UI work

- `apps/hepta-native/src/app.rs`: fixture mode now marks the app logged in, injects the local `Hepta Runtime Cockpit` workspace, and skips Matrix SDK startup while local fixture mode is active.
- `apps/hepta-native/src/lib.rs`: exports Hepta event, fixture, bridge, action-bridge policy, action queue, command templates, context snapshot, runtime status, and composer modules.
- `apps/hepta-native/src/event_preview.rs`: routes known `m.hepta.*` events to Hepta preview labels.
- `apps/hepta-native/src/home/room_screen.rs`: renders first-class `HeptaEventCard`s, adds policy badges, inspect-payload and local approval buttons, routes approve/reject clicks through shared exact-payload confirmation modals, and shows a selectable fixture timeline surface for `!hepta-runtime-fixture:local`.
- `apps/hepta-native/src/room/room_input_bar.rs`: stages explicit `/task`, `/agent`, `/tool`, `/approve`, `/reject`, `/status`, and `/hepta ...` commands locally as dry-run native action previews without Matrix send or OpenClaw Gateway mutation; shows an inline composer preview and suppresses Matrix typing notices for reserved Hepta command prefixes.
- `apps/hepta-native/src/home/hepta_inspector.rs` and `apps/hepta-native/src/home/hepta_mobile_detail.rs`: share the Hepta runtime status, command templates, context snapshot, and action outbox panes so desktop and mobile expose the same read-only runtime/context, staged, blocked, and evidence mutation lanes.
- `apps/hepta-native/src/home/main_mobile_ui.rs`: adds a persistent Hepta mobile safety bar above the Matrix-heart room screen, keeping live mutation state visible in actual mobile room context.
- `apps/hepta-native/src/home/main_desktop_ui.rs`: adds a persistent desktop Inspector / Control pane beside the Matrix-heart conversation surface.
- `apps/hepta-native/src/home/welcome_screen.rs`: introduces the Hepta runtime cockpit overview, embedded fixture cockpit, and mobile drill-down detail surface.
- `apps/hepta-native/src/home/navigation_tab_bar.rs`, `apps/hepta-native/src/home/rooms_list_header.rs`, and `apps/hepta-native/src/home/add_room.rs`: reframe Robrix room chrome as Hepta agent cockpit / workspace connection language while keeping Matrix transport semantics.

## Selective upstream UI intake on 2026-08-01

This bounded intake was reviewed against Robrix `main` at exact commit
`a5a664da569c577ab1a3e5a33f45dcc9364954a0`. The initial Hepta source baseline remains
`b2bb6cf33a51e5c8a0a91ebca2025f09212304bd`; GitHub's compare API reports the selected
upstream commit as 267 commits ahead and 0 behind that baseline. No whole-tree merge was
performed.

| source_repo | source_commit | source_license | upstream_file | hepta_destination | modification_summary | status |
| --- | --- | --- | --- | --- | --- | --- |
| project-robius/robrix | a5a664da569c577ab1a3e5a33f45dcc9364954a0 | MIT | `src/shared/slash_commands.rs` | `apps/hepta-native/src/shared/slash_commands.rs` | retained the `/html` and `/plain` parser/message-construction behavior; added deterministic tests; intentionally not wired to send dispatch | adapted_parser_ready_not_wired |
| project-robius/robrix | a5a664da569c577ab1a3e5a33f45dcc9364954a0 | MIT | `src/shared/file_upload_modal.rs` | `apps/hepta-native/src/shared/file_upload_modal.rs` | retained local metadata inspection, bounded text preview, image-kind metadata, attempt IDs, and large-file threshold; removed duplicate modal, `robius-file-picker`, GPU decode, Matrix request submission, and TSP behavior so Hepta's confirmation-first composer stays authoritative | quarantined_preflight_ready_no_picker_or_send |
| project-robius/robrix | a5a664da569c577ab1a3e5a33f45dcc9364954a0 | MIT | `src/home/upload_progress.rs` | `apps/hepta-native/src/home/upload_progress.rs` | retained deterministic progress, failure, cancel-eligibility, and retry-eligibility presentation state; removed upstream abort handle and retry submission because accepted queue ownership remains on the Hepta timeline | quarantined_progress_model_no_queue_control |
| project-robius/robrix | a5a664da569c577ab1a3e5a33f45dcc9364954a0 | MIT | `src/shared/attachment_download.rs` | `apps/hepta-native/src/shared/attachment_download.rs` | retained MXC extraction and download/share presentation states; removed file I/O, save dialog, share sheet, popup notification, timer, and Matrix worker submission pending explicit platform-adapter work | quarantined_download_model_no_io_or_share |
| project-robius/robrix | a5a664da569c577ab1a3e5a33f45dcc9364954a0 | MIT | `src/shared/mention_popup.rs` | `apps/hepta-native/src/shared/mention_popup.rs` | retained item vocabulary, local insertion text, result replacement, and wrapping keyboard selection; removed remote ranking, avatar fetching, global overlay ownership, and message submission to preserve Hepta's existing cached local mention path | quarantined_selection_model_no_remote_lookup |
| project-robius/robrix | a5a664da569c577ab1a3e5a33f45dcc9364954a0 | MIT | `src/shared/room_input_popup_menu.rs` | `apps/hepta-native/src/shared/room_input_popup_menu.rs` | retained the registered Makepad popup, actions, Escape/back close, outside-event helpers, and 44-point rows; restyled with generated Hepta light-glass tokens and existing icons; deliberately not wired over the custom confirmation-first RoomInputBar | registered_widget_not_wired_to_custom_composer |

The upstream versions also assume newer Makepad, `robius-file-picker`, `robius-share`, newer
Matrix worker variants, and the redesigned upstream MentionableTextInput. Those dependency and
runtime changes are outside this UI-only intake and were not added transitively.

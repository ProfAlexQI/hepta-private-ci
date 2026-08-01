use super::*;

pub(super) fn emoji_sticker_lifecycle_metadata_label(
    action: &str,
    panel_visible: bool,
    last_choice: Option<&str>,
    stage_count: usize,
    local_status: Option<&str>,
) -> String {
    let panel_state = if panel_visible {
        "panel visible"
    } else {
        "panel hidden"
    };
    let choice_state = last_choice
        .filter(|choice| !choice.trim().is_empty())
        .map(|choice| format!("last choice {choice}"))
        .unwrap_or_else(|| "no staged emoji/sticker choice".to_string());
    let status_state = local_status
        .filter(|status| !status.trim().is_empty())
        .unwrap_or("no local emoji/sticker status");
    format!(
        "Emoji/sticker lifecycle {action}: {panel_state}; {choice_state}; staged count {stage_count}; status: {status_state}. {EMOJI_STICKER_LIFECYCLE_METADATA_LABEL}"
    )
}

#[cfg(test)]
mod emoji_sticker_lifecycle_metadata_tests {
    use super::*;

    #[test]
    fn emoji_sticker_lifecycle_metadata_label_summarizes_repeated_selection() {
        let label = emoji_sticker_lifecycle_metadata_label(
            "staged Heart",
            true,
            Some("Heart"),
            3,
            Some("Heart emoji/sticker preview staged locally"),
        );

        assert!(label.contains("Emoji/sticker lifecycle staged Heart"));
        assert!(label.contains("panel visible"));
        assert!(label.contains("last choice Heart"));
        assert!(label.contains("staged count 3"));
        assert!(label.contains("Heart emoji/sticker preview staged locally"));
        assert!(label.contains(EMOJI_STICKER_LIFECYCLE_METADATA_LABEL));
        assert!(
            EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE
                .contains("repeated Smile/Thumbs/Heart/Sticker staging")
        );
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("last staged choice"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("staged choice count"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("remote picker/search"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("gateway/runtime/auth"));
        assert!(EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn emoji_sticker_lifecycle_metadata_label_uses_empty_fallbacks() {
        let label = emoji_sticker_lifecycle_metadata_label("closed", false, None, 0, Some(""));

        assert!(label.contains("Emoji/sticker lifecycle closed"));
        assert!(label.contains("panel hidden"));
        assert!(label.contains("no staged emoji/sticker choice"));
        assert!(label.contains("staged count 0"));
        assert!(label.contains("no local emoji/sticker status"));
    }
}

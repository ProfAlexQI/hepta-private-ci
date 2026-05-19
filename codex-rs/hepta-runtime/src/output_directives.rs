use hepta_core::HeptaError;
use serde::{Deserialize, Serialize};

use crate::{
    ChannelSendHandoffInput, ChannelSendHandoffReport, DurableDeliveryQueue, ReadbackEvidenceLedger,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputDirectivePlan {
    pub visible_text: String,
    pub media_refs: Vec<String>,
    pub reply_to: Option<String>,
    pub audio_as_voice_hint_present: bool,
    pub voice_note_delivery_planned: bool,
    pub silent_reply: bool,
    pub delivery_required: bool,
    pub readback_evidence_required: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutputDirectiveDeliveryHandoffInput {
    pub target: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutputDirectiveDeliveryHandoffReport {
    pub plan: OutputDirectivePlan,
    pub handoff: Option<ChannelSendHandoffReport>,
    pub delivery_required: bool,
    pub silent_reply: bool,
    pub queue_mutated_by_gate: bool,
    pub external_send_performed_by_gate: bool,
}

pub fn plan_output_directives(assistant_output: &str) -> Result<OutputDirectivePlan, HeptaError> {
    let mut working = assistant_output.trim().to_string();
    let mut warnings = Vec::new();
    let reply_to = parse_and_strip_leading_reply_tag(&mut working)?;
    let audio_as_voice_hint_present = strip_audio_as_voice_tags(&mut working);
    let mut visible_lines = Vec::new();
    let mut media_refs = Vec::new();
    let mut silent_reply = false;

    for line in working.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if matches!(trimmed, "NO_REPLY" | "no_reply") {
            silent_reply = true;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("MEDIA:") {
            let media_ref = rest.trim();
            if media_ref.is_empty() {
                warnings.push("empty MEDIA directive ignored".into());
            } else if media_ref.contains(char::is_whitespace) {
                warnings.push("MEDIA directive contains whitespace and was ignored".into());
            } else {
                media_refs.push(redact_media_ref(media_ref));
            }
            continue;
        }
        if trimmed.contains("MEDIA:") {
            warnings.push("inline MEDIA directive ignored; MEDIA must start its own line".into());
        }
        visible_lines.push(trimmed.to_string());
    }

    if silent_reply {
        visible_lines.clear();
        media_refs.clear();
    }

    let audio_media_present = media_refs
        .iter()
        .any(|media_ref| is_audio_media_ref(media_ref));
    let voice_note_delivery_planned =
        !silent_reply && audio_as_voice_hint_present && audio_media_present;
    if audio_as_voice_hint_present && !audio_media_present && !silent_reply {
        warnings.push(
            "audio_as_voice hint ignored because no audio-shaped MEDIA directive is present".into(),
        );
    }
    let visible_text = visible_lines.join("\n");
    let delivery_required = !silent_reply && (!visible_text.is_empty() || !media_refs.is_empty());
    Ok(OutputDirectivePlan {
        visible_text,
        media_refs,
        reply_to,
        audio_as_voice_hint_present,
        voice_note_delivery_planned,
        silent_reply,
        delivery_required,
        readback_evidence_required: delivery_required,
        warnings,
    })
}

pub fn handoff_output_directives_to_delivery_queue(
    queue: &DurableDeliveryQueue,
    evidence_ledger: &ReadbackEvidenceLedger,
    assistant_output: &str,
    input: OutputDirectiveDeliveryHandoffInput,
) -> Result<OutputDirectiveDeliveryHandoffReport, HeptaError> {
    let plan = plan_output_directives(assistant_output)?;
    if !plan.delivery_required {
        return Ok(OutputDirectiveDeliveryHandoffReport {
            delivery_required: false,
            silent_reply: plan.silent_reply,
            plan,
            handoff: None,
            queue_mutated_by_gate: false,
            external_send_performed_by_gate: false,
        });
    }
    let delivery_kind = if plan.voice_note_delivery_planned {
        "voice-note-send"
    } else if plan.media_refs.is_empty() {
        "text-send"
    } else {
        "media-send"
    };
    let payload_preview = directive_payload_preview(&plan);
    let handoff = queue.gated_channel_send_handoff(
        evidence_ledger,
        ChannelSendHandoffInput {
            delivery_kind: delivery_kind.into(),
            target: input.target,
            payload_preview,
            policy_decision: input.policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key: input.idempotency_key,
        },
    )?;
    Ok(OutputDirectiveDeliveryHandoffReport {
        delivery_required: true,
        silent_reply: plan.silent_reply,
        queue_mutated_by_gate: handoff.queue_mutated_by_gate,
        external_send_performed_by_gate: handoff.external_send_performed_by_gate,
        plan,
        handoff: Some(handoff),
    })
}

fn directive_payload_preview(plan: &OutputDirectivePlan) -> String {
    let mut parts = Vec::new();
    if !plan.visible_text.is_empty() {
        parts.push(format!("text={}", compact_preview(&plan.visible_text, 180)));
    }
    if !plan.media_refs.is_empty() {
        parts.push(format!("media_refs={}", plan.media_refs.join(",")));
    }
    if let Some(reply_to) = &plan.reply_to {
        parts.push(format!("reply_to={reply_to}"));
    }
    if plan.voice_note_delivery_planned {
        parts.push("audio_as_voice=true".into());
    }
    parts.join("; ")
}

fn compact_preview(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

fn parse_and_strip_leading_reply_tag(value: &mut String) -> Result<Option<String>, HeptaError> {
    let trimmed = value.trim_start();
    if !trimmed.starts_with("[[") {
        return Ok(None);
    }
    let end = trimmed
        .find("]]")
        .ok_or_else(|| HeptaError("unterminated output reply directive".into()))?;
    let raw_tag = &trimmed[2..end];
    let tag = raw_tag.trim();
    let target = if tag == "reply_to_current" {
        Some("current".into())
    } else if let Some(rest) = tag.strip_prefix("reply_to:") {
        let id = rest.trim();
        if id.is_empty() {
            return Err(HeptaError(
                "reply_to directive target must not be empty".into(),
            ));
        }
        Some(redact_reply_target(id))
    } else if tag == "audio_as_voice" {
        None
    } else {
        return Ok(None);
    };
    *value = trimmed[end + 2..].trim_start().to_string();
    Ok(target)
}

fn strip_audio_as_voice_tags(value: &mut String) -> bool {
    let mut found = false;
    let mut output = Vec::new();
    for line in value.lines() {
        let mut line = line.to_string();
        while let Some(start) = line.find("[[") {
            if let Some(relative_end) = line[start + 2..].find("]]") {
                let end = start + 2 + relative_end;
                let tag = line[start + 2..end].trim();
                if tag == "audio_as_voice" {
                    found = true;
                    line.replace_range(start..end + 2, "");
                    continue;
                }
            }
            break;
        }
        output.push(line);
    }
    *value = output.join("\n");
    found
}

fn redact_media_ref(media_ref: &str) -> String {
    if media_ref.contains("token=") || media_ref.contains("secret") {
        "<redacted-media-ref>".into()
    } else {
        media_ref.to_string()
    }
}

fn redact_reply_target(target: &str) -> String {
    if target.len() > 64 {
        format!("{}…", &target[..64])
    } else {
        target.into()
    }
}

fn is_audio_media_ref(media_ref: &str) -> bool {
    let lower = media_ref.to_ascii_lowercase();
    [".mp3", ".wav", ".m4a", ".ogg", ".opus", ".flac"]
        .iter()
        .any(|suffix| lower.ends_with(suffix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DurableDeliveryQueue, ReadbackEvidenceLedger, current_unix_ms};
    use std::{fs, path::PathBuf};

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-output-directives-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn output_directives_plan_media_reply_and_voice_note() {
        let plan = plan_output_directives(
            "[[reply_to_current]] [[audio_as_voice]]\nMEDIA:/tmp/sound.mp3\ncaption",
        )
        .expect("directive plan should parse");
        assert_eq!(plan.reply_to.as_deref(), Some("current"));
        assert_eq!(plan.media_refs, vec!["/tmp/sound.mp3"]);
        assert_eq!(plan.visible_text, "caption");
        assert!(plan.audio_as_voice_hint_present);
        assert!(plan.voice_note_delivery_planned);
        assert!(plan.readback_evidence_required);
    }

    #[test]
    fn output_directives_silent_reply_suppresses_delivery() {
        let plan = plan_output_directives("MEDIA:/tmp/example.png\nNO_REPLY")
            .expect("silent directive should parse");
        assert!(plan.silent_reply);
        assert!(!plan.delivery_required);
        assert!(plan.media_refs.is_empty());
        assert!(plan.visible_text.is_empty());
    }

    #[test]
    fn output_directives_warn_on_inline_media_and_redact_sensitive_refs() {
        let plan = plan_output_directives(
            "hello MEDIA:/tmp/not-standalone.png\nMEDIA:https://example.invalid/a.png?token=abc",
        )
        .expect("directive plan should parse");
        assert_eq!(plan.visible_text, "hello MEDIA:/tmp/not-standalone.png");
        assert_eq!(plan.media_refs, vec!["<redacted-media-ref>"]);
        assert!(
            plan.warnings
                .iter()
                .any(|warning| warning.contains("inline MEDIA"))
        );
    }

    #[test]
    fn output_directives_handoff_respects_silent_reply() {
        let queue_path = temp_file("silent-queue");
        let ledger_path = temp_file("silent-ledger");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let report = handoff_output_directives_to_delivery_queue(
            &queue,
            &ledger,
            "MEDIA:/tmp/example.png\nNO_REPLY",
            OutputDirectiveDeliveryHandoffInput {
                target: "telegram:chat".into(),
                policy_decision: "allow-send".into(),
                operator_confirmed: true,
                idempotency_key: "silent".into(),
            },
        )
        .unwrap();
        assert!(!report.delivery_required);
        assert!(report.silent_reply);
        assert!(report.handoff.is_none());
        assert!(!report.queue_mutated_by_gate);
        assert!(!queue_path.exists());
        assert!(!ledger_path.exists());
    }

    #[test]
    fn output_directives_handoff_queues_delivery_with_readback_gate() {
        let queue_path = temp_file("handoff-queue");
        let ledger_path = temp_file("handoff-ledger");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        assert!(
            handoff_output_directives_to_delivery_queue(
                &queue,
                &ledger,
                "[[reply_to_current]] [[audio_as_voice]]\nMEDIA:/tmp/sound.mp3\ncaption",
                OutputDirectiveDeliveryHandoffInput {
                    target: "telegram:chat".into(),
                    policy_decision: "allow-send".into(),
                    operator_confirmed: false,
                    idempotency_key: "voice-handoff".into(),
                },
            )
            .is_err()
        );
        let report = handoff_output_directives_to_delivery_queue(
            &queue,
            &ledger,
            "[[reply_to_current]] [[audio_as_voice]]\nMEDIA:/tmp/sound.mp3\ncaption",
            OutputDirectiveDeliveryHandoffInput {
                target: "telegram:chat".into(),
                policy_decision: "approved-send".into(),
                operator_confirmed: true,
                idempotency_key: "voice-handoff".into(),
            },
        )
        .unwrap();
        assert!(report.delivery_required);
        assert!(report.queue_mutated_by_gate);
        assert!(!report.external_send_performed_by_gate);
        assert!(report.plan.voice_note_delivery_planned);
        let handoff = report.handoff.expect("handoff should be present");
        assert_eq!(handoff.status.label(), "queued");
        let queue_report = queue.report(None).unwrap();
        assert_eq!(queue_report.queued_count, 1);
        assert_eq!(queue_report.queue.items[0].delivery_kind, "voice-note-send");
        assert!(
            queue_report.queue.items[0]
                .payload_preview
                .contains("audio_as_voice=true")
        );
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }
}

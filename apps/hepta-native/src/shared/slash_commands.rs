//! Slash-command parsing selectively adapted from Robrix.
//!
//! This module deliberately owns only parsing and Matrix message construction. It does not
//! submit a request or mutate the active room, so the existing Hepta composer remains the sole
//! owner of send confirmation and dispatch.

use ruma::events::room::message::RoomMessageEventContent;

pub const ROBRIX_UPSTREAM_COMMIT: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";
pub const INTAKE_STATUS: &str = "adapted_parser_ready_not_wired";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlashCommand {
    /// The command name without the leading slash, for example `html`.
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
}

/// Commands supported by both the upstream parser and the existing Hepta composer semantics.
pub static SLASH_COMMANDS: &[SlashCommand] = &[
    SlashCommand {
        name: "html",
        description: "Send the message as raw HTML",
        usage: "/html <message>",
    },
    SlashCommand {
        name: "plain",
        description: "Send as plain text, without Markdown formatting",
        usage: "/plain <message>",
    },
];

/// Returns all commands whose names start with `query`, ignoring ASCII case.
pub fn matching_commands(query: &str) -> impl Iterator<Item = &'static SlashCommand> {
    let query = query.to_ascii_lowercase();
    SLASH_COMMANDS
        .iter()
        .filter(move |command| command.name.starts_with(&query))
}

/// Builds Matrix message content for a supported command without sending it.
pub fn build_message_for_command(text: &str) -> Option<RoomMessageEventContent> {
    let (name, argument) = split_command(text)?;
    match name {
        "html" => Some(RoomMessageEventContent::text_html(
            html_to_plaintext(argument),
            argument,
        )),
        "plain" => Some(RoomMessageEventContent::text_plain(argument)),
        _ => None,
    }
}

fn split_command(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix('/')?;
    Some(match rest.split_once(char::is_whitespace) {
        Some((name, argument)) => (name, argument),
        None => (rest, ""),
    })
}

/// Builds a plaintext fallback for clients that cannot render formatted HTML.
fn html_to_plaintext(html: &str) -> String {
    let mut output = String::with_capacity(html.len());
    let mut in_tag = false;
    for character in html.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(character),
            _ => {}
        }
    }
    decode_basic_html_entities(&output)
}

fn decode_basic_html_entities(text: &str) -> String {
    text.replace("&nbsp;", "\u{a0}")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robrix_intake_command_matching_is_case_insensitive_and_prefix_based() {
        let matches = matching_commands("H")
            .map(|command| command.name)
            .collect::<Vec<_>>();
        assert_eq!(matches, vec!["html"]);
    }

    #[test]
    fn robrix_intake_unsupported_or_non_command_text_is_not_claimed() {
        assert!(build_message_for_command("hello").is_none());
        assert!(build_message_for_command("/unknown body").is_none());
    }

    #[test]
    fn robrix_intake_html_fallback_strips_tags_and_decodes_entities() {
        assert_eq!(html_to_plaintext("<strong>A &amp; B</strong>"), "A & B");
    }
}

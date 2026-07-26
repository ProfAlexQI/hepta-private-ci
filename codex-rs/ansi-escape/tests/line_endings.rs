use codex_ansi_escape::ansi_escape;
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;

#[test]
fn crlf_and_bare_cr_split_lines_without_losing_style() {
    assert_eq!(
        ansi_escape("A\x1b[31mB\r\nC\rD"),
        Text::from(vec![
            Line::from(vec![Span::raw("A"), "B".red()]),
            Line::from("C".red()),
            Line::from("D".red()),
        ])
    );
}

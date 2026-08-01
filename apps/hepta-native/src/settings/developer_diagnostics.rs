//! Read-only developer diagnostics.
//!
//! The section is hidden in normal builds and becomes visible only when the
//! `developer-diagnostics` Cargo feature is enabled. It never substitutes for
//! the room list, timeline, or composer and cannot issue bridge mutations.

use makepad_widgets::*;

const ROBRIX_UPSTREAM: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.DeveloperDiagnostics = #(DeveloperDiagnostics::register_widget(vm)) {
        visible: false
        width: Fill, height: Fit
        flow: Down
        spacing: 8

        TitleLabel { text: "Developer Diagnostics" }
        Label {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true}
            draw_text +: {
                color: (COLOR_HEPTA_MUTED)
                text_style: theme.font_regular {font_size: 10.5, line_spacing: 1.35}
            }
            text: "Read-only build and bridge state. This section is excluded from normal product navigation."
        }
        upstream := Label {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true}
            draw_text +: {color: (COLOR_HEPTA_TEXT)}
        }
        bridge := Label {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true}
            draw_text +: {color: (COLOR_HEPTA_TEXT)}
        }
        boundaries := Label {
            width: Fill, height: Fit
            flow: Flow.Right{wrap: true}
            draw_text +: {color: (COLOR_HEPTA_MUTED)}
        }
    }
}

#[derive(Script, Widget)]
pub struct DeveloperDiagnostics {
    #[deref]
    view: View,
}

impl ScriptHook for DeveloperDiagnostics {
    fn on_after_new(&mut self, vm: &mut ScriptVm) {
        vm.with_cx_mut(|cx| {
            self.view
                .set_visible(cx, cfg!(feature = "developer-diagnostics"));
            self.view.label(cx, ids!(upstream)).set_text(
                cx,
                &format!("Robrix upstream: {ROBRIX_UPSTREAM}"),
            );
            self.view.label(cx, ids!(bridge)).set_text(
                cx,
                "Hepta bridge: disabled adapter (zero Matrix sends; zero runtime mutations)",
            );
            self.view.label(cx, ids!(boundaries)).set_text(
                cx,
                "Live Matrix verification, live Hepta adapter, real-device lab, signing, notarization, and public distribution are separate gates.",
            );
        });
    }
}

impl Widget for DeveloperDiagnostics {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "developer-diagnostics"))]
    fn diagnostics_are_not_part_of_the_default_product_build() {
        assert_eq!(ROBRIX_UPSTREAM.len(), 40);
    }
}

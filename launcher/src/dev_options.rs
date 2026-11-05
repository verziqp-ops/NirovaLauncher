//! Developer information shown after seven clicks on the launcher version.
//! Local Guest profiles are launcher identities only; they never grant Minecraft ownership.
use super::*;

const CLICKS: u8 = 7;

impl Launcher {
    /// A click on the launcher version; the last one reveals the developer information.
    pub(super) fn version_clicked(&mut self) {
        if self.dev_options {
            return;
        }
        self.dev_clicks += 1;
        if self.dev_clicks >= CLICKS {
            self.dev_options = true;
            self.go(Page::Settings);
        }
    }

    /// No hidden ownership bypasses: the developer section explains the Guest profile boundary.
    pub(super) fn dev_options_section(&self, _cx: &mut Context<Self>) -> Option<Div> {
        if !self.dev_options {
            return None;
        }
        let t = self.theme;
        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    section_title("Developer information", "terminal", &t)
                        .mt(px(20.))
                        .mb(px(12.)),
                )
                .child(index_row(
                    0,
                    tr("Guest profile"),
                    Some(
                        div()
                            .child(tr(
                                "A Guest profile is a local launcher name. Starting Minecraft still requires a Microsoft account with Minecraft: Java Edition ownership.",
                            ))
                            .into_any_element(),
                    ),
                    None,
                    &t,
                )),
        )
    }
}

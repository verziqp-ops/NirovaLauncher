//! Settings' accent colour and Home background rows (`horizon_launcher::themes`).
use super::*;
use gpui::{
    canvas, fill, point, size, Bounds, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point,
};
use horizon_launcher::themes::{self, Accent};
use std::path::PathBuf;

/// Each accent's swatch, in points.
const SWATCH: f32 = 26.;
/// The hue strip beside the swatches, in points: 0 to 360 degrees across it.
const STRIP: f32 = 120.;

impl Launcher {
    /// The theme Settings ask for, under the window's own light or dark.
    pub(super) fn settings_theme(&self, window: &Window) -> Theme {
        let mut theme = Theme::for_appearance(
            self.settings.appearance,
            self.settings.accent,
            window.appearance(),
        );
        let opacity = self.settings.panel_opacity.unwrap_or(75).clamp(35, 100) as f32 / 100.;
        theme.night.card_alpha = opacity;
        theme.night.card_hi_alpha = opacity;
        theme.panel.a = opacity;
        theme.surface.a = opacity;
        theme
    }

    pub(super) fn appearance_tiles(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let mut tiles = div().flex().gap(px(18.));
        for (choice, label) in [
            (backend::Appearance::Dark, "Dark"),
            (backend::Appearance::Light, "Light"),
            (backend::Appearance::System, "System"),
        ] {
            let selected = self.settings.appearance == choice;
            let mut art = div().relative().size(px(100.)).overflow_hidden();
            for y in 0..20 {
                for x in 0..20 {
                    let dark = choice == backend::Appearance::Dark
                        || (choice == backend::Appearance::System && x + y < 20);
                    let color = if y >= 16 {
                        if (x + y) % 3 == 0 {
                            0x705037
                        } else {
                            0x876044
                        }
                    } else if y >= 14 {
                        if y == 14 {
                            0x68ac47
                        } else {
                            0x4e8038
                        }
                    } else if dark && [(4, 3), (13, 4), (9, 8), (16, 10), (3, 11)].contains(&(x, y))
                    {
                        0xdce9ff
                    } else if !dark
                        && ((3..9).contains(&x) && (4..6).contains(&y)
                            || (11..17).contains(&x) && (9..11).contains(&y))
                    {
                        0xe4f4ff
                    } else if dark {
                        0x141d3a
                    } else {
                        0x64baf2
                    };
                    art = art.child(
                        div()
                            .absolute()
                            .left(px(x as f32 * 5.))
                            .top(px(y as f32 * 5.))
                            .size(px(5.))
                            .bg(gpui::rgb(color)),
                    );
                }
            }
            tiles = tiles.child(
                keyboard(div().id(SharedString::from(format!("theme-{label}"))), &t)
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.))
                    .cursor_pointer()
                    .child(
                        div()
                            .p(px(4.))
                            .border_2()
                            .pixel_border(if selected { t.accent } else { t.outline })
                            .child(art),
                    )
                    .child(text(theme::UI).child(tr(label)))
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        this.settings.appearance = choice;
                        this.settings.save();
                        titlebar::apply_appearance(window, choice);
                        this.theme = this.settings_theme(window);
                        cx.notify();
                    })),
            );
        }
        tiles
    }

    pub(super) fn scenery_controls(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let mut levels = div().flex().gap(px(6.));
        for value in [35u8, 50, 75, 100] {
            levels = levels.child(
                button(
                    SharedString::from(format!("opacity-{value}")),
                    format!("{value}%"),
                    if self.settings.panel_opacity.unwrap_or(75) == value {
                        Kind::Primary
                    } else {
                        Kind::Secondary
                    },
                    &t,
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.settings.panel_opacity = Some(value);
                    this.settings.save();
                    this.theme = this.settings_theme(window);
                    cx.notify();
                })),
            );
        }
        let animated = self.settings.animate_sky.unwrap_or(true);
        div()
            .flex()
            .flex_col()
            .child(index_row(
                0,
                tr("Panel opacity"),
                None,
                Some(levels.into_any_element()),
                &t,
            ))
            .child(index_row(
                0,
                tr("Animate background"),
                None,
                Some(
                    button(
                        "animate-sky",
                        if animated { "On" } else { "Off" },
                        Kind::Secondary,
                        &t,
                    )
                    .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                        this.settings.animate_sky = Some(!animated);
                        this.settings.save();
                        cx.notify();
                    }))
                    .into_any_element(),
                ),
                &t,
            ))
    }

    /// The picture behind Home, if the player keeps one.
    pub(super) fn home_background(&self) -> Option<PathBuf> {
        themes::background(&backend::data_dir(), self.settings.background.as_deref())
    }

    pub(super) fn accent_row(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let mut swatches = div().flex().gap(px(6.));
        for accent in Accent::ALL {
            let face = Theme::for_appearance(
                if t.dark {
                    backend::Appearance::Dark
                } else {
                    backend::Appearance::Light
                },
                accent,
                gpui::WindowAppearance::Dark,
            )
            .face;
            let chosen = self.settings.accent == accent;
            swatches = swatches.child(
                keyboard(
                    div().id(SharedString::from(format!("accent-{}", accent.name()))),
                    &t,
                )
                .size(px(SWATCH))
                .pixel_bg(face)
                .border_2()
                .pixel_border(if chosen { t.ink } else { t.outline })
                .cursor_pointer()
                .when(chosen, |d| {
                    d.flex()
                        .items_center()
                        .justify_center()
                        .child(div().size(px(8.)).bg(t.on_face))
                })
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.settings.accent = accent;
                    this.settings.save();
                    this.theme = this.settings_theme(window);
                    cx.notify();
                })),
            );
        }
        index_row(
            0,
            tr("Accent colour"),
            Some(
                div()
                    .child(tr(
                        "NIROVA's rose-pink by default. Every colour keeps text easy to read.",
                    ))
                    .into_any_element(),
            ),
            Some(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .child(swatches)
                    .child(self.hue_strip(&t, cx))
                    .into_any_element(),
            ),
            &t,
        )
    }

    /// A strip of every hue: a click or a drag picks any of them, the swatches being shortcuts.
    fn hue_strip(&self, t: &Theme, cx: &mut Context<Self>) -> Div {
        let current = self.settings.accent.hue();
        let area = self.hue_area.clone();
        let ink = t.ink;
        let outline = t.outline;
        // Hues per painted quad: 60 of them, fine enough at this width.
        const STEPS: usize = 60;
        div()
            .relative()
            .flex_none()
            .w(px(STRIP))
            .h(px(SWATCH))
            .border_2()
            .pixel_border(outline)
            .cursor_pointer()
            .child(
                canvas(
                    move |bounds, _, _| {
                        area.set(Some(bounds));
                        bounds
                    },
                    move |bounds, _, window, _| {
                        let width: f32 = bounds.size.width.into();
                        let step = width / STEPS as f32;
                        for i in 0..STEPS {
                            let left = px(step * i as f32);
                            let colour: Hsla =
                                gpui::rgb(themes::hue_colour(i as f32 * 360. / STEPS as f32))
                                    .into();
                            window.paint_quad(fill(
                                Bounds::new(
                                    point(bounds.left() + left, bounds.top()),
                                    size(px(step), bounds.size.height),
                                ),
                                colour,
                            ));
                        }
                        if let Some(hue) = current {
                            let at = px(width * hue / 360.);
                            window.paint_quad(fill(
                                Bounds::new(
                                    point(bounds.left() + at - px(1.5), bounds.top()),
                                    size(px(3.), bounds.size.height),
                                ),
                                ink,
                            ));
                        }
                    },
                )
                .size_full(),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, ev: &MouseDownEvent, window, cx| {
                    this.pick_hue(ev.position, window, cx);
                }),
            )
            .on_mouse_move(cx.listener(|this, ev: &MouseMoveEvent, window, cx| {
                if ev.pressed_button == Some(MouseButton::Left) {
                    this.pick_hue(ev.position, window, cx);
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, _| this.finish_hue()),
            )
            // `on_mouse_up` fires only while the strip is hovered, so a drag released off it would
            // keep its accent in memory and never write it.
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _: &MouseUpEvent, _, _| this.finish_hue()),
            )
    }

    /// A release ends the drag and writes `settings.json` once.
    fn finish_hue(&mut self) {
        if self.hue_dragging {
            self.hue_dragging = false;
            self.settings.save();
        }
    }

    /// The hue under the pointer, as the accent. Saving waits until the mouse comes up, so a drag
    /// writes `settings.json` once.
    fn pick_hue(&mut self, at: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(area) = self.hue_area.get() else {
            return;
        };
        let width: f32 = area.size.width.into();
        if width <= 0. {
            return;
        }
        let across: f32 = (at.x - area.origin.x).into();
        self.settings.accent = Accent::custom((across / width).clamp(0., 1.) * 360.);
        self.hue_dragging = true;
        self.theme = self.settings_theme(window);
        cx.notify();
    }

    fn choose_background(&mut self, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(tr("Choose").to_string().into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(mut paths))) = paths.await else {
                return;
            };
            let Some(path) = paths.pop() else {
                return;
            };
            let _ = this.update(cx, |this, cx| {
                this.take_background(&path);
                cx.notify();
            });
        })
        .detach();
    }

    fn take_background(&mut self, path: &std::path::Path) {
        match themes::keep_background(&backend::data_dir(), path) {
            Ok(name) => {
                self.settings.background = Some(name);
                self.settings.save();
                self.background_error = None;
            }
            Err(e) => self.background_error = Some(e),
        }
    }

    pub(super) fn home_background_row(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let custom = self.home_background().is_some();
        let mut actions = div().flex().gap(px(4.)).child(
            button(
                "background-choose",
                "Choose a picture…",
                Kind::Secondary,
                &t,
            )
            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.choose_background(cx);
                cx.notify();
            })),
        );
        if custom {
            actions = actions.child(
                button("background-reset", "Landscape", Kind::Ghost, &t).on_click(cx.listener(
                    |this, _: &ClickEvent, _, cx| {
                        this.settings.background = None;
                        this.settings.save();
                        cx.notify();
                    },
                )),
            );
        }
        let said = if custom {
            tr("Your picture, behind Home, cropped to fill it.")
        } else {
            tr("NIROVA's landscape. Pick a PNG, JPEG or WebP picture of your own.")
        };
        let detail = div().flex().flex_col().child(said.to_string()).children(
            self.background_error
                .clone()
                .map(|e| div().text_color(t.danger).child(e)),
        );
        index_row(
            0,
            tr("Home background"),
            Some(detail.into_any_element()),
            Some(actions.into_any_element()),
            &t,
        )
    }
}

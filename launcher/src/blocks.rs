//! The blocky look's parts (`docs/design-system.md`, "Blocks"): pixel type, pixel icons, panels
//! with a dark outline, a light bevel on top and a darker lip below, sunk wells, badges, the
//! inventory tabs, and the slots that hold icons downloaded at run time.
use crate::plate::PixelSurface;
use crate::theme::{self, Theme};
use gpui::{
    div, img, prelude::*, px, svg, AnyElement, App, ClickEvent, Div, FontWeight, Hsla, ImageSource,
    ObjectFit, SharedString, Svg, Window,
};
use std::borrow::Cow;

/// The icons, served to `svg()` as `icons/<name>.svg`.
pub struct Assets;

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        let name = path
            .strip_prefix("icons/")
            .and_then(|p| p.strip_suffix(".svg"));
        Ok(name.and_then(horizon_theme::icons::get).map(Cow::Borrowed))
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(horizon_theme::icons::ALL
            .iter()
            .map(|(name, _)| format!("icons/{name}.svg").into())
            .collect())
    }
}

/// A pixel icon (`horizon_theme::icons`) in `color`.
pub fn icon(name: &'static str, size: f32, color: Hsla) -> Svg {
    svg()
        .path(format!("icons/{name}.svg"))
        .flex_none()
        .size(px(size))
        .text_color(color)
}

/// Text in the pixel face: headings, navigation, buttons, badges.
pub fn pixel(size: f32, weight: FontWeight) -> Div {
    div()
        .font_family(theme::font())
        .font_weight(weight)
        .text_size(px(size))
        .line_height(px((size * 1.2).round()))
}

/// The light edge along the top of a raised block and the darker lip along its bottom; the
/// block must be `relative`.
pub fn edges(top: Hsla, bottom: Hsla, lip: f32) -> [AnyElement; 2] {
    [
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .h(px(theme::BEVEL))
            .bg(top)
            .into_any_element(),
        div()
            .absolute()
            .bottom_0()
            .left_0()
            .right_0()
            .h(px(lip))
            .bg(bottom)
            .into_any_element(),
    ]
}

/// A panel: rows, cards, notices, dialogs.
pub fn panel(t: &Theme) -> Div {
    div()
        .relative()
        .pixel_bg(t.surface)
        .border_1()
        .pixel_border(t.rule)
        .rounded(px(0.))
}

/// A small label in a block of its colour: "Runs", "Partly", a version.
pub fn badge(label: impl Into<SharedString>, color: Hsla, t: &Theme) -> Div {
    let label: SharedString = label.into();
    let label = SharedString::from(crate::i18n::tr(&label).to_string());
    let tint = if t.dark { 0.14 } else { 0.12 };
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(px(6.))
        .h(px(22.))
        .px(px(10.))
        .pixel_bg(color.opacity(tint))
        .rounded(px(0.))
        .child(
            pixel(12., FontWeight::MEDIUM)
                .text_color(color)
                .child(label),
        )
}

/// Inventory tabs: a row of tabs on top of a panel, the open one joined to it.
pub fn tabs<T: Copy + PartialEq + 'static>(
    id: &'static str,
    options: &[(T, &'static str, &'static str)],
    current: T,
    t: &Theme,
    on_pick: impl Fn(T, &mut Window, &mut App) + Clone + 'static,
) -> Div {
    let t = *t;
    div()
        .flex()
        .items_center()
        .gap(px(4.))
        .p(px(4.))
        .pixel_bg(t.plate_soft)
        .rounded(px(0.))
        .children(options.iter().map(|&(value, label, icon_name)| {
            let open = current == value;
            let pick = on_pick.clone();
            let tab = div()
                .id(SharedString::from(format!("{id}-{label}")))
                .relative()
                .flex()
                .items_center()
                .gap(px(8.))
                .h(px(32.))
                .px(px(12.))
                .rounded(px(0.))
                .cursor_pointer()
                .when(open, |d| d.pixel_bg(t.ink))
                .child(icon(icon_name, 16., if open { t.page } else { t.muted }))
                .child(
                    pixel(14., FontWeight::MEDIUM)
                        .text_color(if open { t.page } else { t.ink_soft })
                        .child(crate::i18n::tr(label)),
                )
                .when(!open, |d| d.hover(move |s| s.opacity(0.9)))
                .on_click(move |_: &ClickEvent, window, cx| pick(value, window, cx));
            crate::keyboard(tab, &t)
        }))
}

/// A square slot for an icon: the picture at `source` when there is one, else the first letter of
/// `name` on a raised block.
pub fn slot(source: Option<ImageSource>, name: &str, size: f32, t: &Theme) -> Div {
    let frame = div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .overflow_hidden()
        .rounded(px(0.))
        .pixel_bg(t.plate_soft);
    match source {
        Some(source) => frame.child(img(source).size(px(size - 4.)).object_fit(ObjectFit::Cover)),
        None => frame.pixel_bg(t.plate).child(
            pixel((size * 0.42).round(), FontWeight::BOLD)
                .text_color(t.muted)
                .child(
                    name.chars()
                        .find(|c| c.is_alphanumeric())
                        .unwrap_or('?')
                        .to_uppercase()
                        .to_string(),
                ),
        ),
    }
}

use gpui::{
    div, point, prelude::*, px, rgba, size, App, Application, Bounds, Context, SharedString, Timer,
    TitlebarOptions, Window, WindowBounds, WindowOptions,
};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::liquid_glass::GlassVariant;

const WINDOW_SIZE: gpui::Size<gpui::Pixels> = size(px(920.0), px(620.0));
// The NSWindow owns the full-window silhouette. Adding a second radius to the
// glass view produces a visible inset arc at each native window corner.
const WINDOW_GLASS_CORNER_RADIUS: gpui::Pixels = px(0.0);

pub struct WindowGlassDemo {
    variant: GlassVariant,
    render_count: Arc<AtomicUsize>,
}

impl WindowGlassDemo {
    fn new(variant: GlassVariant, render_count: Arc<AtomicUsize>) -> Self {
        Self {
            variant,
            render_count,
        }
    }

    fn select_variant(
        &mut self,
        variant: GlassVariant,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.variant == variant {
            return;
        }
        self.variant = variant;
        window.set_background_appearance(variant.window_background(WINDOW_GLASS_CORNER_RADIUS));
        window.set_window_title(&format!("Window Glass - {}", variant.title()));
        cx.notify();
    }

    fn material_button(&self, variant: GlassVariant, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.variant == variant;
        div()
            .id(SharedString::from(format!("window-glass-{:?}", variant)))
            .h(px(34.0))
            .px_3()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(6.0))
            .cursor_pointer()
            .text_size(px(13.0))
            .text_color(rgba(if selected { 0xffffffff } else { 0xffffffb8 }))
            .when(selected, |button| button.bg(rgba(0xffffff24)))
            .hover(|button| button.bg(rgba(0xffffff18)))
            .active(|button| button.opacity(0.72))
            .child(variant.title())
            .on_click(cx.listener(move |this, _, window, cx| {
                this.select_variant(variant, window, cx);
            }))
    }
}

impl Render for WindowGlassDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_count.fetch_add(1, Ordering::Relaxed);
        let tint_swatch = matches!(
            self.variant,
            GlassVariant::RegularTinted | GlassVariant::ClearTinted
        );
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .text_color(rgba(0xffffffff))
            .child(
                div()
                    .absolute()
                    .top(px(16.0))
                    .left(px(84.0))
                    .right(px(24.0))
                    .h(px(30.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .text_size(px(13.0))
                    .child("Window Glass")
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_color(rgba(0xffffffb8))
                            .when(tint_swatch, |row| {
                                row.child(
                                    div()
                                        .size(px(10.0))
                                        .rounded_full()
                                        .bg(rgba(0xff3847ff)),
                                )
                            })
                            .child(self.variant.title()),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .left(px(48.0))
                    .right(px(48.0))
                    .top(px(112.0))
                    .flex()
                    .flex_col()
                    .gap_5()
                    .child(
                        div()
                            .text_size(px(34.0))
                            .child("The window is the material."),
                    )
                    .child(
                        div()
                            .max_w(px(610.0))
                            .text_size(px(16.0))
                            .text_color(rgba(0xffffffc7))
                            .child(
                                "Move this window across photos, video, or another app. The native compositor keeps the glass live while GPUI content stays sharp.",
                            ),
                    )
                    .child(
                        div()
                            .mt_8()
                            .flex()
                            .gap_3()
                            .child(metric("Source", "Behind window"))
                            .child(metric("Rendering", "Native AppKit"))
                            .child(metric("Scene copies", "Zero")),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .left(px(48.0))
                    .right(px(48.0))
                    .bottom(px(34.0))
                    .p_1()
                    .flex()
                    .items_center()
                    .gap_1()
                    .rounded(px(8.0))
                    .bg(rgba(0x00000024))
                    .children(
                        GlassVariant::ALL
                            .into_iter()
                            .map(|variant| self.material_button(variant, cx)),
                    ),
            )
    }
}

fn metric(label: &'static str, value: &'static str) -> impl IntoElement {
    div()
        .w(px(174.0))
        .p_3()
        .flex()
        .flex_col()
        .gap_1()
        .rounded(px(8.0))
        .bg(rgba(0x00000020))
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgba(0xffffff91))
                .child(label),
        )
        .child(div().text_size(px(15.0)).child(value))
}

pub fn launch_window_glass() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let variant = GlassVariant::from_args(&args);
    let background_run = args.iter().any(|argument| argument == "background-run");
    let benchmark = args.iter().any(|argument| argument == "benchmark");

    Application::new().run(move |cx: &mut App| {
        cx.activate(!background_run);
        let render_count = Arc::new(AtomicUsize::new(0));
        let bounds = Bounds::centered(None, WINDOW_SIZE, cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(format!("Window Glass - {}", variant.title()).into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(14.0), px(14.0))),
                }),
                window_background: variant.window_background(WINDOW_GLASS_CORNER_RADIUS),
                window_min_size: Some(size(px(640.0), px(440.0))),
                focus: !background_run,
                show: true,
                ..Default::default()
            },
            {
                let render_count = render_count.clone();
                move |_window, cx| cx.new(|_| WindowGlassDemo::new(variant, render_count.clone()))
            },
        )
        .expect("open Window Glass demo");

        if benchmark {
            cx.spawn(async move |cx| {
                Timer::after(Duration::from_secs(6)).await;
                eprintln!(
                    "window glass renders: count={}",
                    render_count.load(Ordering::Relaxed)
                );
                cx.update(|cx| cx.quit()).ok();
            })
            .detach();
        }
    });
}

#[cfg(test)]
mod tests {
    use gpui::{WindowBackgroundAppearance, WindowGlassStyle};

    use super::*;

    #[test]
    fn every_component_variant_maps_to_a_window_material() {
        for variant in GlassVariant::ALL {
            let appearance = variant.window_background(WINDOW_GLASS_CORNER_RADIUS);
            match variant {
                GlassVariant::Identity => {
                    assert_eq!(appearance, WindowBackgroundAppearance::Transparent);
                }
                GlassVariant::Regular | GlassVariant::RegularTinted => match appearance {
                    WindowBackgroundAppearance::LiquidGlass(glass) => {
                        assert_eq!(glass.style, WindowGlassStyle::Regular);
                        assert_eq!(glass.tint.is_some(), variant == GlassVariant::RegularTinted);
                        assert_eq!(glass.corner_radius, gpui::Pixels::ZERO);
                    }
                    _ => panic!("expected native Regular window glass"),
                },
                GlassVariant::Clear | GlassVariant::ClearTinted => match appearance {
                    WindowBackgroundAppearance::LiquidGlass(glass) => {
                        assert_eq!(glass.style, WindowGlassStyle::Clear);
                        assert_eq!(glass.tint.is_some(), variant == GlassVariant::ClearTinted);
                        assert_eq!(glass.corner_radius, gpui::Pixels::ZERO);
                    }
                    _ => panic!("expected native Clear window glass"),
                },
            }
        }
    }
}

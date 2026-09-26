use std::io::Cursor;
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{
    canvas, div, enable_liquid_glass, img, point, prelude::*, px, rgb, rgba, size, Animation,
    AnimationExt, App, AppContext, Application, Bounds, Context, ImageSource, LiquidGlassVariant,
    MouseButton, MouseDownEvent, MouseUpEvent, ObjectFit, PaintLiquidGlass, PathBuilder,
    RenderImage, StyledImage, TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds,
    WindowGlassAppearance, WindowOptions,
};
use image::{Frame, Rgba, RgbaImage};

const WINDOW_W: f32 = 1200.0;
const WINDOW_H: f32 = 800.0;
const GLASS_W: f32 = 440.0;
const GLASS_H: f32 = 96.0;
const GLASS_X: f32 = (WINDOW_W - GLASS_W) * 0.5;
const GLASS_Y: f32 = WINDOW_H - 102.0 - GLASS_H;
const CORNER_RADIUS: f32 = 34.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassGeometry {
    pub canvas_width: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub corner_radius: f32,
}

impl GlassGeometry {
    pub const fn reference_player() -> Self {
        Self {
            canvas_width: WINDOW_W,
            x: GLASS_X,
            y: GLASS_Y,
            width: GLASS_W,
            height: GLASS_H,
            corner_radius: CORNER_RADIUS,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlassVariant {
    Regular,
    Clear,
    RegularTinted,
    ClearTinted,
    Identity,
}

impl GlassVariant {
    pub const ALL: [Self; 5] = [
        Self::Regular,
        Self::Clear,
        Self::RegularTinted,
        Self::ClearTinted,
        Self::Identity,
    ];

    pub fn from_args(args: &[String]) -> Self {
        for argument in args {
            match argument.as_str() {
                "regular" => return Self::Regular,
                "clear" => return Self::Clear,
                "regular-tinted" => return Self::RegularTinted,
                "clear-tinted" => return Self::ClearTinted,
                "identity" => return Self::Identity,
                _ => {}
            }
        }
        Self::Regular
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::Regular => "Regular",
            Self::Clear => "Clear",
            Self::RegularTinted => "Regular Tinted",
            Self::ClearTinted => "Clear Tinted",
            Self::Identity => "Identity",
        }
    }

    fn is_clear(self) -> bool {
        matches!(self, Self::Clear | Self::ClearTinted)
    }

    fn is_tinted(self) -> bool {
        matches!(self, Self::RegularTinted | Self::ClearTinted)
    }

    pub const fn shader_variant(self) -> LiquidGlassVariant {
        match self {
            Self::Regular => LiquidGlassVariant::Regular,
            Self::Clear => LiquidGlassVariant::Clear,
            Self::RegularTinted => LiquidGlassVariant::RegularTinted,
            Self::ClearTinted => LiquidGlassVariant::ClearTinted,
            Self::Identity => LiquidGlassVariant::Identity,
        }
    }

    /// Maps the component material to its native full-window equivalent.
    pub const fn window_background(
        self,
        corner_radius: gpui::Pixels,
    ) -> WindowBackgroundAppearance {
        let coral = gpui::Rgba {
            r: 1.0,
            g: 0.22,
            b: 0.28,
            a: 1.0,
        };
        match self {
            Self::Regular => WindowBackgroundAppearance::LiquidGlass(
                WindowGlassAppearance::regular().corner_radius(corner_radius),
            ),
            Self::Clear => WindowBackgroundAppearance::LiquidGlass(
                WindowGlassAppearance::clear().corner_radius(corner_radius),
            ),
            Self::RegularTinted => WindowBackgroundAppearance::LiquidGlass(
                WindowGlassAppearance::regular()
                    .tint(coral)
                    .corner_radius(corner_radius),
            ),
            Self::ClearTinted => WindowBackgroundAppearance::LiquidGlass(
                WindowGlassAppearance::clear()
                    .tint(coral)
                    .corner_radius(corner_radius),
            ),
            Self::Identity => WindowBackgroundAppearance::Transparent,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DemoBackground {
    Harbour,
    CityNight,
    Prism,
    Facade,
}

impl DemoBackground {
    fn from_args(args: &[String]) -> Self {
        for argument in args {
            match argument.as_str() {
                "harbour" => return Self::Harbour,
                "city-night" => return Self::CityNight,
                "prism" => return Self::Prism,
                "facade" => return Self::Facade,
                _ => {}
            }
        }
        Self::Harbour
    }

    fn title(self) -> &'static str {
        match self {
            Self::Harbour => "Harbour",
            Self::CityNight => "City Night",
            Self::Prism => "Prism",
            Self::Facade => "Facade",
        }
    }

    fn bytes(self) -> &'static [u8] {
        match self {
            Self::Harbour => include_bytes!("../../validation/shared/harbour.png"),
            Self::CityNight => include_bytes!("../../validation/shared/city-night.png"),
            Self::Prism => include_bytes!("../../validation/shared/prism.png"),
            Self::Facade => include_bytes!("../../validation/shared/facade.png"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteractionState {
    Rest,
    Hover,
    Pressed,
}

#[derive(Clone, Debug)]
pub struct LiquidGlassFrames {
    pub rest: RgbaImage,
    pub hover: RgbaImage,
    pub pressed: RgbaImage,
}

impl InteractionState {
    fn from_args(args: &[String]) -> Option<Self> {
        for argument in args {
            match argument.as_str() {
                "rest" => return Some(Self::Rest),
                "hover" => return Some(Self::Hover),
                "pressed" => return Some(Self::Pressed),
                _ => {}
            }
        }
        None
    }

    pub const fn energy(self) -> f32 {
        match self {
            Self::Rest => 0.0,
            Self::Hover => 0.48,
            Self::Pressed => 1.0,
        }
    }
}

struct Demo {
    backdrop: Arc<RenderImage>,
    variant: GlassVariant,
    interaction: InteractionState,
    interaction_locked: bool,
    dynamic_scene: bool,
    no_glass: bool,
    benchmark: bool,
    stress_count: usize,
    last_frame: Option<Instant>,
    frame_intervals: Vec<Duration>,
}

impl Demo {
    fn new(
        variant: GlassVariant,
        forced_interaction: Option<InteractionState>,
        background: DemoBackground,
        dynamic_scene: bool,
        no_glass: bool,
        benchmark: bool,
        stress_count: usize,
    ) -> Self {
        let source = image::load(Cursor::new(background.bytes()), image::ImageFormat::Png)
            .expect("decode shared test backdrop")
            .into_rgba8();
        Self {
            backdrop: render_image(source),
            variant,
            interaction: forced_interaction.unwrap_or(InteractionState::Rest),
            interaction_locked: forced_interaction.is_some(),
            dynamic_scene,
            no_glass,
            benchmark,
            stress_count,
            last_frame: None,
            frame_intervals: Vec::with_capacity(600),
        }
    }

    fn on_mouse_down(&mut self, _: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.variant != GlassVariant::Identity && !self.interaction_locked {
            self.interaction = InteractionState::Pressed;
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.variant != GlassVariant::Identity && !self.interaction_locked {
            self.interaction = InteractionState::Hover;
            cx.notify();
        }
    }
}

impl Render for Demo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.benchmark {
            let now = Instant::now();
            if let Some(last) = self.last_frame.replace(now) {
                if self.frame_intervals.len() >= 60 {
                    self.frame_intervals.push(now.duration_since(last));
                } else {
                    self.frame_intervals.push(Duration::ZERO);
                }
            }
            if self.frame_intervals.len() == 540 {
                report_dynamic_frame_intervals(&self.frame_intervals[60..]);
                cx.quit();
            }
        }

        let has_material = self.variant != GlassVariant::Identity && !self.no_glass;
        let variant = self.variant;
        let interaction = self.interaction;
        let stress_count = self.stress_count;
        let playback_control = div()
            .id("liquid-glass-control")
            .absolute()
            .left(px(GLASS_X))
            .top(px(GLASS_Y))
            .w(px(GLASS_W))
            .h(px(GLASS_H))
            .rounded(px(CORNER_RADIUS))
            .cursor_pointer()
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _| {
                        if has_material {
                            window.paint_liquid_glass(PaintLiquidGlass {
                                bounds,
                                corner_radius: px(CORNER_RADIUS),
                                variant: variant.shader_variant(),
                                interaction: interaction.energy(),
                            });
                        }
                    },
                )
                .size_full()
                .absolute()
                .inset_0(),
            )
            .child(playback_contents())
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                if this.variant == GlassVariant::Identity
                    || this.interaction_locked
                    || this.interaction == InteractionState::Pressed
                {
                    return;
                }
                let next = if *hovered {
                    InteractionState::Hover
                } else {
                    InteractionState::Rest
                };
                if this.interaction != next {
                    this.interaction = next;
                    cx.notify();
                }
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    if this.variant != GlassVariant::Identity && !this.interaction_locked {
                        this.interaction = InteractionState::Rest;
                        cx.notify();
                    }
                }),
            );
        div()
            .relative()
            .size_full()
            .overflow_hidden()
            .bg(rgb(0x0b4265))
            .child(
                img(ImageSource::Render(self.backdrop.clone()))
                    .absolute()
                    .inset_0()
                    .size_full()
                    .object_fit(ObjectFit::Cover),
            )
            .when(self.dynamic_scene, |element| {
                element.child(animated_scene_overlay())
            })
            .when(stress_count > 0, |element| {
                element
                    .child(stress_scene_overlay())
                    .child(stress_glass_grid(stress_count, variant, has_material))
            })
            .when(stress_count == 0, |element| element.child(playback_control))
    }
}

fn stress_scene_overlay() -> impl IntoElement {
    let mut grid = div()
        .absolute()
        .w(px(1440.0))
        .h(px(960.0))
        .flex()
        .flex_wrap();
    let colors = [0xffffff28, 0x00000024, 0xff453a38, 0x64d2ff38];
    for index in 0..2400 {
        grid = grid.child(
            div()
                .w(px(24.0))
                .h(px(24.0))
                .bg(rgba(colors[(index + index / 60) % colors.len()])),
        );
    }
    grid.with_animation(
        "stress-backdrop",
        Animation::new(Duration::from_secs_f32(0.9)).repeat(),
        |element, delta| {
            let phase = delta * std::f32::consts::TAU;
            element
                .left(px(-120.0 + phase.sin() * 96.0))
                .top(px(-80.0 + phase.cos() * 64.0))
        },
    )
}

fn stress_glass_grid(count: usize, variant: GlassVariant, has_material: bool) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |canvas_bounds, _, window, _| {
            if !has_material {
                return;
            }
            window.paint_layer(canvas_bounds, |window| {
                let columns = 4;
                let width = 250.0;
                let height = 96.0;
                let gap_x = 40.0;
                let gap_y = 28.0;
                let visible_slots = count.min(24);
                let rows = visible_slots.div_ceil(columns);
                let total_width = columns as f32 * width + (columns - 1) as f32 * gap_x;
                let total_height = rows as f32 * height + rows.saturating_sub(1) as f32 * gap_y;
                let start_x = (WINDOW_W - total_width) * 0.5;
                let start_y = (WINDOW_H - total_height) * 0.5;
                for index in 0..count {
                    let slot = index % 24;
                    let column = slot % columns;
                    let row = slot / columns;
                    window.paint_liquid_glass(PaintLiquidGlass {
                        bounds: Bounds {
                            origin: point(
                                px(start_x + column as f32 * (width + gap_x)),
                                px(start_y + row as f32 * (height + gap_y)),
                            ),
                            size: size(px(width), px(height)),
                        },
                        corner_radius: px(24.0),
                        variant: variant.shader_variant(),
                        interaction: 0.0,
                    });
                }
            });
        },
    )
    .absolute()
    .inset_0()
    .size_full()
}

fn animated_scene_overlay() -> impl IntoElement {
    let colors = [
        0xff3b30aa, 0xffcc00a8, 0x34c759a8, 0x00c7bea8, 0x0a84ffa8, 0xaf52dea8,
    ];
    let mut band = div()
        .absolute()
        .top_0()
        .h_full()
        .w(px(1440.0))
        .flex()
        .opacity(0.48);
    for color in colors {
        band = band.child(div().h_full().w(px(240.0)).bg(rgba(color)));
    }
    band.with_animation(
        "dynamic-backdrop",
        Animation::new(Duration::from_secs_f32(1.35)).repeat(),
        |element, delta| {
            let x = -120.0 + (delta * std::f32::consts::TAU).sin() * 120.0;
            element.left(px(x))
        },
    )
}

fn report_dynamic_frame_intervals(samples: &[Duration]) {
    let mut milliseconds = samples
        .iter()
        .map(|duration| duration.as_secs_f64() * 1_000.0)
        .collect::<Vec<_>>();
    milliseconds.sort_by(f64::total_cmp);
    let median = milliseconds[milliseconds.len() / 2];
    let p95 = milliseconds[milliseconds.len() * 95 / 100];
    let dropped = milliseconds.iter().filter(|sample| **sample > 12.5).count();
    println!(
        "dynamic frames: count={} median={median:.3} ms p95={p95:.3} ms over-12.5ms={dropped}",
        milliseconds.len()
    );
}

fn playback_contents() -> impl IntoElement {
    div()
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .px(px(20.0))
        .text_color(rgb(0xffffff))
        .child(
            div()
                .relative()
                .flex_none()
                .w(px(56.0))
                .h(px(56.0))
                .rounded_full()
                .bg(rgba(0xffffff2e))
                .child(
                    canvas(
                        |_, _, _| {},
                        |bounds, _, window, _| {
                            let mut path = PathBuilder::fill();
                            path.move_to(bounds.origin);
                            path.line_to(point(
                                bounds.origin.x + bounds.size.width,
                                bounds.origin.y + bounds.size.height * 0.5,
                            ));
                            path.line_to(point(
                                bounds.origin.x,
                                bounds.origin.y + bounds.size.height,
                            ));
                            path.close();
                            window.paint_path(
                                path.build().expect("valid play triangle"),
                                gpui::white(),
                            );
                        },
                    )
                    .absolute()
                    .left(px(20.0))
                    .top(px(17.0))
                    .w(px(18.0))
                    .h(px(22.0)),
                ),
        )
        .child(
            div()
                .ml(px(16.0))
                .flex()
                .flex_col()
                .gap(px(5.0))
                .child(
                    div()
                        .text_size(px(17.0))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .line_height(px(20.0))
                        .child("Glass Horizon"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .line_height(px(16.0))
                        .text_color(rgba(0xffffffb8))
                        .child("Harbour Sessions"),
                ),
        )
        .child(div().flex_1())
        .child(equalizer())
        .child(
            div()
                .ml(px(16.0))
                .text_size(px(13.0))
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(rgba(0xffffffe0))
                .child("3:42"),
        )
}

fn equalizer() -> impl IntoElement {
    let heights = [10.0, 20.0, 15.0, 25.0];
    let mut bars = div()
        .flex()
        .items_center()
        .gap(px(3.0))
        .w(px(28.0))
        .h(px(25.0));
    for height in heights {
        bars = bars.child(
            div()
                .w(px(4.0))
                .h(px(height))
                .rounded_full()
                .bg(rgba(0xffffffe5)),
        );
    }
    bars
}

pub fn render_liquid_glass(
    source: &RgbaImage,
    variant: GlassVariant,
    interaction: InteractionState,
) -> RgbaImage {
    render_liquid_glass_in_rect(
        source,
        GlassGeometry::reference_player(),
        variant,
        interaction,
    )
}

pub fn render_liquid_glass_in_rect(
    source: &RgbaImage,
    geometry: GlassGeometry,
    variant: GlassVariant,
    interaction: InteractionState,
) -> RgbaImage {
    let metrics = glass_metrics(source, geometry);
    if variant == GlassVariant::Identity {
        return RgbaImage::new(metrics.width, metrics.height);
    }
    let prepared = prepare_liquid_glass(source, geometry, variant, metrics);
    render_prepared_states(&prepared, variant, [interaction])
        .into_iter()
        .next()
        .expect("one requested interaction frame")
}

pub fn render_liquid_glass_frames(source: &RgbaImage, variant: GlassVariant) -> LiquidGlassFrames {
    render_liquid_glass_frames_in_rect(source, GlassGeometry::reference_player(), variant)
}

pub fn render_liquid_glass_frames_in_rect(
    source: &RgbaImage,
    geometry: GlassGeometry,
    variant: GlassVariant,
) -> LiquidGlassFrames {
    let metrics = glass_metrics(source, geometry);
    if variant == GlassVariant::Identity {
        return LiquidGlassFrames {
            rest: RgbaImage::new(metrics.width, metrics.height),
            hover: RgbaImage::new(metrics.width, metrics.height),
            pressed: RgbaImage::new(metrics.width, metrics.height),
        };
    }
    let prepared = prepare_liquid_glass(source, geometry, variant, metrics);
    let [rest, hover, pressed] = render_prepared_states(
        &prepared,
        variant,
        [
            InteractionState::Rest,
            InteractionState::Hover,
            InteractionState::Pressed,
        ],
    );
    LiquidGlassFrames {
        rest,
        hover,
        pressed,
    }
}

#[derive(Clone, Copy)]
struct GlassMetrics {
    source_scale: f32,
    width: u32,
    height: u32,
    radius: f32,
}

struct PreparedGlass {
    metrics: GlassMetrics,
    clear: bool,
    bevel: f32,
    body_tint: [f32; 3],
    softened: RgbaImage,
}

fn glass_metrics(source: &RgbaImage, geometry: GlassGeometry) -> GlassMetrics {
    assert!(geometry.canvas_width > 0.0, "canvas width must be positive");
    assert!(geometry.width > 0.0, "glass width must be positive");
    assert!(geometry.height > 0.0, "glass height must be positive");
    assert!(geometry.x >= 0.0, "glass x must not be negative");
    assert!(geometry.y >= 0.0, "glass y must not be negative");
    let source_scale = source.width() as f32 / geometry.canvas_width;
    assert!(
        (geometry.x + geometry.width) * source_scale <= source.width() as f32 + 0.5,
        "glass bounds exceed the source width"
    );
    assert!(
        (geometry.y + geometry.height) * source_scale <= source.height() as f32 + 0.5,
        "glass bounds exceed the source height"
    );
    GlassMetrics {
        source_scale,
        width: (geometry.width * source_scale).round().max(1.0) as u32,
        height: (geometry.height * source_scale).round().max(1.0) as u32,
        radius: (geometry.corner_radius * source_scale).clamp(
            0.0,
            geometry.width.min(geometry.height) * source_scale * 0.5,
        ),
    }
}

fn prepare_liquid_glass(
    source: &RgbaImage,
    geometry: GlassGeometry,
    variant: GlassVariant,
    metrics: GlassMetrics,
) -> PreparedGlass {
    let GlassMetrics {
        source_scale,
        width,
        height,
        radius,
    } = metrics;
    let clear = variant.is_clear();
    let bevel = (if clear { 16.0 } else { 15.0 }) * source_scale;
    let warmth = if clear {
        0.0
    } else {
        let saturation_response =
            ((average_chroma(source, geometry) - 10.0) / 46.0).clamp(0.0, 1.0);
        0.55 + 0.45 * saturation_response
    };
    let body_tint = [255.0, 255.0 - 17.0 * warmth, 255.0 - 42.0 * warmth];
    let mut refracted = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let local_x = x as f32 + 0.5;
            let local_y = y as f32 + 0.5;
            let (distance, normal) = rounded_rect_distance_and_normal(
                local_x,
                local_y,
                width as f32,
                height as f32,
                radius,
            );
            let edge = if distance >= 0.0 {
                (1.0 - distance / bevel).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let lens = edge * edge * (3.0 - 2.0 * edge);
            let displacement = (if clear { 26.0 } else { 3.2 }) * source_scale * lens;
            let base_x = geometry.x * source_scale + local_x;
            let base_y = geometry.y * source_scale + local_y;
            let sample_x = base_x - normal.0 * displacement;
            let sample_y = base_y - normal.1 * displacement;
            let c = bilinear(source, sample_x, sample_y);
            refracted.put_pixel(x, y, Rgba([c[0] as u8, c[1] as u8, c[2] as u8, 255]));
        }
    }

    let softened = if clear {
        refracted
    } else {
        image::imageops::blur(&refracted, 8.0 * source_scale)
    };
    PreparedGlass {
        metrics,
        clear,
        bevel,
        body_tint,
        softened,
    }
}

fn render_prepared_states<const N: usize>(
    prepared: &PreparedGlass,
    variant: GlassVariant,
    states: [InteractionState; N],
) -> [RgbaImage; N] {
    let GlassMetrics {
        width,
        height,
        radius,
        ..
    } = prepared.metrics;
    let clear = prepared.clear;
    let bevel = prepared.bevel;
    let body_tint = prepared.body_tint;
    let mut outputs = std::array::from_fn(|_| RgbaImage::new(width, height));
    for y in 0..height {
        for x in 0..width {
            let local_x = x as f32 + 0.5;
            let local_y = y as f32 + 0.5;
            let (distance, normal) = rounded_rect_distance_and_normal(
                local_x,
                local_y,
                width as f32,
                height as f32,
                radius,
            );
            if distance < 0.0 {
                continue;
            }
            let edge = (1.0 - distance / bevel).clamp(0.0, 1.0);
            let pixel = prepared.softened.get_pixel(x, y).0;
            let mut c = [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32];
            let luma = c[0] * 0.2126 + c[1] * 0.7152 + c[2] * 0.0722;
            if clear {
                c = [
                    0.724344 * c[0] - 0.171892 * c[1] + 0.127461 * c[2] + 40.8984,
                    0.069654 * c[0] + 0.514368 * c[1] + 0.096509 * c[2] + 39.1233,
                    0.067487 * c[0] - 0.101700 * c[1] + 0.723461 * c[2] + 34.4699,
                ];
            } else {
                for channel in &mut c {
                    *channel = luma + (*channel - luma) * 1.05;
                    *channel = *channel * 1.01 + 2.0;
                }
            }

            let ny = normal.1;
            let nx = normal.0;
            let top_left_light = ((-nx - ny) * 0.5 + 0.5).clamp(0.0, 1.0);
            let bright_rim = if clear {
                edge.powf(6.0) * (0.12 + 0.58 * top_left_light)
            } else {
                edge.powf(5.0) * (0.06 + 0.18 * top_left_light)
            };
            let inner_glow = edge.powf(1.7) * if clear { 0.008 } else { 0.025 };
            let dark_rim =
                edge.powf(4.0) * (1.0 - top_left_light) * if clear { 0.13 } else { 0.04 };
            let antialias = distance.min(1.0).clamp(0.0, 1.0);

            for (output, state) in outputs.iter_mut().zip(states.iter()) {
                let energy = state.energy();
                let mut shaded = c;
                let tint = (if clear { 0.0 } else { 0.56 }) + inner_glow + energy * 0.025;
                for channel in 0..3 {
                    shaded[channel] = shaded[channel] * (1.0 - tint) + body_tint[channel] * tint;
                }
                for channel in &mut shaded {
                    *channel = *channel * (1.0 - bright_rim) + 255.0 * bright_rim;
                    *channel *= 1.0 - dark_rim;
                }

                if !clear {
                    let fitted = [
                        1.677902 * shaded[0] - 0.618335 * shaded[1] - 0.180438 * shaded[2]
                            + 6.82167,
                        -0.178502 * shaded[0] + 1.112136 * shaded[1] - 0.101458 * shaded[2]
                            + 29.125513,
                        -0.077173 * shaded[0] - 0.230698 * shaded[1]
                            + 1.197948 * shaded[2]
                            + 25.698877,
                    ];
                    let fit_mix = 1.0 - edge.powf(2.0);
                    for channel in 0..3 {
                        shaded[channel] =
                            shaded[channel] * (1.0 - fit_mix) + fitted[channel] * fit_mix;
                    }
                }

                if variant.is_tinted() {
                    let tinted = if clear {
                        [
                            0.053004 * shaded[0] + 0.209435 * shaded[1] - 0.008474 * shaded[2]
                                + 176.3245,
                            0.099248 * shaded[0] + 0.323315 * shaded[1]
                                - 0.025185 * shaded[2]
                                - 50.8425,
                            0.100361 * shaded[0] + 0.345083 * shaded[1]
                                - 0.016379 * shaded[2]
                                - 22.3261,
                        ]
                    } else {
                        [
                            0.054469 * shaded[0]
                                + 0.190262 * shaded[1]
                                + 0.026954 * shaded[2]
                                + 211.7142,
                            0.178054 * shaded[0] + 0.507345 * shaded[1]
                                - 0.211269 * shaded[2]
                                - 95.3155,
                            0.155782 * shaded[0] + 0.465979 * shaded[1]
                                - 0.131520 * shaded[2]
                                - 55.3333,
                        ]
                    };
                    shaded = tinted;
                }

                // Cross-background fit to the active, undimmed Xcode reference.
                if variant == GlassVariant::Clear {
                    for channel in &mut shaded {
                        *channel = channel.clamp(0.0, 255.0);
                    }
                    shaded = [
                        1.334711 * shaded[0] + 0.214589 * shaded[1]
                            - 0.281378 * shaded[2]
                            - 0.049184 * 255.0,
                        -0.147284 * shaded[0] + 1.629309 * shaded[1]
                            - 0.224290 * shaded[2]
                            - 0.039991 * 255.0,
                        -0.196570 * shaded[0] + 0.115755 * shaded[1] + 1.324347 * shaded[2]
                            - 0.012006 * 255.0,
                    ];
                } else if variant == GlassVariant::ClearTinted {
                    for channel in &mut shaded {
                        *channel = channel.clamp(0.0, 255.0);
                    }
                    shaded = [
                        1.472903 * shaded[0]
                            - 0.258041 * shaded[1]
                            - 0.097548 * shaded[2]
                            - 0.138594 * 255.0,
                        1.510637 * shaded[0] + 0.643022 * shaded[1]
                            - 0.306962 * shaded[2]
                            - 1.070967 * 255.0,
                        1.930113 * shaded[0] + 0.456934 * shaded[1]
                            - 0.273570 * shaded[2]
                            - 1.329566 * 255.0,
                    ];
                    for channel in &mut shaded {
                        *channel = channel.clamp(0.0, 255.0);
                    }
                    shaded = [
                        0.884876 * shaded[0] - 0.007972 * shaded[1]
                            + 0.132201 * shaded[2]
                            + 0.042586 * 255.0,
                        -0.222265 * shaded[0]
                            + 1.163902 * shaded[1]
                            + 0.052029 * shaded[2]
                            + 0.011485 * 255.0,
                        -0.182171 * shaded[0]
                            + 0.156777 * shaded[1]
                            + 1.006_06 * shaded[2]
                            + 0.013433 * 255.0,
                    ];
                }

                let interaction_glow = energy * (0.055 + edge.powf(2.2) * 0.12);
                for channel in &mut shaded {
                    *channel = *channel * (1.0 - interaction_glow) + 255.0 * interaction_glow;
                }

                output.put_pixel(
                    x,
                    y,
                    Rgba([
                        shaded[0].clamp(0.0, 255.0) as u8,
                        shaded[1].clamp(0.0, 255.0) as u8,
                        shaded[2].clamp(0.0, 255.0) as u8,
                        (255.0 * antialias) as u8,
                    ]),
                );
            }
        }
    }
    outputs
}

fn average_chroma(source: &RgbaImage, geometry: GlassGeometry) -> f32 {
    let source_scale = source.width() as f32 / geometry.canvas_width;
    let x0 = (geometry.x * source_scale).round() as u32;
    let y0 = (geometry.y * source_scale).round() as u32;
    let width = (geometry.width * source_scale).round() as u32;
    let height = (geometry.height * source_scale).round() as u32;
    let mut total = 0.0;
    let mut count = 0;
    for y in (y0..y0 + height).step_by(4) {
        for x in (x0..x0 + width).step_by(4) {
            let pixel = source.get_pixel(x, y).0;
            let high = pixel[0].max(pixel[1]).max(pixel[2]);
            let low = pixel[0].min(pixel[1]).min(pixel[2]);
            total += f32::from(high - low);
            count += 1;
        }
    }
    total / count as f32
}

fn rounded_rect_distance_and_normal(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
) -> (f32, (f32, f32)) {
    let cx = width * 0.5;
    let cy = height * 0.5;
    let px = x - cx;
    let py = y - cy;
    let half_x = width * 0.5 - radius;
    let half_y = height * 0.5 - radius;
    let qx = px.abs() - half_x;
    let qy = py.abs() - half_y;
    let ox = qx.max(0.0);
    let oy = qy.max(0.0);
    let outside = (ox * ox + oy * oy).sqrt();
    let sdf = outside + qx.max(qy).min(0.0) - radius;
    let inside_distance = -sdf;

    let epsilon = 0.5;
    let dx = rounded_rect_sdf(px + epsilon, py, half_x, half_y, radius)
        - rounded_rect_sdf(px - epsilon, py, half_x, half_y, radius);
    let dy = rounded_rect_sdf(px, py + epsilon, half_x, half_y, radius)
        - rounded_rect_sdf(px, py - epsilon, half_x, half_y, radius);
    let length = (dx * dx + dy * dy).sqrt().max(0.0001);
    (inside_distance, (dx / length, dy / length))
}

fn rounded_rect_sdf(x: f32, y: f32, half_x: f32, half_y: f32, radius: f32) -> f32 {
    let qx = x.abs() - half_x;
    let qy = y.abs() - half_y;
    let ox = qx.max(0.0);
    let oy = qy.max(0.0);
    (ox * ox + oy * oy).sqrt() + qx.max(qy).min(0.0) - radius
}

fn bilinear(image: &RgbaImage, x: f32, y: f32) -> [f32; 3] {
    let x = x.clamp(0.0, image.width() as f32 - 1.001);
    let y = y.clamp(0.0, image.height() as f32 - 1.001);
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = (x0 + 1).min(image.width() - 1);
    let y1 = (y0 + 1).min(image.height() - 1);
    let tx = x - x0 as f32;
    let ty = y - y0 as f32;
    let p00 = image.get_pixel(x0, y0).0;
    let p10 = image.get_pixel(x1, y0).0;
    let p01 = image.get_pixel(x0, y1).0;
    let p11 = image.get_pixel(x1, y1).0;
    let mut result = [0.0; 3];
    for channel in 0..3 {
        let top = p00[channel] as f32 * (1.0 - tx) + p10[channel] as f32 * tx;
        let bottom = p01[channel] as f32 * (1.0 - tx) + p11[channel] as f32 * tx;
        result[channel] = top * (1.0 - ty) + bottom * ty;
    }
    result
}

fn render_image(mut rgba: RgbaImage) -> Arc<RenderImage> {
    for pixel in rgba.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    Arc::new(RenderImage::new(vec![Frame::new(rgba)]))
}

pub fn launch_media_player() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let variant = GlassVariant::from_args(&args);
    let background = DemoBackground::from_args(&args);
    let forced_interaction = InteractionState::from_args(&args);
    let dynamic_scene = args.iter().any(|argument| argument == "dynamic");
    let no_glass = args.iter().any(|argument| argument == "no-glass");
    let benchmark = args.iter().any(|argument| argument == "benchmark");
    let background_run = args.iter().any(|argument| argument == "background-run");
    let stress_count = args
        .iter()
        .find_map(|argument| argument.strip_prefix("stress-")?.parse().ok())
        .unwrap_or(0);
    if variant != GlassVariant::Identity && !no_glass {
        enable_liquid_glass();
    }
    Application::new().run(move |cx: &mut App| {
        cx.activate(!background_run);
        let bounds = Bounds::centered(None, size(px(WINDOW_W), px(WINDOW_H)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(
                        format!(
                            "Horizon - {} Glass - {}",
                            variant.title(),
                            background.title()
                        )
                        .into(),
                    ),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(14.0), px(14.0))),
                }),
                is_resizable: false,
                focus: !background_run,
                show: true,
                ..Default::default()
            },
            |_window, cx| {
                cx.new(|_| {
                    Demo::new(
                        variant,
                        forced_interaction,
                        background,
                        dynamic_scene,
                        no_glass,
                        benchmark,
                        stress_count,
                    )
                })
            },
        )
        .expect("open Horizon media player");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bilinear_sampling_blends_four_neighbors() {
        let image = RgbaImage::from_fn(2, 2, |x, y| {
            Rgba([(x * 100) as u8, (y * 100) as u8, 0, 255])
        });
        assert_eq!(bilinear(&image, 0.5, 0.5), [50.0, 50.0, 0.0]);
    }

    #[test]
    fn every_variant_has_expected_alpha_behavior() {
        let source = RgbaImage::from_pixel(1200, 800, Rgba([80, 110, 140, 255]));
        for variant in [
            GlassVariant::Regular,
            GlassVariant::Clear,
            GlassVariant::RegularTinted,
            GlassVariant::ClearTinted,
        ] {
            let glass = render_liquid_glass(&source, variant, InteractionState::Rest);
            assert_eq!(glass.get_pixel(220, 48).0[3], 255);
        }
        let identity = render_liquid_glass(&source, GlassVariant::Identity, InteractionState::Rest);
        assert!(identity.pixels().all(|pixel| pixel.0[3] == 0));
    }

    #[test]
    fn average_chroma_distinguishes_neutral_and_saturated_backdrops() {
        let neutral = RgbaImage::from_pixel(1200, 800, Rgba([100, 100, 100, 255]));
        let saturated = RgbaImage::from_pixel(1200, 800, Rgba([255, 0, 0, 255]));
        let geometry = GlassGeometry::reference_player();
        assert_eq!(average_chroma(&neutral, geometry), 0.0);
        assert_eq!(average_chroma(&saturated, geometry), 255.0);
    }

    #[test]
    fn custom_geometry_clamps_capsule_and_negative_corner_radii() {
        let source = RgbaImage::from_pixel(1200, 800, Rgba([80, 110, 140, 255]));
        for corner_radius in [-20.0, 10_000.0] {
            let glass = render_liquid_glass_in_rect(
                &source,
                GlassGeometry {
                    canvas_width: 1200.0,
                    x: 80.0,
                    y: 120.0,
                    width: 120.0,
                    height: 40.0,
                    corner_radius,
                },
                GlassVariant::Clear,
                InteractionState::Rest,
            );
            assert_eq!(glass.get_pixel(60, 20).0[3], 255);
        }
    }

    #[test]
    fn batched_frames_are_identical_to_independent_renders() {
        let source = RgbaImage::from_fn(1200, 800, |x, y| {
            Rgba([(x % 251) as u8, (y % 239) as u8, ((x + y) % 233) as u8, 255])
        });
        for variant in [
            GlassVariant::Regular,
            GlassVariant::Clear,
            GlassVariant::RegularTinted,
            GlassVariant::ClearTinted,
            GlassVariant::Identity,
        ] {
            let batched = render_liquid_glass_frames(&source, variant);
            assert_eq!(
                batched.rest,
                render_liquid_glass(&source, variant, InteractionState::Rest)
            );
            assert_eq!(
                batched.hover,
                render_liquid_glass(&source, variant, InteractionState::Hover)
            );
            assert_eq!(
                batched.pressed,
                render_liquid_glass(&source, variant, InteractionState::Pressed)
            );
        }
    }
}

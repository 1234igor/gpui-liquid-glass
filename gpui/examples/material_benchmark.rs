use std::io::Cursor;
use std::time::{Duration, Instant};

use gpui_liquid_glass::liquid_glass::{
    render_liquid_glass, render_liquid_glass_frames, GlassVariant, InteractionState,
};

const HARBOUR: &[u8] = include_bytes!("../../validation/shared/harbour.png");
const SAMPLES: usize = 9;

fn measure(mut operation: impl FnMut()) -> Vec<Duration> {
    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        operation();
        samples.push(started.elapsed());
    }
    samples.sort_unstable();
    samples
}

fn report(label: &str, samples: &[Duration]) {
    let median = samples[samples.len() / 2].as_secs_f64() * 1_000.0;
    let minimum = samples[0].as_secs_f64() * 1_000.0;
    let maximum = samples[samples.len() - 1].as_secs_f64() * 1_000.0;
    println!("{label}: median={median:.3} ms min={minimum:.3} ms max={maximum:.3} ms");
}

fn main() {
    let decode = measure(|| {
        std::hint::black_box(
            image::load(Cursor::new(HARBOUR), image::ImageFormat::Png)
                .expect("decode benchmark backdrop")
                .into_rgba8(),
        );
    });
    report("png decode", &decode);

    let source = image::load(Cursor::new(HARBOUR), image::ImageFormat::Png)
        .expect("decode benchmark backdrop")
        .into_rgba8();
    let one_frame = measure(|| {
        std::hint::black_box(render_liquid_glass(
            &source,
            GlassVariant::Regular,
            InteractionState::Rest,
        ));
    });
    report("one material frame", &one_frame);

    let interaction_set = measure(|| {
        for state in [
            InteractionState::Rest,
            InteractionState::Hover,
            InteractionState::Pressed,
        ] {
            std::hint::black_box(render_liquid_glass(&source, GlassVariant::Regular, state));
        }
    });
    report("three independent frames", &interaction_set);

    let batched_interaction_set = measure(|| {
        std::hint::black_box(render_liquid_glass_frames(&source, GlassVariant::Regular));
    });
    report("three batched frames", &batched_interaction_set);

    if let Ok(prefix) = std::env::var("MATERIAL_CAPTURE_PREFIX") {
        let frames = render_liquid_glass_frames(&source, GlassVariant::Regular);
        for (name, frame) in [
            ("rest", frames.rest),
            ("hover", frames.hover),
            ("pressed", frames.pressed),
        ] {
            frame
                .save(format!("{prefix}-{name}.png"))
                .expect("save deterministic material capture");
        }
    }
}

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

static WINDOW_WIDTH: f32 = 1600.0;
static WINDOW_HEIGHT: f32 = 900.0;
static MIN_WINDOW_WIDTH: f32 = 800.0;
static MIN_WINDOW_HEIGHT: f32 = 600.0;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let window_res: egui::Vec2 = [WINDOW_WIDTH, WINDOW_HEIGHT].into();
    let min_window_res: egui::Vec2 = [MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT].into();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_res)
            .with_min_inner_size(min_window_res)
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Uninterruptible",
        native_options,
        Box::new(|cc| Ok(Box::new(uninterruptible::Uninterruptible::new(cc)))),
    )
}


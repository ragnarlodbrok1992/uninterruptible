use egui::util::cache::{ComputerMut, FrameCache};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Uninterruptible {
    // Example stuff:
    // label: String,

    // TODO put stuff here for global context passable state
    // this might not be it tho...
    // command_buffer: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    // value: f32,
}

// Memory for application - persistent, between frames etc
// --> https://docs.rs/egui/latest/egui/struct.Memory.html#structfield.data
// Holy shit, this is absurd...


impl Uninterruptible {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Uninterruptible {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        // TODO put widgets here
        let mut hello_world = String::new();

        // Running every frame application loop
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Uninterruptible - focus work application");

            // TODO put text field that inputs commands for application
            // TODO check how to storage strings? show I use context?
            ui.text_edit_singleline(&mut hello_world);
        });
    }
}

impl Default for Uninterruptible {
    fn default() -> Self {
        Self {
            // Example stuff:
            // label: "Hello World!".to_owned(),
            // value: 2.7,
            // command_buffer: String::new(),
        }
    }
}


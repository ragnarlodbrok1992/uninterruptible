/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Uninterruptible {

    // #[serde(skip)] // This how you opt-out of serialization of a field
    // value: f32,
    #[serde(skip)]
    command_buffer: String,
}

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
        // Taking stuff out of Default TODO check this out
        let Self {command_buffer} = self;

        // Running every frame application loop
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Uninterruptible - focus work application");
            ui.text_edit_singleline(command_buffer);
        });
    }
}

impl Default for Uninterruptible {
    fn default() -> Self {
        Self {
            command_buffer: String::new(),
        }
    }
}


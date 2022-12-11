//use egui_extras::RetainedImage;

#[derive(Default)]
pub struct JuliaSetApp {
    julia_set: crate::apps::JuliaSet,
}

impl eframe::App for JuliaSetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.julia_set.ui(ui);
            });
    }
}

// ----------------------------------------------------------------------------

/// The state that we persist (serialize).
#[derive(Default)]
pub struct State {
    clock: JuliaSetApp,
    selected_anchor: String,
}

pub struct WrapApp {
    state: State,
    //image: Option<RetainedImage>
}

impl WrapApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
            //image: None
        };

        slf
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let /*mut*/ vec = vec![
            (
                "Julia Set",
                "juliaset",
                &mut self.state.clock as &mut dyn eframe::App,
            ),
        ];

        vec.into_iter()
    }
}

impl eframe::App for WrapApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // if let Some(window_info) = frame.info().window_info {
        //     if let Some(anchor) = window_info.location.hash.strip_prefix('#') {
        //         self.state.selected_anchor = anchor.to_owned();
        //     }
        // }

        if self.state.selected_anchor.is_empty() {
            let selected_anchor = self.apps_iter_mut().next().unwrap().0.to_owned();
            self.state.selected_anchor = selected_anchor;
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {

            egui::trace!(ui);

            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });

        self.show_selected_app(ctx, frame);
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    #[cfg(feature = "glow")]
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        self.custom3d.on_exit(gl);
    }

    fn clear_color(&self, visuals: &egui::Visuals) -> egui::Rgba {
        visuals.window_fill().into()
    }
}

impl WrapApp {

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_anchor = "demo".into();
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {

        egui::widgets::global_dark_light_mode_switch(ui);

        let mut selected_anchor = self.state.selected_anchor.clone();
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_owned();
                if frame.is_web() {
                    ui.output().open_url(format!("#{}", anchor));
                }
            }
        }
        self.state.selected_anchor = selected_anchor;

    }

}

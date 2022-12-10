use std::f32;

use egui::{containers::*, widgets::*, *};
//use egui_extras::RetainedImage;
//use poll_promise::Promise;

#[derive(PartialEq)]
pub struct JuliaSet {
    zoom: f32,
    start_line_width: f32,
    width_factor: f32,
    line_count: usize,

    step: usize,
    it: i32,

    //mouse_pos_old: Pos2,
    mouse_pos: Pos2,
    mouse_t_pos: Pos2,
    c: Pos2,
    cx_str: String,
    cy_str: String,
    use_mouse: bool,
    hover_mouse: bool,
    mandelbrot: bool,

    shapes: Vec<Shape>,
    shapes_meta: Vec<Shape>,
}

impl Default for JuliaSet {
    fn default() -> Self {
        Self {
            zoom: 0.5,
            start_line_width: 1.0,

            width_factor: 1.0,
            line_count: 0,

            step: 2,
            it: 100,

            //mouse_pos_old: Pos2::ZERO,
            mouse_pos: Pos2::ZERO,
            mouse_t_pos: Pos2::ZERO,
            c: pos2(0.285, 0.01),
            cx_str: "0.285".to_string(),
            cy_str: "0.01".to_string(),
            use_mouse: false,
            hover_mouse: false,
            mandelbrot: false,

            shapes: Vec::new(),
            shapes_meta: Vec::new(),
        }
    }
}

impl JuliaSet {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );
        let from_screen = to_screen.inverse();

        if let Some(pos) = { ui.ctx().input().pointer.hover_pos() } {
            if ui.ctx().input().pointer.any_click() && self.use_mouse {
                self.c = from_screen.transform_pos(pos);
                self.shapes.clear();
            }
            if self.hover_mouse {
                self.c = from_screen.transform_pos(pos);
                self.shapes.clear();
            }

            //self.mouse_pos_old = self.mouse_pos;
            self.mouse_pos = pos;
        }

        // let promise = Promise::spawn_thread("slow_operation",
        // move || self.paint(&painter));
        // if let Some(result) = promise.ready() {
        //     painter.extend(self.shapes.to_vec());
        // } else {
        // }

        self.paint(&painter);
        painter.extend(self.shapes.to_vec());
        if self.mandelbrot {
            painter.extend(self.shapes_meta.to_vec());
        }

        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::none())
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
            });
    }

    // options popup
    fn options_ui(&mut self, ui: &mut Ui) {
        ui.label(format!(
            "Mouse pos: {}, {}",
            self.mouse_pos.x, self.mouse_pos.y
        ));
        ui.label(format!(
            "Transf pos: {}, {}",
            self.mouse_t_pos.x, self.mouse_t_pos.y
        ));

        ui.separator();
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            ui.add(egui::widgets::Label::new("X:"));
            let response = ui.add(TextEdit::singleline(&mut self.cx_str));
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if self.cx_str.parse::<f32>().is_ok() {
                    self.c.x = self.cx_str.parse::<f32>().expect("a float");
                    self.shapes.clear();
                }
            }
        });

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            ui.add(egui::widgets::Label::new("Y:"));
            let response = ui.add(TextEdit::singleline(&mut self.cy_str));
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if self.cy_str.parse::<f32>().is_ok() {
                    self.c.y = self.cy_str.parse::<f32>().expect("a float");
                    self.shapes.clear();
                }
            }
        });

        if ui
            .add(Slider::new(&mut self.it, 0..=1000).text("iterations"))
            .changed()
        {
            self.shapes.clear();
        }
        if ui
            .add(Slider::new(&mut self.step, 0..=10).text("step"))
            .changed()
        {
            self.shapes.clear();
        }
        ui.checkbox(&mut self.use_mouse, "Click Mouse");
        ui.checkbox(&mut self.hover_mouse, "Hover Mouse");
        ui.checkbox(&mut self.mandelbrot, "Mandelbrot");

        ui.separator();
        ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        ui.add(Slider::new(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
        ui.add(Slider::new(&mut self.width_factor, 0.0..=1.0).text("width factor"));

        egui::reset_button(ui, self);
    }

    // paint the shapes
    fn paint(&mut self, painter: &Painter /*, shapes: &mut std::vec::Vec<egui::Shape> */) {
        // if self.mouse_pos_old == self.mouse_pos {
        //     return;
        // }

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );
        let from_screen = to_screen.inverse();

        self.mouse_t_pos = from_screen.transform_pos(self.mouse_pos);

        if self.shapes.len() > 0 {
            return;
        }

        // let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
        //     let line = [to_screen * points[0], to_screen * points[1]];

        //     // culling
        //     if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
        //         self.shapes.push(Shape::line_segment(line, (width, color)));
        //     }
        // };

        // line options
        let mut width = self.start_line_width;
        width *= self.width_factor;

        // dbg line to mouse cursor
        // let a = pos2(0.0, 0.0);
        // let line2 = to_screen.inverse().transform_pos(self.mouse_pos);
        // paint_line(
        //     [a, line2],
        //     Color32::from_additive_luminance(luminance_u8),
        //     width,
        // );

        for rx in (rect.min.x as i32..rect.max.x as i32).step_by(self.step) {
            for ry in (rect.min.y as i32..rect.max.y as i32).step_by(self.step) {
                let screen_pos = pos2(rx as f32, ry as f32);
                let pos = from_screen.transform_pos(screen_pos);

                let mut x = pos.x;
                let mut y = pos.y;
                let mut i = 0;
                //let mut t = 0.0;

                while x * x + y * y < 4.0 && i < self.it {
                    let t = x * x - y * y + self.c.x; //re
                    y = 2.0 * x * y + self.c.y; //im
                    x = t;
                    i += 1;
                }

                let cutoff = (self.it as f32 * 0.8).round() as i32;
                if i >= cutoff {
                    // luminance
                    let mut luminance = 1.0;
                    let luminance_factor = (i as f64) / (self.it as f64);
                    luminance *= luminance_factor / 2.0;
                    let luminance_u8 = (255.0 * luminance).round() as u8;

                    // add shape
                    self.shapes.push(Shape::circle_filled(
                        screen_pos,
                        width,
                        Color32::from_additive_luminance(luminance_u8),
                    ));
                }
            }
        }

        if self.shapes.len() > 0 && self.mandelbrot {
            self.shapes_meta.push(Shape::circle_filled(
                to_screen.transform_pos(self.c),
                width,
                Color32::GOLD,
            ));
        }

        //painter.extend(self.shapes.to_vec());
    }
}

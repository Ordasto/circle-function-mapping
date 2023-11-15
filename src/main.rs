use std::collections::HashMap;

use macroquad::{
    input::KeyCode,
    miniquad::window::screen_size,
    prelude::*,
    ui::{root_ui, widgets},
};

pub struct State {
    running: bool,
    drawing_gui: bool,
    circle_size: f32,
}
impl Default for State {
    fn default() -> Self {
        State {
            running: true,
            drawing_gui: true,
            circle_size: 0.9,
        }
    }
}

impl State {
    pub fn update(&mut self) {
        self.process_keypresses();
        self.draw_gui();
    }

    fn process_keypresses(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.running = !self.running;
        }

        if is_key_pressed(KeyCode::E) {
            self.drawing_gui = !self.drawing_gui;
        }

        if is_key_down(KeyCode::Escape) {
            macroquad::miniquad::window::request_quit();
        }
    }

    fn draw_gui(&self) {
        if self.drawing_gui {
            widgets::Window::new(21, vec2(50., 50.), vec2(200., 200.))
                .label("Settings")
                .movable(true)
                .titlebar(true)
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, "Some random text");
                    if ui.button(None, "click me") {
                        println!("hi");
                    }

                    ui.separator();

                    ui.label(None, "Some other random text");
                    if ui.button(None, "other button") {
                        println!("hi2");
                    }

                    ui.separator();

                    ui.separator();
                });
        }
    }
}

type Theta = f32;
type Line = (Theta, Theta);

#[macroquad::main("Circle-mapping")]
async fn main() {
    let (w, h) = screen_size();
    //     let w = 1920.0 * 0.6;
    //     let h = 1080.0 * 0.6;
    //     request_new_screen_size(w, h);
    let mut state = State::default();

    //let circle_size = 0.9; // percent of screen to fill,
    let radius = (h * state.circle_size) / 2.0; // fit to height,

    let mut center = Vec2 {
        x: w / 2.0,
        y: h / 2.0,
    };

    let mut thet = 0.0;

    let mut lines: Vec<Line> = (0..1000)
        .map(|_| {
            thet += 0.05;
            (thet, thet)
        })
        .collect();

    let mut modif = 0.0;

    // let mut keybinds: HashMap<KeyCode, BindingType> = HashMap::new();
    // keybinds.insert(KeyCode::Space, BindingType::Toggle(&running));

    loop {
        clear_background(BLACK);

        // might be a better way to do this
        center.x = screen_width() / 2.;
        center.y = screen_height() / 2.;

        // Draw_circle kinda blocky
        draw_poly_lines(center.x, center.y, 255, radius, 0.0, 1.0, WHITE);

        // recalc
        // p1.x = center.x + radius * theta.cos();
        // p1.y = center.y + radius * theta.sin();
        // p2.x = center.x + radius * theta.cos();
        // p2.y = center.y + radius * theta.sin();

        for (theta1, theta2) in lines.iter_mut() {
            // *theta1 += 0.00;
            // at some point, make the theta calc a function pointer or something
            // so that i can pass in different functions faster
            // ALSO make a gui to modify the parameters, imgui stype (immediate gui)
            if state.running {
                // do custom theta_function  fn theta_function(f32, f32) -> (f32, f32)
                // variable that stores currently selected theta function, function to set it
                *theta2 = *theta1 + modif;
                modif += 0.00005;
            }

            let x1 = center.x + radius * theta1.cos();
            let y1 = center.y + radius * theta1.sin();
            let x2 = center.x + radius * theta2.cos();
            let y2 = center.y + radius * theta2.sin();

            draw_line(x1, y1, x2, y2, 0.9, WHITE);
        }

        state.update();

        next_frame().await;
    }
}

// Pulsing constrictiong thing
// *theta2 = *theta1 + modif;
// modif += 0.00005;

// cool peaks forming thing
// *theta2 = *theta1 * modif;
// modif += 0.00005;

use std::collections::HashMap;

use macroquad::{
    input::KeyCode,
    miniquad::window::screen_size,
    prelude::*,
    ui::{root_ui, widgets},
};

fn window_conf() -> Conf{
    Conf {
        window_title: "Circle Mapping".to_string(),
        // icon:
        ..Default::default()
    }
}
struct ScreenSize {
    // this name sucks
    w: f32,
    h: f32,
}
impl Into<ScreenSize> for (f32, f32) {
    fn into(self) -> ScreenSize {
        ScreenSize {
            w: self.0,
            h: self.1,
        }
    }
}
pub struct State {
    screen_size: ScreenSize,
    fullscreen: bool,
    running: bool,
    drawing_gui: bool,
    circle_size: f32,
}
impl Default for State {
    fn default() -> Self {
        State {
            // this will break if high_dpi is set to true
            screen_size: screen_size().into(), // when switching out of fullscreen, the screen size changes
            fullscreen: false,
            running: true,
            drawing_gui: true,
            // should change this to radius it's currently the percent of the height so 0.5 would be 50% of the screen
            circle_size: 0.9,
        }
    }
}

impl State {
    pub fn update(&mut self) {
        self.process_keypresses();
        self.draw_gui();

        if !self.fullscreen {
            self.screen_size = screen_size().into();
        }
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

    fn draw_gui(&mut self) {
        if self.drawing_gui {
            // let mut ui = root_ui();
            // ui.canvas()
            //     .rect(Rect::new(5.0, 5.0, 150.0, 200.0), GRAY, GRAY);
            // if ui.button(vec2(10.0, 10.0), "Toggle Fullscreen") {
            //     self.fullscreen = !self.fullscreen;
            //     set_fullscreen(self.fullscreen);

            //     // When switchout out of fullscreen, it sets the screensize to max
            //     // this means it will revert to the old screensize before fullscreening
            //     if self.fullscreen == false {
            //         request_new_screen_size(self.screen_size.w, self.screen_size.h);
            //     }
            // }
            // if ui.button(vec2(10.0, 40.0), "Start/Stop") {
            //     self.running = !self.running;
            // }

            // This window format might look better, but it's harder and I just wan't to get this ui done
            widgets::Window::new(1, vec2(50., 50.), vec2(200., 200.))
                .label("Settings")
                .movable(true)
                .titlebar(true)
                .ui(&mut root_ui(), |ui| {
                    ui.group(2, vec2(190.0, 190.0), |ui| {
                        if ui.button(vec2(10.0, 10.0), "Toggle Fullscreen") {
                            self.fullscreen = !self.fullscreen;
                            set_fullscreen(self.fullscreen);

                            // When switchout out of fullscreen, it sets the screensize to max
                            // this means it will revert to the old screensize before fullscreening
            
                            if self.fullscreen == false {
                                request_new_screen_size(self.screen_size.w, self.screen_size.h);
                            }
                        }
                        if ui.button(vec2(10.0, 40.0), "Start/Stop") {
                            self.running = !self.running;
                        }
                        ui.tabbar(3, vec2(50.0, 50.0), &["-","+"]);
                        ui.checkbox(22, "Running", &mut self.running);
                    });
                });
        }
    }
}

type Theta = f32;
type Line = (Theta, Theta);

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::default();

    let mut radius = (state.screen_size.h * state.circle_size) / 2.0; // fit to height,

    let mut center = Vec2 {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    let mut thet = 0.0;

    let mut lines: Vec<Line> = (0..1000)
        .map(|_| {
            thet += 0.05;
            (thet, thet)
        })
        .collect();

    let mut modif = 0.0;

    loop {
        clear_background(BLACK);

        center.x = state.screen_size.w / 2.;
        center.y = state.screen_size.h / 2.;

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

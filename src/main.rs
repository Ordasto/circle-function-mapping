use macroquad::{prelude::*, ui::{widgets::{self, Group}, root_ui}};

// use egui_macroquad;
// This might be a bad idea, who knows
type Theta = f32;
type Line = (Theta, Theta);

#[macroquad::main("Circle-mapping")]
async fn main() {
    // let (w,h) = screen_size();
    let w = 1920.0 * 1.;
    let h = 1080.0 * 1.;
    set_fullscreen(true);

    let circle_size = 0.9; // percent of screen to fill,
    let radius  = (h * circle_size) / 2.0; // fit to height,

    let center = Vec2 {
        x: w / 2.0,
        y: h / 2.0,
    };

    let mut thet = 0.0;

    let mut lines: Vec<Line> = (0..1000)
        .map(|_| {
            thet += 0.05;
            return (thet, thet);
        })
        .collect();
    let mut running = true;
    let mut modif = 0.0;
    let mut drawing_gui = true;

    loop {


        clear_background(BLACK);
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
            if running {
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

        // [DO THIS BEFORE IT'S TOO GROSS]
        // i hate this current system, use callbacks or some shit
        // have a hash list of key:value pairs, new keybinding is appending a key:KeyCode and value:Vec<functions>
        // if keybind already exsists, add function to vec, else new pair

        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }
        if is_key_pressed(KeyCode::E) { 
            drawing_gui = !drawing_gui;
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        
        if drawing_gui { // code  put this inside the function with a paramter to clean it up
            draw_gui();
        }

        next_frame().await;
    }
}

fn draw_gui(){
    widgets::Window::new(21, vec2(50., 50.), vec2(200., 200.))
    .label("Settings")
    .movable(true)
    .titlebar(true)
    .ui(&mut root_ui(), |ui|{
        ui.label(vec2(2.0, 5.0), "1");
        ui.label(vec2(2.0, 15.0), "2");

    });
}

// Pulsing constrictiong thing
// *theta2 = *theta1 + modif;
// modif += 0.00005;

// cool peaks forming thing
// *theta2 = *theta1 * modif;
// modif += 0.00005;

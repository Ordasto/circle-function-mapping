use std::f32::consts::PI;

use macroquad::{miniquad::window::screen_size, prelude::*, rand, window};

#[macroquad::main("Circle-mapping")]
async fn main() {
    // let (w,h) = screen_size();

    let w = 1920.0 * 0.8;
    let h = 1080.0 * 0.8;

    request_new_screen_size(w, h);

    let radius = 350.0;

    let center = Vec2 {
        x: w / 2.0,
        y: h / 2.0,
    };

    // let mut points:Vec<Vec2> = vec![];

    let mut theta: f32 = 1.0 * PI;
    let mut p1 = vec2(center.x, center.y);

    let mut p2 = vec2(
        center.x + radius * theta.cos(),
        center.y + radius * theta.sin(),
    );

    loop {
        clear_background(BLACK);
        // Draw_circle kinda blocky
        draw_poly_lines(center.x, center.y, 255, radius, 0.0, 2.0, WHITE);

        theta += 0.3;

        // recalc
        p2.x = center.x + radius * theta.cos();
        p2.y = center.y + radius * theta.sin();

        draw_linev(p1, p2);

        next_frame().await;
    }
}

// GET THE MATH DOWN FIRST
// struct Line{
//     p1:Vec2,
//     p2:Vec2,
// }
// impl Line {
//     fn draw(&self) {
//         draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, 2.0, WHITE);
//     }
// }

// I don't care about color rn
fn draw_linev(p1: Vec2, p2: Vec2) {
    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);
}

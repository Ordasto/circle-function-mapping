use std::{f32::consts::PI, fmt::format};

use macroquad::{miniquad::window::screen_size, prelude::*, rand, window};


// This might be a bad idea, who knows
type Theta = f32;
type Line = (Theta, Theta);

#[macroquad::main("Circle-mapping")]
async fn main() {
    // let (w,h) = screen_size();
    let w = 1920.0 * 1.;
    let h = 1080.0 * 1.;
    set_fullscreen(true);

    // request_new_screen_size(w, h);

    let radius = 350.0;

    let center = Vec2 {
        x: w / 2.0,
        y: h / 2.0,
    };
    
    let mut thet = 0.0;

    let mut lines:Vec<Line> = (0..100).map(|x|
        {
            thet += 0.05;
            return (thet, thet*1.2);
        }
    ).collect();

    let mut modif = 0.0;

    loop {

        clear_background(BLACK);
        // Draw_circle kinda blocky
        draw_poly_lines(center.x, center.y, 255, radius, 0.0, 1.0, WHITE);

        // recalc
        // p1.x = center.x + radius * theta.cos();
        // p1.y = center.y + radius * theta.sin();  
        // p2.x = center.x + radius * theta.cos();
        // p2.y = center.y + radius * theta.sin();

        for (theta1,theta2) in lines.iter_mut() {
            // *theta1 += 0.00;
            *theta2 = *theta1 * modif;
            modif += 0.1;
            let x1 = center.x + radius * theta1.cos();
            let y1 = center.y + radius * theta1.sin();  
            let x2 = center.x + radius * theta2.cos();
            let y2 = center.y + radius * theta2.sin();
            println!("{} {} {} {}",x1,y1,x2,y2);
            draw_line(x1, y1, x2, y2, 2.0, WHITE);
        }



        next_frame().await;
    }



}


fn draw_linev(p1: Vec2, p2: Vec2) {
    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);

}



// fn draw_line_theta()


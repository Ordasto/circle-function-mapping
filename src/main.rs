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
    // get all points from 0 .. 2*PI 
    // stepsize would indicate segment number (f32 stepsize not possible though)
    
    let mut theta: f32 = 1.8 * PI;
    let mut p1 = vec2(
        center.x + radius * theta.cos(),
        center.y + radius * theta.sin(),
    );
    theta = 1.5 * PI;
    let mut p2 = vec2(
        center.x + radius * theta.cos(),
        center.y + radius * theta.sin(),
    );

    let mut lines:Vec<(Vec2,Vec2)> = vec![];
    for i in 0..10{
        let pos:f32 = (i+1) as f32 / 10.;
        // let pos = 1.0;
        lines.push((
            vec2(
                center.x + radius * (theta*pos).cos(),
                center.y + radius * (theta*pos).sin(),
            ),
            vec2(
                center.x + radius * (theta*pos*2.).cos(),
                center.y + radius * (theta*pos*2.).sin(),
            )
        ));
    }

    let mut test_change = 0.0;
    loop {
        clear_background(BLACK);
        // Draw_circle kinda blocky
        draw_poly_lines(center.x, center.y, 255, radius, 0.0, 1.0, WHITE);

        // theta += 0.15;

        // recalc
        // p1.x = center.x + radius * theta.cos();
        // p1.y = center.y + radius * theta.sin();  
        // p2.x = center.x + radius * theta.cos();
        // p2.y = center.y + radius * theta.sin();


        // p2.x = center.x + radius * (theta*test_change).cos();
        // p2.y = center.y + radius * (theta*test_change).sin();
        // test_change += 0.01;

        // draw_linev(p1, p2);

        // for i in lines.iter_mut(){
        //     draw_linev(i.0,i.1);
        //     i.0.x = center.x + radius * (theta).cos();
        //     i.0.y = center.y + radius * (theta).sin();

        //     i.1.x = center.x + radius * (theta*test_change*2.0).cos();
        //     i.1.y = center.y + radius * (theta*test_change*2.0).sin();
        //     test_change += 0.5;
        // }

        



        next_frame().await;
    }
}

// GET THE MATH DOWN FIRST
// struct CircleLine{
//     p1:Vec2,
//     p2:Vec2,
// }
// impl CircleLine {
//     fn from_theta(theta:f32) -> Line{
//         Line {
//             p1: Vec2{
//                 x: center.x + radius * theta.cos(),
//                 y: center.y + radius * theta.sin(),
//             },
//             p2: Vec2{
//                 x: center.x + radius * theta.cos(),
//                 y: center.y + radius * theta.sin(),
//             }
//         }
//     }

//     fn draw(&self) {
//         draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, 1.0, WHITE);
//     }
    
// }

// pub trait test{
//     fn from_theta(&self, theta:f32) -> Self;
// }

// impl test for Vec2{
//     fn update_with_thate(&self, theta:f32){
//         p2.x = center.x + radius * (theta*test_change).cos();
//         p2.y = center.y + radius * (theta*test_change).sin();
//     }
// }
// // I don't care about color rn
fn draw_linev(p1: Vec2, p2: Vec2) {
    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, WHITE);

}

// struct CircleLine<'a>{
//     p1:Vec2,
//     p2:Vec2,
//     theta:f32,
//     circle: &'a CircleLines<'a>,
// }

// impl CircleLine<'_>{
//     fn theta_update(&mut self, theta: f32, ind: usize){
//         self.p1.x = self.circle. + radius * theta.cos();
//         self.p1.y = center.y + radius * theta.sin();  
//         self.p2.x = center.x + radius * theta.cos();
//         self.p2.y = center.y + radius * theta.sin();
//     }
// }

// struct Line{
//     p1:Vec2,
//     p2:Vec2,
//     theta:f32,
// }


// I can just store theta values god damn it
// the x and y is only ever generated from them




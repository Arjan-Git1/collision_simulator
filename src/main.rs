

use macroquad::prelude::*;
struct shape{
    x: f32,
    y:f32,
    size:f32,
    speed:f32
}
fn configuration()->Conf {
    Conf { window_title: "Collision".to_string(), fullscreen: true, ..Default::default()}
}
#[macroquad::main(configuration)]
async fn main() {
    let mut speed = 0.0;
    let mut x = screen_width()/2.0;
    let mut y = screen_height()/2.0;
    let mut circle= shape{
        x:90.0,
        y:y,
        speed:speed,
        size : 32.0
    };
    let circle2 = shape{
        x:1200.0,
        y:y,
        size:32.0,
        speed:speed
    };
    let center1 = circle.size/2.0;
    let center2 = circle2.size/2.0;
    let collision_detector = center1+center2;
    loop {
        let mut   time = get_frame_time();
        clear_background(DARKPURPLE);
        
        if is_key_down(KeyCode::Right) {
            
            speed = 750.0;
         }
        if  is_key_down(KeyCode::Left) {
            speed = -750.0;
    }
        circle.x+=time*speed;
        draw_circle(circle.x, circle.y, circle.size,RED);
        draw_circle(circle2.x, circle2.y, circle2.size, RED);
        if circle2.x - circle.x<collision_detector {
            speed = -speed;
                     }
        circle.x = clamp(circle.x,0.0, screen_width());
        next_frame().await;
    
    
  
}
}

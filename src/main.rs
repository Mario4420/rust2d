extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

struct Player {
    x: i32,
    y: i32,
    size: u32,
    accel: u8,
    vel: u8,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    const GRAVITY: i8 = 10;

    let player: Player = Player { x: 10, y: 10, size: 10, accel: 5, vel: 0 };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
               Event::Quit {..} |
               Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                   break 'running;
               },
               _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(
            Rect::new(
                player.x,
                player.y,
                player.size,
                player.size
            )
        ).unwrap();

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;

struct Player {
    rect: sdl2::rect::Rect,
    velx: i32,
    vely: i32,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    const WINDOW_SIZE: u32 = 600;
    let window = video_subsystem.window("rust", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    const GRAVITY: i32 = 3;
    let mut player: Player = Player {
        rect: Rect::new(10, 10, 30, 30),
        velx: 0,
        vely: 0,
    };

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
               Event::Quit {..} |
               Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                   break 'running;
               },
               Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    if player.rect.y() == 600 - player.rect.size().0 as i32 {
                        player.vely = player.rect.size().0 as i32 * -1;
                    }
               }
               Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    player.velx = (player.rect.size().0 as i32 * -1) / 2;
               }
               Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    player.velx = player.rect.size().0 as i32 / 2;
               }
               Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    player.velx = 0;
               }
               Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    player.velx = 0;
               }
               _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(player.rect).unwrap();
        canvas.present();

        player.rect.set_x(player.rect.x() + player.velx);
        player.rect.set_y(player.rect.y() + player.vely);
        player.vely += GRAVITY;

        if player.rect.y() >= 600 - player.rect.size().0 as i32 {
            player.rect.set_y((WINDOW_SIZE - player.rect.size().0) as i32);
        }
        println!("{} {}", player.rect.x(), player.rect.y());

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

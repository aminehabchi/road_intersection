extern crate sdl2;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::time::{ SystemTime, UNIX_EPOCH };

mod drawing;
use drawing::*;

mod vehicles;
use vehicles::*;

mod helprs;
use helprs::*;

fn now_in_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX EPOCH!")
        .as_millis()
}

fn main() -> Result<(), String> {
    /*************************/

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 Window", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut traffic: Traffic = Traffic::new();

    let mut delay: u128 = (vehicle_width * 16 + (vehicle_width / 4) * 16) as u128;
    let mut last_click: [u128; 4] = [0, 0, 0, 0];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {}
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if now_in_millis() - last_click[0] > delay {
                        let vehicle: Vehicle = Vehicle::new(Direction::Up);
                        traffic.vehicles.push(vehicle);
                        last_click[0] = now_in_millis();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if now_in_millis() - last_click[1] > delay {
                        let vehicle: Vehicle = Vehicle::new(Direction::Down);
                        traffic.vehicles.push(vehicle);
                        last_click[1] = now_in_millis();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if now_in_millis() - last_click[2] > delay {
                        let vehicle: Vehicle = Vehicle::new(Direction::Left);
                        traffic.vehicles.push(vehicle);
                        last_click[2] = now_in_millis();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if now_in_millis() - last_click[3] > delay {
                        let vehicle: Vehicle = Vehicle::new(Direction::Right);
                        traffic.vehicles.push(vehicle);
                        last_click[3] = now_in_millis();
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for v in &traffic.vehicles {
            v.draw(&mut canvas);
        }
        traffic.move_all();

        /**************************************/
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (p1, p2) in &road_lines() {
            canvas.draw_line(*p1, *p2)?;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for line in &road_dashed_lines() {
            for (p1, p2) in line {
                canvas.draw_line(*p1, *p2).unwrap();
            }
        }
        /**************************************/

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

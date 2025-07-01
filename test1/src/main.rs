extern crate sdl2;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod drawing;
use drawing::*;

mod vehicles;
use vehicles::*;

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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {}
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let vehicle: Vehicle = Vehicle::new(Color::RGB(255, 0, 0), Direction::Up);
                    traffic.vehicles.push(vehicle);
                    println!("{}", traffic.vehicles.len());
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let vehicle: Vehicle = Vehicle::new(Color::RGB(255, 255, 0), Direction::Down);
                    traffic.vehicles.push(vehicle);
                    println!("{}", traffic.vehicles.len());
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let vehicle: Vehicle = Vehicle::new(Color::RGB(0, 255, 0), Direction::Left);
                    traffic.vehicles.push(vehicle);
                    println!("{}", traffic.vehicles.len());
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let vehicle: Vehicle = Vehicle::new(Color::RGB(0, 0, 255), Direction::Right);
                    traffic.vehicles.push(vehicle);
                    println!("{}", traffic.vehicles.len());
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

        // Draw road surface
        let road_rect = Rect::new(road_v as i32, road_h as i32, road_width, road_width);
        canvas.set_draw_color(Color::RGB(60, 60, 60));
        canvas.fill_rect(road_rect)?;

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

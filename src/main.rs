extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod drawing;
use drawing::*;

mod vehicles;
use vehicles::*;

mod helprs;
use helprs::*;

fn main() -> Result<(), String> {
    /*************************/

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", WINDOW_WIDTH, WINDOW_HEIGHT)
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

    let delay: u128 = (VEHICLE_WIDTH * 16 + (VEHICLE_WIDTH / 4) * 16) as u128;
    let mut last_click: [u128; 5] = [0, 0, 0, 0, 0];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    if now_in_millis() - last_click[4] > delay {
                        let r = random_between(0, 3);

                        let dir = match r {
                            0 => Direction::Down,
                            1 => Direction::Left,
                            2 => Direction::Right,
                            _ => Direction::Up,
                        };
                        let vehicle: Vehicle = Vehicle::new(dir);
                        traffic.vehicles.push(vehicle);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if now_in_millis() - last_click[0] > delay && traffic.nbr_waiting_vehicle.0 < 6
                    {
                        traffic.nbr_waiting_vehicle.0 += 1;
                        let vehicle: Vehicle = Vehicle::new(Direction::Up);
                        traffic.vehicles.push(vehicle);
                        last_click[0] = now_in_millis();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if now_in_millis() - last_click[1] > delay && traffic.nbr_waiting_vehicle.1 < 6
                    {
                        traffic.nbr_waiting_vehicle.1 += 1;
                        let vehicle: Vehicle = Vehicle::new(Direction::Down);
                        traffic.vehicles.push(vehicle);
                        last_click[1] = now_in_millis();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if now_in_millis() - last_click[2] > delay && traffic.nbr_waiting_vehicle.2 < 6
                    {
                        traffic.nbr_waiting_vehicle.2 += 1;
                        let vehicle: Vehicle = Vehicle::new(Direction::Left);
                        traffic.vehicles.push(vehicle);
                        last_click[2] = now_in_millis();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if now_in_millis() - last_click[3] > delay && traffic.nbr_waiting_vehicle.3 < 6
                    {
                        traffic.nbr_waiting_vehicle.3 += 1;
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
            v.draw(&mut canvas).unwrap();
        }

        draw_lights(&traffic.light, &mut canvas).unwrap();
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

pub fn draw_lights(
    lights: &Option<Direction>,
    canvas: &mut Canvas<Window>,
) -> Result<(), Box<dyn std::error::Error>> {
    let light_size: u32 = 50;
    let lights_position = vec![
        (Direction::Up, ROAD_H - light_size, ROAD_V - light_size),
        (Direction::Right, WINDOW_WIDTH - ROAD_H, ROAD_V - light_size),
        (
            Direction::Down,
            WINDOW_WIDTH - ROAD_H,
            WINDOW_HEIGHT - ROAD_V,
        ),
        (Direction::Left, ROAD_H - light_size, WINDOW_HEIGHT - ROAD_V),
    ];
    for pos in lights_position {
        let rect = Rect::new(
            pos.1.try_into().unwrap(),
            pos.2.try_into().unwrap(),
            light_size,
            light_size,
        );
        if Some(pos.0) == *lights {
            canvas.set_draw_color(Color::RGB(0, 255, 0)); //green
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0)); //red
        }
        canvas.fill_rect(rect)?;
    }
    Ok(())
}

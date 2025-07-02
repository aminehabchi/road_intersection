use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;

use sdl2::rect::Rect;
use crate::drawing::*;
use crate::helprs::*;

pub struct Vehicle {
    pub id: u128,
    pub color: Color,
    pub x: i32,
    pub y: i32,
    pub toward: Towards,
    pub dir: Direction,
}

pub enum Towards {
    Forward,
    Right,
    Left,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Vehicle {
    pub fn new(dir: Direction) -> Self {
        let started_points = started_points();
        let mut x = 0;
        let mut y = 0;
        match dir {
            Direction::Up => {
                x = started_points[0].x;
                y = started_points[0].y - (vehicle_width as i32);
            }
            Direction::Down => {
                x = started_points[1].x;
                y = started_points[1].y + (vehicle_width as i32);
            }
            Direction::Right => {
                x = started_points[3].x + (vehicle_width as i32);
                y = started_points[3].y;
            }
            Direction::Left => {
                x = started_points[2].x - (vehicle_width as i32);
                y = started_points[2].y;
            }
        }
        let mut toward: Towards = Towards::Left;
        let mut color = Color::RGB(0, 0, 255);

        match random_between(0, 3) {
            0 => {
                toward = Towards::Left;
                color = Color::RGB(0, 0, 255);
            }
            1 => {
                toward = Towards::Right;
                color = Color::RGB(255, 255, 0);
            }
            2 => {
                toward = Towards::Forward;
                color = Color::RGB(255, 0, 0);
            }
            _ => {}
        }
        Vehicle {
            id: now_in_millis(),
            color,
            x,
            y,
            dir,
            toward,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
        let rect = Rect::new(self.x, self.y, vehicle_width, vehicle_width);

        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect)?;

        Ok(())
    }
    pub fn movee(&mut self, add_x: i32, add_y: i32) {
        self.x += add_x;
        self.y += add_y;
    }
}

pub struct Traffic {
    pub vehicles: Vec<Vehicle>,
    pub light: Option<Direction>,
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            vehicles: vec![],
            light: None,
        }
    }
    pub fn move_all(&mut self) {
        for vehicle in &mut self.vehicles {
            match vehicle.dir {
                Direction::Down => {
                    if vehicle.y == (window_height - road_h).try_into().unwrap() {
                        match &self.light {
                            Some(light) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                vehicle.movee(0, -1);
                            }
                        }
                    } else {
                        handle_move_down(vehicle, &mut self.light);
                    }
                }
                Direction::Up => {
                    if vehicle.y + (vehicle_width as i32) == road_h.try_into().unwrap() {
                        match &self.light {
                            Some(light) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                vehicle.movee(0, 1);
                            }
                        }
                    } else {
                        handle_move_up(vehicle, &mut self.light);
                    }
                }
                Direction::Right => {
                    if vehicle.x == (window_width - road_h).try_into().unwrap() {
                        match &self.light {
                            Some(light) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                vehicle.movee(-1, 0);
                            }
                        }
                    } else {
                        handle_move_rigth(vehicle, &mut self.light);
                    }
                }
                Direction::Left => {
                    if vehicle.x + (vehicle_width as i32) == road_h.try_into().unwrap() {
                        match &self.light {
                            Some(light) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                vehicle.movee(1, 0);
                            }
                        }
                    } else {
                        handle_move_left(vehicle, &mut self.light);
                    }
                }
            }
        }
    }
}

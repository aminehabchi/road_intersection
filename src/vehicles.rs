use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;

use sdl2::rect::Rect;
use crate::drawing::*;
use crate::helprs::*;

pub struct Vehicle {
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

        let (x, y) = match dir {
            Direction::Up => (started_points[0].x, started_points[0].y - (VEHICLE_WIDTH as i32)),
            Direction::Down => (started_points[1].x, started_points[1].y + (VEHICLE_WIDTH as i32)),
            Direction::Right => (started_points[3].x + (VEHICLE_WIDTH as i32), started_points[3].y),
            Direction::Left => (started_points[2].x - (VEHICLE_WIDTH as i32), started_points[2].y),
        };

        let (toward, color) = match random_between(0, 2) {
            0 => (Towards::Left, Color::RGB(0, 0, 255)), // Blue
            1 => (Towards::Right, Color::RGB(255, 255, 0)), // Yellow
            _ => (Towards::Forward, Color::RGB(255, 0, 0)), // Red
        };

        Vehicle {
            color,
            x,
            y,
            dir,
            toward,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
        let rect = Rect::new(self.x, self.y, VEHICLE_WIDTH, VEHICLE_WIDTH);

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
    pub nbr_waiting_vehicle: (u8, u8, u8, u8),
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            vehicles: vec![],
            light: None,
            nbr_waiting_vehicle: (0, 0, 0, 0),
        }
    }
    pub fn move_all(&mut self) {
        let mut last_vehicle_pos = (0, 0, 0, 0);

        for vehicle in &mut self.vehicles {
            match vehicle.dir {
                Direction::Down => {
                    if vehicle.y == (WINDOW_HEIGHT - ROAD_H).try_into().unwrap() {
                        match &self.light {
                            Some(_) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                self.nbr_waiting_vehicle.1 -= 1;
                                vehicle.movee(0, -1);
                            }
                        }
                    } else if check_safe_distance(vehicle.y, last_vehicle_pos.0) {
                        handle_move_down(vehicle, &mut self.light);
                    }
                    last_vehicle_pos.0 = vehicle.y;
                }
                Direction::Up => {
                    if vehicle.y + (VEHICLE_WIDTH as i32) == ROAD_H.try_into().unwrap() {
                        match &self.light {
                            Some(_) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                self.nbr_waiting_vehicle.0 -= 1;
                                vehicle.movee(0, 1);
                            }
                        }
                    } else if check_safe_distance(vehicle.y, last_vehicle_pos.1) {
                        handle_move_up(vehicle, &mut self.light);
                    }
                    last_vehicle_pos.1 = vehicle.y;
                }
                Direction::Right => {
                    if vehicle.x == (WINDOW_WIDTH - ROAD_H).try_into().unwrap() {
                        match &self.light {
                            Some(_) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                self.nbr_waiting_vehicle.3 -= 1;
                                vehicle.movee(-1, 0);
                            }
                        }
                    } else if check_safe_distance(vehicle.x, last_vehicle_pos.2) {
                        handle_move_rigth(vehicle, &mut self.light);
                    }
                    last_vehicle_pos.2 = vehicle.x;
                }
                Direction::Left => {
                    if vehicle.x + (VEHICLE_WIDTH as i32) == ROAD_H.try_into().unwrap() {
                        match &self.light {
                            Some(_) => {}
                            None => {
                                self.light = Some(vehicle.dir.clone());
                                self.nbr_waiting_vehicle.2 -= 1;
                                vehicle.movee(1, 0);
                            }
                        }
                    } else if check_safe_distance(vehicle.x, last_vehicle_pos.3) {
                        handle_move_left(vehicle, &mut self.light);
                    }
                    last_vehicle_pos.3 = vehicle.x;
                }
            }
        }
    }
}

fn check_safe_distance(current: i32, last: i32) -> bool {
    last == 0 || ((last as f32) - (current as f32)).abs() > (SAFE_DISTANCE as f32)
}

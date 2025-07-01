use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;

use sdl2::rect::Rect;
use crate::drawing::*;

// fn dirs() -> Vec<(i32, i32)> {
//     vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
// }

pub struct Vehicle {
    pub color: Color,
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    // pub next_dir: Direction,
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Vehicle {
    pub fn new(color: Color, dir: Direction) -> Self {
        let started_points = started_points();
        let mut x = 0;
        let mut y = 0;
        match dir {
            Direction::Up => {
                x = started_points[1].x;
                y = started_points[1].y;
            }
            Direction::Down => {
                x = started_points[0].x;
                y = started_points[0].y;
            }
            Direction::Right => {
                x = started_points[3].x;
                y = started_points[3].y;
            }
            Direction::Left => {
                x = started_points[2].x;
                y = started_points[2].y;
            }
        }
        Vehicle {
            color,
            x,
            y,
            dir,
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
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            vehicles: vec![],
        }
    }
    pub fn move_all(&mut self) {
        for vehicle in &mut self.vehicles {
            match vehicle.dir {
                Direction::Up => {
                    vehicle.movee(0, -1);
                }
                Direction::Down => {
                    vehicle.movee(0, 1);
                }
                Direction::Right => {
                    vehicle.movee(-1, 0);
                }
                Direction::Left => {
                    vehicle.movee(1, 0);
                }
            }
        }
    }
}

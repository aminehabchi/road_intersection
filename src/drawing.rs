extern crate sdl2;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub const window_width: u32 = 800;
pub const window_height: u32 = 800;
pub const road_width: u32 = 200;
pub const vehicle_width: u32 = road_width / 2;
pub const road_v: u32 = (window_width - road_width) / 2;
pub const road_h: u32 = (window_height - road_width) / 2;

pub fn started_points() -> Vec<Point> {
    vec![
        Point::new(road_v as i32, 0),
        Point::new(
            (road_v + vehicle_width) as i32,
            (window_height - vehicle_width).try_into().unwrap()
        ),
        Point::new(0, (road_v + vehicle_width) as i32),
        Point::new((window_width - vehicle_width).try_into().unwrap(), road_h as i32)
    ]
}

pub fn road_lines() -> Vec<(Point, Point)> {
    vec![
        (Point::new(road_v as i32, 0), Point::new(road_v as i32, road_h as i32)),
        (
            Point::new((road_v + road_width) as i32, 0),
            Point::new((road_v + road_width) as i32, road_h as i32),
        ),
        /*****************************/
        (
            Point::new((road_v + road_width) as i32, (road_h + road_width) as i32),
            Point::new((road_v + road_width) as i32, window_height as i32),
        ),
        (
            Point::new(road_v as i32, (road_h + road_width) as i32),
            Point::new(road_v as i32, window_height as i32),
        ),
        /*****************************/
        (Point::new(0, road_h as i32), Point::new(road_v as i32, road_h as i32)),
        (
            Point::new(0, (road_h + road_width) as i32),
            Point::new(road_v as i32, (road_h + road_width) as i32),
        ),
        /*****************************/
        (
            Point::new((window_width - road_v) as i32, road_h as i32),
            Point::new(window_width as i32, road_h as i32),
        ),
        (
            Point::new((window_width - road_v) as i32, (road_h + road_width) as i32),
            Point::new(window_width as i32, (road_h + road_width) as i32),
        )
    ]
}

pub fn road_dashed_lines() -> Vec<Vec<(Point, Point)>> {
    vec![
        create_dashed_line(
            (window_width / 2).try_into().unwrap(),
            0,
            (window_width / 2).try_into().unwrap(),
            road_h.try_into().unwrap()
        ),
        create_dashed_line(
            (window_width / 2).try_into().unwrap(),
            (road_h + road_width).try_into().unwrap(),
            (window_width / 2).try_into().unwrap(),
            window_height.try_into().unwrap()
        ),
        create_dashed_line(
            0,
            (window_height / 2).try_into().unwrap(),
            road_h.try_into().unwrap(),
            (window_height / 2).try_into().unwrap()
        ),
        create_dashed_line(
            (window_width - road_h).try_into().unwrap(), //x1
            (window_height / 2).try_into().unwrap(), //y1
            window_width.try_into().unwrap(),
            (window_height / 2).try_into().unwrap()
        )
    ]
}

pub fn create_dashed_line(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(Point, Point)> {
    let dash_length = 10.0;
    let gap_length = 10.0;

    let dx = (x2 - x1) as f64;
    let dy = (y2 - y1) as f64;
    let line_length = (dx * dx + dy * dy).sqrt();

    let mut segments = Vec::new();

    let ux = dx / line_length;
    let uy = dy / line_length;

    let mut traveled = 0.0;

    while traveled < line_length {
        let start_x = (x1 as f64) + ux * traveled;
        let start_y = (y1 as f64) + uy * traveled;

        let end_pos = (traveled + dash_length).min(line_length);
        let end_x = (x1 as f64) + ux * end_pos;
        let end_y = (y1 as f64) + uy * end_pos;

        segments.push((
            Point::new(start_x.round() as i32, start_y.round() as i32),
            Point::new(end_x.round() as i32, end_y.round() as i32),
        ));

        traveled += dash_length + gap_length;
    }

    segments
}



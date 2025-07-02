extern crate sdl2;
use sdl2::rect::Point;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 800;
pub const ROAD_WIDTH: u32 = 100;
pub const VEHICLE_WIDTH: u32 = ROAD_WIDTH / 2;
pub const ROAD_V: u32 = (WINDOW_WIDTH - ROAD_WIDTH) / 2;
pub const ROAD_H: u32 = (WINDOW_HEIGHT - ROAD_WIDTH) / 2;
pub const SAFE_DISTANCE: i32 = ((VEHICLE_WIDTH as f64) * 1.2) as i32;

pub fn started_points() -> Vec<Point> {
    vec![
        Point::new(ROAD_V as i32, 0),
        Point::new(
            (ROAD_V + VEHICLE_WIDTH) as i32,
            (WINDOW_HEIGHT - VEHICLE_WIDTH).try_into().unwrap()
        ),
        Point::new(0, (ROAD_V + VEHICLE_WIDTH) as i32),
        Point::new((WINDOW_WIDTH - VEHICLE_WIDTH).try_into().unwrap(), ROAD_H as i32)
    ]
}

pub fn road_lines() -> Vec<(Point, Point)> {
    vec![
        (Point::new(ROAD_V as i32, 0), Point::new(ROAD_V as i32, ROAD_H as i32)),
        (
            Point::new((ROAD_V + ROAD_WIDTH) as i32, 0),
            Point::new((ROAD_V + ROAD_WIDTH) as i32, ROAD_H as i32),
        ),
        /*****************************/
        (
            Point::new((ROAD_V + ROAD_WIDTH) as i32, (ROAD_H + ROAD_WIDTH) as i32),
            Point::new((ROAD_V + ROAD_WIDTH) as i32, WINDOW_HEIGHT as i32),
        ),
        (
            Point::new(ROAD_V as i32, (ROAD_H + ROAD_WIDTH) as i32),
            Point::new(ROAD_V as i32, WINDOW_HEIGHT as i32),
        ),
        /*****************************/
        (Point::new(0, ROAD_H as i32), Point::new(ROAD_V as i32, ROAD_H as i32)),
        (
            Point::new(0, (ROAD_H + ROAD_WIDTH) as i32),
            Point::new(ROAD_V as i32, (ROAD_H + ROAD_WIDTH) as i32),
        ),
        /*****************************/
        (
            Point::new((WINDOW_WIDTH - ROAD_V) as i32, ROAD_H as i32),
            Point::new(WINDOW_WIDTH as i32, ROAD_H as i32),
        ),
        (
            Point::new((WINDOW_WIDTH - ROAD_V) as i32, (ROAD_H + ROAD_WIDTH) as i32),
            Point::new(WINDOW_WIDTH as i32, (ROAD_H + ROAD_WIDTH) as i32),
        )
    ]
}

pub fn road_dashed_lines() -> Vec<Vec<(Point, Point)>> {
    vec![
        create_dashed_line(
            (WINDOW_WIDTH / 2).try_into().unwrap(),
            0,
            (WINDOW_WIDTH / 2).try_into().unwrap(),
            ROAD_H.try_into().unwrap()
        ),
        create_dashed_line(
            (WINDOW_WIDTH / 2).try_into().unwrap(),
            (ROAD_H + ROAD_WIDTH).try_into().unwrap(),
            (WINDOW_WIDTH / 2).try_into().unwrap(),
            WINDOW_HEIGHT.try_into().unwrap()
        ),
        create_dashed_line(
            0,
            (WINDOW_HEIGHT / 2).try_into().unwrap(),
            ROAD_H.try_into().unwrap(),
            (WINDOW_HEIGHT / 2).try_into().unwrap()
        ),
        create_dashed_line(
            (WINDOW_WIDTH - ROAD_H).try_into().unwrap(), //x1
            (WINDOW_HEIGHT / 2).try_into().unwrap(), //y1
            WINDOW_WIDTH.try_into().unwrap(),
            (WINDOW_HEIGHT / 2).try_into().unwrap()
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

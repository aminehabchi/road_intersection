use rand::Rng;
use std::time::{ SystemTime, UNIX_EPOCH };
use crate::vehicles::*;
use crate::drawing::*;

pub const MIDILTE_POINT: (i32, i32) = ((window_width as i32) / 2, (window_height as i32) / 2);

pub fn random_between(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn now_in_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX EPOCH!")
        .as_millis()
}

pub fn handle_move_rigth(vehicle: &mut Vehicle, light: &mut Option<Direction>) {
    match vehicle.toward {
        Towards::Left => {
            if MIDILTE_POINT.0 == ((vehicle.x - 1) as i32) {
                vehicle.dir = Direction::Down;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(-1, 0);
            }
        }
        Towards::Right => {
            if MIDILTE_POINT.0 - (vehicle_width as i32) == ((vehicle.x - 1) as i32) {
                vehicle.dir = Direction::Up;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(-1, 0);
            }
        }
        Towards::Forward => {
            if vehicle.x + (vehicle_width as i32) == road_h.try_into().unwrap() {
                *light = None;
            }
            // if vehicle.y + (vehicle_width as i32) == road_v.try_into().unwrap() {
            //     *light = None;
            // }
            // if vehicle.x == (window_height - road_h).try_into().unwrap() {
            //     *light = None;
            // }
            vehicle.movee(-1, 0);
        }
    }
}

pub fn handle_move_left(vehicle: &mut Vehicle, light: &mut Option<Direction>) {
    match vehicle.toward {
        Towards::Left => {
            if MIDILTE_POINT.0 - (vehicle_width as i32) == ((vehicle.x - 1) as i32) {
                vehicle.dir = Direction::Up;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(1, 0);
            }
        }
        Towards::Right => {
            if MIDILTE_POINT.0 == ((vehicle.x - 1) as i32) {
                vehicle.dir = Direction::Down;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(1, 0);
            }
        }
        Towards::Forward => {
            if vehicle.x == (window_width - road_h).try_into().unwrap() {
                *light = None;
            }
            vehicle.movee(1, 0);
        }
    }
}

pub fn handle_move_down(vehicle: &mut Vehicle, light: &mut Option<Direction>) {
    match vehicle.toward {
        Towards::Left => {
            if MIDILTE_POINT.1 == ((vehicle.y - 1) as i32) {
                vehicle.dir = Direction::Left;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(0, -1);
            }
        }
        Towards::Right => {
            if MIDILTE_POINT.1 - (vehicle_width as i32) == ((vehicle.y - 1) as i32) {
                vehicle.dir = Direction::Right;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(0, -1);
            }
        }
        Towards::Forward => {
            if vehicle.y + (vehicle_width as i32) == road_h.try_into().unwrap() {
                *light = None;
            }
            vehicle.movee(0, -1);
        }
    }
}

pub fn handle_move_up(vehicle: &mut Vehicle, light: &mut Option<Direction>) {
    match vehicle.toward {
        Towards::Left => {
            if MIDILTE_POINT.1 - (vehicle_width as i32) == ((vehicle.y - 1) as i32) {
                vehicle.dir = Direction::Right;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(0, 1);
            }
        }
        Towards::Right => {
            if MIDILTE_POINT.1 == ((vehicle.y - 1) as i32) {
                vehicle.dir = Direction::Left;
                vehicle.toward = Towards::Forward;
            } else {
                vehicle.movee(0, 1);
            }
        }
        Towards::Forward => {
            if vehicle.y == (window_height - road_h).try_into().unwrap() {
                *light = None;
            }
            vehicle.movee(0, 1);
        }
    }
}

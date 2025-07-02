use crate::drawing::*;
use crate::helprs::*;
use crate::vehicles::*;

pub struct Traffic {
    pub vehicles: Vec<Vehicle>,
    pub light: Option<Direction>,
    pub nbr_waiting_vehicle: (u8, u8, u8, u8),
    pub max_vehicle: u8,
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            vehicles: vec![],
            light: None,
            nbr_waiting_vehicle: (0, 0, 0, 0),
            max_vehicle: 5,
        }
    }
    pub fn add_vehicle(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                if self.nbr_waiting_vehicle.0 == self.max_vehicle {
                    return;
                }
                self.nbr_waiting_vehicle.0 += 1;
            }
            Direction::Down => {
                if self.nbr_waiting_vehicle.1 == self.max_vehicle {
                    return;
                }
                self.nbr_waiting_vehicle.1 += 1;
            }
            Direction::Left => {
                if self.nbr_waiting_vehicle.2 == self.max_vehicle {
                    return;
                }
                self.nbr_waiting_vehicle.2 += 1;
            }
            Direction::Right => {
                if self.nbr_waiting_vehicle.3 == self.max_vehicle {
                    return;
                }
                self.nbr_waiting_vehicle.3 += 1;
            }
        }
        self.vehicles.push(Vehicle::new(dir));
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



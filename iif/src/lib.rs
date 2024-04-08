use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Read;
use std::{fs::File, io::BufReader};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VehDataInFrame {
    pub lane: u32,
    pub id: i32,
    pub distance: f32,
    pub speed: f32,
}

#[derive(Debug, Deserialize)]
struct EgoData {
    lane: u32,
    speed: f32,
}

pub fn init_ego_vehicle(filepath: &str, ego_vehicle: &mut types::adas::VehicleType) {
    let file = File::open(filepath).expect("File open failed!");
    let reader = BufReader::new(file);
    let data: EgoData = serde_json::from_reader(reader).expect("Error parsing JSON!");

    ego_vehicle.distance_m = 0.0;
    ego_vehicle.id = types::adas::constants::EGO_VEHICLE_ID;
    ego_vehicle.speed_mps = data.speed;
    ego_vehicle.lane = types::adas::LaneAssociationType::from(data.lane);
}

pub fn get_vehicles_data_iter(filepath: &str) -> Vec<(String, Vec<VehDataInFrame>)> {
    let mut file = File::open(filepath).expect("Failed to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");

    let deserialized_data: HashMap<String, Vec<VehDataInFrame>> =
        serde_json::from_str(&data).expect("Failed to deserialze data");

    let mut result: Vec<(String, Vec<VehDataInFrame>)> = deserialized_data.into_iter().collect();
    result.sort_by_key(|tuple| tuple.0.clone());
    result
}

pub fn load_cycle_data(
    vehicles: &mut types::adas::NeighborVehiclesType,
    vehs_data_in_frame: &Vec<VehDataInFrame>,
) {
    let mut left_idx = 0;
    let mut center_idx = 0;
    let mut right_idx = 0;

    for veh_data_in_frame in vehs_data_in_frame {
        if types::adas::LaneAssociationType::from(veh_data_in_frame.lane)
            == types::adas::LaneAssociationType::LEFT
        {
            vehicles.vehicles_left_lane[left_idx].lane = types::adas::LaneAssociationType::LEFT;
            vehicles.vehicles_left_lane[left_idx].id = veh_data_in_frame.id;
            vehicles.vehicles_left_lane[left_idx].distance_m = veh_data_in_frame.distance;
            vehicles.vehicles_left_lane[left_idx].speed_mps = veh_data_in_frame.speed;

            left_idx += 1;
        } else if types::adas::LaneAssociationType::from(veh_data_in_frame.lane)
            == types::adas::LaneAssociationType::RIGHT
        {
            vehicles.vehicles_right_lane[right_idx].lane = types::adas::LaneAssociationType::RIGHT;
            vehicles.vehicles_right_lane[right_idx].id = veh_data_in_frame.id;
            vehicles.vehicles_right_lane[right_idx].distance_m = veh_data_in_frame.distance;
            vehicles.vehicles_right_lane[right_idx].speed_mps = veh_data_in_frame.speed;

            right_idx += 1;
        } else if types::adas::LaneAssociationType::from(veh_data_in_frame.lane)
            == types::adas::LaneAssociationType::CENTER
        {
            vehicles.vehicles_center_lane[center_idx].lane =
                types::adas::LaneAssociationType::CENTER;
            vehicles.vehicles_center_lane[center_idx].id = veh_data_in_frame.id;
            vehicles.vehicles_center_lane[center_idx].distance_m = veh_data_in_frame.distance;
            vehicles.vehicles_center_lane[center_idx].speed_mps = veh_data_in_frame.speed;

            center_idx += 1;
        }
    }
}

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Read;
use std::{fs::File, io::BufReader};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataEntry {
    pub lane: i32,
    pub distance: f64,
    pub speed: Vec<f64>,
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

pub fn init_vehicles(filepath: &str) -> HashMap<String, DataEntry> {
    let mut file = File::open(filepath).expect("Failed to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");
    let deserialized_data: HashMap<String, DataEntry> =
        serde_json::from_str(&data).expect("Failed to deserialze data");

    deserialized_data
}

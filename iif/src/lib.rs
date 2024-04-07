use std::{fs::File, io::BufReader};

pub fn init_ego_vehicle(filepath: &str, ego_vehicle: &mut types::adas::VehicleType) {
    #[derive(Debug, serde::Deserialize)]
    struct Data {
        lane: u32,
        speed: f32,
    }

    let file = File::open(filepath).expect("File open failed!");
    let reader = BufReader::new(file);
    let data: Data = serde_json::from_reader(reader).expect("Error parsing JSON!");

    println!("{:?}", data);

    ego_vehicle.distance_m = 0.0;
    ego_vehicle.id = types::adas::constants::EGO_VEHICLE_ID;
    ego_vehicle.speed_mps = data.speed;
    ego_vehicle.lane = types::adas::LaneAssociationType::from(data.lane);
}

// pub fn init_vehicles(filepath: &str, vehicles: &mut types::adas::NeighborVehiclesType) {}

// pub fn load_cycle(cycle: u32, vehicles: &mut types::adas::NeighborVehiclesType) {}

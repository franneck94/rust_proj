pub mod adas {
    pub mod constants {
        pub const EGO_VEHICLE_ID: i32 = -1;
        pub const NUM_VEHICLES_ON_LANE: usize = 2;
        pub const VIEW_RANGE_M: f32 = 100.0;
        pub const LONGITUDINAL_DIFFERENCE_PERCENTAGE: f32 = 0.05;
    }

    #[derive(Debug, Clone)]
    pub enum LaneAssociationType {
        LEFT,
        CENTER,
        RIGHT,
        NONE,
    }

    impl LaneAssociationType {
        pub fn from(lane: u32) -> Self {
            match lane {
                0 => LaneAssociationType::LEFT,
                1 => LaneAssociationType::CENTER,
                2 => LaneAssociationType::RIGHT,
                _ => LaneAssociationType::NONE,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct VehicleType {
        pub id: i32,
        pub lane: LaneAssociationType,
        pub speed_mps: f32,
        pub distance_m: f32,
    }

    impl VehicleType {
        pub fn new() -> Self {
            VehicleType {
                id: 0,
                lane: LaneAssociationType::NONE,
                distance_m: 0.0,
                speed_mps: 0.0,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct NeighborVehiclesType {
        pub vehicles_left_lane: [VehicleType; constants::NUM_VEHICLES_ON_LANE],
        pub vehicles_center_lane: [VehicleType; constants::NUM_VEHICLES_ON_LANE],
        pub vehicles_right_lane: [VehicleType; constants::NUM_VEHICLES_ON_LANE],
    }

    impl NeighborVehiclesType {
        pub fn new() -> Self {
            NeighborVehiclesType {
                vehicles_left_lane: [VehicleType::new(), VehicleType::new()],
                vehicles_center_lane: [VehicleType::new(), VehicleType::new()],
                vehicles_right_lane: [VehicleType::new(), VehicleType::new()],
            }
        }
    }
}

pub mod iif {
    pub mod constants {
        pub const NUM_VEHICLES: usize = 6;
        pub const NUM_ITERATIONS: usize = 1 - 000;
    }

    #[derive(Debug, Clone)]
    pub struct VehicleLogData {
        pub id: i32,
        pub lane: super::adas::LaneAssociationType,
        pub start_distance_m: f32,
        pub speeds_mps: [i32; constants::NUM_ITERATIONS],
    }
}

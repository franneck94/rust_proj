fn kph_to_mps(kph: f32) -> f32 {
    kph / 3.6
}

fn mps_to_kph(mps: f32) -> f32 {
    mps * 3.6
}

pub fn print_vehicle_on_lane(
    vehicle: &types::adas::VehicleType,
    range_m: f32,
    offset_m: u32,
    string: &mut String,
    idx: &mut usize,
) {
    if (range_m >= vehicle.distance_m) && (vehicle.distance_m > (range_m - offset_m as f32)) {
        string.push('V');
        *idx += 1;
    } else if f32::abs(vehicle.distance_m) > types::adas::constants::VIEW_RANGE_M as f32 {
        *idx += 1;
    }
}

pub fn print_scene(
    ego_vehicle: &types::adas::VehicleType,
    vehicles: &types::adas::NeighborVehiclesType,
) {
    println!("    \t   L     C     R  ");

    let mut left_idx = 0;
    let mut center_idx = 0;
    let mut right_idx = 0;

    let offset_m: u32 = 10;

    for i in (-(types::adas::constants::VIEW_RANGE_M as f32) as i32
        ..=(types::adas::constants::VIEW_RANGE_M as i32))
        .rev()
        .step_by(offset_m as usize as usize)
    {
        let range_m = i as f32;

        let mut left_string = String::from("   ");
        let mut center_string = String::from("   ");
        let mut right_string = String::from("   ");

        let mut ego_string: Option<&mut String> = None;

        match ego_vehicle.lane {
            types::adas::LaneAssociationType::LEFT => ego_string = Some(&mut left_string),
            types::adas::LaneAssociationType::CENTER => ego_string = Some(&mut center_string),
            types::adas::LaneAssociationType::RIGHT => ego_string = Some(&mut right_string),
            _ => {}
        }

        if let Some(string) = ego_string {
            if (range_m >= ego_vehicle.distance_m)
                && (ego_vehicle.distance_m > (range_m - offset_m as f32))
            {
                string.push('E');
            }
        }

        if left_idx < types::adas::constants::NUM_VEHICLES_ON_LANE {
            print_vehicle_on_lane(
                &vehicles.vehicles_left_lane[left_idx],
                range_m,
                offset_m,
                &mut left_string,
                &mut left_idx,
            );
        }
        if center_idx < types::adas::constants::NUM_VEHICLES_ON_LANE {
            print_vehicle_on_lane(
                &vehicles.vehicles_center_lane[center_idx],
                range_m,
                offset_m,
                &mut center_string,
                &mut center_idx,
            );
        }
        if right_idx < types::adas::constants::NUM_VEHICLES_ON_LANE {
            print_vehicle_on_lane(
                &vehicles.vehicles_right_lane[right_idx],
                range_m,
                offset_m,
                &mut right_string,
                &mut right_idx,
            );
        }

        println!(
            "{}\t| {} | {} | {} |",
            i, left_string, center_string, right_string
        );
    }

    println!();
}

pub fn print_vehicle_speed(vehicle: &types::adas::VehicleType, name: &str) {
    println!("{}: ({:.3} mps)", name, vehicle.speed_mps);
}

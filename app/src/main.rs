use clap::Parser;
use std::env;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    vehicles_filepath: Option<String>,

    #[arg(short, long)]
    ego_filepath: Option<String>,
}

fn get_default_paths() -> (String, String) {
    let project_dir_buf = env::current_dir().unwrap();
    // XXX: While debugging the path is diffrent when launched in vscode
    let mut project_dir = Path::new(&project_dir_buf).parent().unwrap();
    if !project_dir.to_str().unwrap().contains("rust_proj") {
        project_dir = Path::new(&project_dir_buf);
    }

    let vehicle_data_rel_path = Path::new("data/vehicle_data.json");
    let ego_data_rel_path = Path::new("data/ego_data.json");

    let vehicle_data_abs_path = project_dir
        .join(vehicle_data_rel_path)
        .to_str()
        .unwrap()
        .to_string();
    let ego_data_abs_path = project_dir
        .join(ego_data_rel_path)
        .to_str()
        .unwrap()
        .to_string();

    (ego_data_abs_path, vehicle_data_abs_path)
}

fn main() {
    let (ego_data_abs_path, veh_data_abs_path) = get_default_paths();

    let args = Args::parse();

    let vehicles_filepath = args.vehicles_filepath.unwrap_or_else(|| veh_data_abs_path);

    let ego_filepath = args.ego_filepath.unwrap_or_else(|| ego_data_abs_path);

    let mut cycle: u32 = 0;
    let mut ego_vehicle = types::adas::VehicleType::new();
    let mut vehicles = types::adas::NeighborVehiclesType::new();

    iif::init_ego_vehicle(&ego_filepath, &mut ego_vehicle);
    let veh_data_iter = iif::get_vehicles_data_iter(&vehicles_filepath);

    for (frame, veh_data_in_frame) in veh_data_iter {
        for veh_id in 0..types::iif::constants::NUM_VEHICLES {
            println!(
                "Frame: {} Veh: {} on Lane: {:?}",
                frame,
                veh_id,
                veh_data_in_frame[&veh_id.to_string()].lane
            );
        }
    }
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    vehicles_filepath: Option<String>,

    #[arg(short, long)]
    ego_filepath: Option<String>,
}

fn main() {
    let args = Args::parse();

    let vehicles_filepath = args.vehicles_filepath.unwrap_or_else(|| {
        r"C:\Users\Jan\Documents\_LocalCoding\rust_proj\data\vehicle_data.json".to_string()
    });
    let ego_filepath = args.ego_filepath.unwrap_or_else(|| {
        r"C:\Users\Jan\Documents\_LocalCoding\rust_proj\data\ego_data.json".to_string()
    });

    let mut cycle: u32 = 0;
    let mut ego_vehicle = types::adas::VehicleType::new();
    let mut vehicles = types::adas::NeighborVehiclesType::new();

    iif::init_ego_vehicle(&ego_filepath, &mut ego_vehicle);
}

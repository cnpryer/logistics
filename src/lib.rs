/*
* TODO:
* - get started. lay out the problem with initial model data including constraint values
* - create initial methods to play with computation
* - abstract into proper testing and modules
* - dev-tooling(?) - debug, testing, automation
*/

pub fn main() {
    // origin test data
    const ORIGIN_LAT: f32 = 40.0;
    const ORIGIN_LON: f32 = 120.0;

    // demand test data
    const N: usize = 10;
    const DEMAND: [i8; N] = [10, 9, 6, 4, 5, 12, 9, 10, 13, 6]; // TODO: add 0 demand

    // destination test data
    const DEST_LATS: [f32; N] = [
        39.6893, 43.7266, 44.7793, 42.3989, 40.3107, 41.9501, 43.2828, 42.0379,
        38.0045, 39.0963,
    ];
    const DEST_LONS: [f32; N] = [
        -86.3919, -87.8242, -93.5197, -87.8554, -88.1462, -87.695866, -84.6088,
        -93.6003, -85.6888, -84.5719,
    ];

    // simple example assertion to abstract
    assert_eq!(
        DEST_LATS.len(),
        DEST_LONS.len(),
        "dest lats ({}) and lons ({})",
        DEST_LATS.len(),
        DEST_LONS.len()
    );

    assert_eq!(N, DEMAND.len());

    // setup params
    const INT_PRECISION: i32 = 100; // NOTE: better way to use less mem?

    // modeling constraints
    const MAX_VEHICLE_CAP: i8 = 26;
    const MAX_ROUTE_DIST: i32 = 3000 * INT_PRECISION;

    println!(
        "Initialized with 
        N={}, 
        origin=({},{}),
        max vehicle capacity={}, 
        max route distance={}",
        N, ORIGIN_LAT, ORIGIN_LON, MAX_VEHICLE_CAP, MAX_ROUTE_DIST
    )
}

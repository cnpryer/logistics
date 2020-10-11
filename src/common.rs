/*
* TODO:
*   - don't use globals
*   - remove _ prefix when everything is used
*/

// origin test data
pub const ORIGIN_LAT: f32 = 40.0;
pub const ORIGIN_LON: f32 = 120.0;

// demand test data
pub const N: usize = 10;
pub const _DEMAND: [i8; N] = [10, 9, 6, 4, 5, 12, 9, 10, 13, 6]; // TODO: add 0 demand

// destination test data
pub const DEST_LATS: [f32; N] = [
    39.6893, 43.7266, 44.7793, 42.3989, 40.3107, 41.9501, 43.2828, 42.0379, 38.0045,
    39.0963,
];
pub const DEST_LONS: [f32; N] = [
    -86.3919, -87.8242, -93.5197, -87.8554, -88.1462, -87.695866, -84.6088, -93.6003,
    -85.6888, -84.5719,
];

// setup params
pub const INT_PRECISION: i32 = 100; // NOTE: better way to use less mem?

// modeling constraints
pub const _MAX_VEHICLE_CAP: i8 = 26;
// const NUM_VEHICLES: usize = N;
pub const _MAX_ROUTE_DIST: i32 = 3000 * INT_PRECISION;
#[allow(dead_code)]
pub enum DistanceUnit {
    KM,
    M,
    MI,
    NM,
    FT,
    IN,
}

pub const DISTANCE_UNIT: DistanceUnit = DistanceUnit::MI;

// "types"
pub type DistanceMatrixType = [[i32; N + 1]; N + 1];

/*
* TODO:
* - get started. lay out the problem with initial model data including constraint values
* - create initial methods to play with computation
* - don't use globals
* - abstract into proper testing and modules
* - dev-tooling(?) - debug, testing, automation
*/

// origin test data
const ORIGIN_LAT: f32 = 40.0;
const ORIGIN_LON: f32 = 120.0;

// demand test data
const N: usize = 10;
const _DEMAND: [i8; N] = [10, 9, 6, 4, 5, 12, 9, 10, 13, 6]; // TODO: add 0 demand

// destination test data
const DEST_LATS: [f32; N] = [
    39.6893, 43.7266, 44.7793, 42.3989, 40.3107, 41.9501, 43.2828, 42.0379, 38.0045,
    39.0963,
];
const DEST_LONS: [f32; N] = [
    -86.3919, -87.8242, -93.5197, -87.8554, -88.1462, -87.695866, -84.6088, -93.6003,
    -85.6888, -84.5719,
];

// setup params
const INT_PRECISION: i32 = 100; // NOTE: better way to use less mem?

// modeling constraints
const MAX_VEHICLE_CAP: i8 = 26;
// const NUM_VEHICLES: usize = N;
const MAX_ROUTE_DIST: i32 = 3000 * INT_PRECISION;

// "types"
type DistanceMatrixType = [[i32; N + 1]; N + 1];

#[allow(dead_code)]
enum DistanceUnit {
    KM,
    M,
    MI,
    NM,
    FT,
    IN,
}

const DISTANCE_UNIT: DistanceUnit = DistanceUnit::MI;

pub fn main() {
    println!(
        "Initialized with N={},origin=({},{}),max vehicle capacity={},max route distance={}",
        N, ORIGIN_LAT, ORIGIN_LON, MAX_VEHICLE_CAP, MAX_ROUTE_DIST
    );

    let olat = ORIGIN_LAT;
    let olon = ORIGIN_LON;
    let dlats = DEST_LATS;
    let dlons = DEST_LONS;

    let _dist_matrix: DistanceMatrixType =
        create_distance_matrix(olat, olon, dlats, dlons);
}

fn create_distance_matrix(
    olat: f32,
    olon: f32,
    dlats: [f32; N],
    dlons: [f32; N],
) -> DistanceMatrixType {
    /*
     * Takes origin lat and lon and destination lats and lons and returns
     * a matrix of distances.
     *
     * 1. [
     *    [olat, olon],
     *    [dlat_0, dlon_0],
     *    ... N
     * ]
     *
     * 2.[
     *    [0, a_1, a_2, ... a_N],
     *    [b_0, 0, b_2, ... b_N],
     *    ... N
     * ]
     *
     * TODO:
     *   - Refactor to use new_arr.len() for iterator
     */
    let mut geocode_matrix = [[0.0, 0.0]; N + 1];
    for i in 0..N + 1 {
        // TODO: refactor to use 1..N (origin always added first)
        if i == 0 {
            geocode_matrix[i] = [olat, olon];
        } else {
            geocode_matrix[i] = [dlats[i - 1], dlons[i - 1]];
        }
    }

    // TODO: don't want 0 initial values
    let unit = DISTANCE_UNIT;
    let mut distance_matrix: DistanceMatrixType = [[0; N + 1]; N + 1];
    for i in 0..N + 1 {
        for j in 0..N + 1 {
            let origin = geocode_matrix[i];
            let dest = geocode_matrix[j];

            distance_matrix[i][j] = (cacluate_haversine(origin, dest, &unit)
                * INT_PRECISION as f32)
                .round() as i32;
        }
    }

    distance_matrix
}

fn cacluate_haversine(origin: [f32; 2], dest: [f32; 2], unit: &DistanceUnit) -> f32 {
    /*
     * Caculates distance between two geo coordinate pairs using haversine method.
     *
     * NOTE:
     *   - currently for miles.
     *   - can use tuples for lat, lon = loc unpacking
     *
     *
     * TODO:
     *   - when refactoring utilize Rust's error handling using enums
     */
    const AVG_EARTH_RADIUS: f32 = 6371.0088; // in km

    let radius = match unit {
        DistanceUnit::KM => AVG_EARTH_RADIUS * 1.0,
        DistanceUnit::MI => AVG_EARTH_RADIUS * 1000.0,
        DistanceUnit::M => AVG_EARTH_RADIUS * 0.621371192,
        DistanceUnit::NM => AVG_EARTH_RADIUS * 0.539956803,
        DistanceUnit::FT => AVG_EARTH_RADIUS * 3280.839895013,
        DistanceUnit::IN => AVG_EARTH_RADIUS * 39370.078740158,
    };

    let r_olat = origin[0].to_radians();
    let r_olon = origin[1].to_radians();
    let r_dlat = dest[0].to_radians();
    let r_dlon = dest[1].to_radians();

    let d = ((r_dlat - r_olat) * 0.5).sin().powi(2)
        + r_olat.cos() * r_dlat.cos() * ((r_dlon - r_olon) * 0.5).powi(2);

    2.0 * radius * d.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocodes() {
        assert_eq!(DEST_LATS.len(), DEST_LONS.len());
    }

    #[test]
    fn test_demand() {
        assert_eq!(N, _DEMAND.len());
    }

    #[test]
    fn test_valid_calculate_haversine() {
        let origin = [0.0, 0.0];
        let dest = [0.0, 0.0];
        let unit = DistanceUnit::MI;

        let result = cacluate_haversine(origin, dest, &unit);
        assert_eq!(result, 0.0);
    }
}

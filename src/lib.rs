/*
* TODO:
* - dev-tooling: debug, automation
*/
mod common;
mod tests; // TODO: look into integration tests (see book)

pub fn solve() {
    println!(
        "Initialized with N={},
        origin=({},{}),
        num demands={},
        max vehicle capacity={},
        max route distance={}",
        common::N,
        common::ORIGIN_LAT,
        common::ORIGIN_LON,
        common::DEMAND.len(),
        common::MAX_VEHICLE_CAP,
        common::MAX_ROUTE_DIST
    );

    let olat = common::ORIGIN_LAT;
    let olon = common::ORIGIN_LON;
    let dlats = common::DEST_LATS;
    let dlons = common::DEST_LONS;

    let geo_matrix = create_geo_matrix(olat, olon, dlats, dlons);
    let _dist_matrix = create_distance_matrix(geo_matrix);
}

fn create_geo_matrix(
    olat: f32,
    olon: f32,
    dlats: [f32; common::N],
    dlons: [f32; common::N],
) -> [[f32; 2]; common::N + 1] {
    /*
     * Takes origin and dest lats and lons and creates matrix of geocode pairs.
     *
     * [
     *    [olat, olon],
     *    [dlat_0, dlon_0],
     *    ... N + 1
     * ]
     */
    let mut geo_matrix = [[0.0, 0.0]; common::N + 1];

    geo_matrix[0] = [olat, olon];
    for i in 1..geo_matrix.len() {
        geo_matrix[i] = [dlats[i - 1], dlons[i - 1]];
    }

    geo_matrix
}

fn create_distance_matrix(
    geo_matrix: [[f32; 2]; common::N + 1],
) -> common::DistanceMatrixType {
    /*
     * Takes origin lat and lon and destination lats and lons and returns
     * a matrix of distances.
     *
     * [
     *    [0, a_1, a_2, ... a_N],
     *    [b_0, 0, b_2, ... b_N],
     *    ... N
     * ]
     *
     * TODO:
     *   - Refactor to use new_arr.len() for iterator
     */

    // TODO: don't want 0 initial values
    let unit = common::DISTANCE_UNIT;
    let mut dist_matrix: common::DistanceMatrixType = [[0; common::N + 1]; common::N + 1];

    for i in 0..dist_matrix.len() {
        for j in 0..dist_matrix.len() {
            let origin = geo_matrix[i];
            let dest = geo_matrix[j];

            dist_matrix[i][j] = (cacluate_haversine(origin, dest, &unit)
                * common::INT_PRECISION as f32)
                .round() as i32;
        }
    }

    dist_matrix
}

fn cacluate_haversine(
    origin: [f32; 2],
    dest: [f32; 2],
    unit: &common::DistanceUnit,
) -> f32 {
    /*
     * Caculates distance between two geo coordinate pairs using haversine method.
     *
     * NOTE: can use tuples for lat, lon = loc unpacking
     */
    const AVG_EARTH_RADIUS: f32 = 6371.0088; // in km

    let radius = match unit {
        common::DistanceUnit::KM => AVG_EARTH_RADIUS * 1.0,
        common::DistanceUnit::MI => AVG_EARTH_RADIUS * 1000.0,
        common::DistanceUnit::M => AVG_EARTH_RADIUS * 0.621371192,
        common::DistanceUnit::NM => AVG_EARTH_RADIUS * 0.539956803,
        common::DistanceUnit::FT => AVG_EARTH_RADIUS * 3280.839895013,
        common::DistanceUnit::IN => AVG_EARTH_RADIUS * 39370.078740158,
    };

    let r_olat = origin[0].to_radians();
    let r_olon = origin[1].to_radians();
    let r_dlat = dest[0].to_radians();
    let r_dlon = dest[1].to_radians();

    let d = ((r_dlat - r_olat) * 0.5).sin().powi(2)
        + r_olat.cos() * r_dlat.cos() * ((r_dlon - r_olon) * 0.5).powi(2);

    2.0 * radius * d.sqrt().asin()
}

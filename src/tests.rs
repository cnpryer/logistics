#[cfg(test)]
pub mod tests {
    use crate::common;

    #[test]
    fn test_geocodes() {
        assert_eq!(common::DEST_LATS.len(), common::DEST_LONS.len());
    }

    #[test]
    fn test_demand() {
        assert_eq!(common::N, common::_DEMAND.len());
    }

    #[test]
    fn test_valid_calculate_haversine() {
        let origin = [0.0, 0.0];
        let dest = [0.0, 0.0];
        let unit = common::DistanceUnit::MI;

        let result = crate::cacluate_haversine(origin, dest, &unit);
        assert_eq!(result, 0.0);
    }
}

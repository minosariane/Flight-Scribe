use crate::gui::waypoint::Waypoint;

pub fn solve(wpt: &mut Waypoint) {
    // Correction factor for true airspeed based on altitude
    let correction = 0.02 * wpt.altitude / 1000.0;
    wpt.true_air_speed = wpt.indicated_air_speed * (1.0 + correction);

    // Delta angle between wind direction and course, normalized in degrees
    let delta_deg = (wpt.wind_direction - wpt.course + 360.0) % 360.0;
    let delta_rad = delta_deg.to_radians();

    // Calculate sine of wind correction angle (WCA)
    let sin_wca = (wpt.wind_speed / wpt.true_air_speed) * delta_rad.sin();
    // Calculate wind correction angle in radians
    let wca_rad = sin_wca.asin();
    // Convert WCA to degrees
    let wca_deg = wca_rad.to_degrees();

    // Calculate ground speed considering wind effect
    let ground_speed = wpt.true_air_speed * wca_rad.cos() - wpt.wind_speed * delta_rad.cos();

    wpt.ground_speed = ground_speed;
    // Correct true heading by adding wind correction angle
    wpt.true_heading = wpt.course + wca_deg;
    // Calculate flight time in seconds
    wpt.flight_time = (wpt.distance / ground_speed) * 3600.0;

}

/// --- TESTS --- ///

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for floating-point comparisons
    fn assert_approx_eq(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() < tol,
            "{} != {} within tolerance {}",
            a,
            b,
            tol
        );
    }

    #[test]
    fn test_altitude_correction() {
        let mut wpt = Waypoint {
            altitude: 5000.0,
            indicated_air_speed: 100.0,
            ..Default::default()
        };

        solve(&mut wpt);
        assert_approx_eq(wpt.true_air_speed, 110.0, 1e-5);
    }

    #[test]
    fn test_headwind() {
        let mut wpt = Waypoint {
            indicated_air_speed: 100.0,
            altitude: 0.0,
            wind_direction: 0.0,
            course: 180.0,
            wind_speed: 20.0,
            distance: 120.0,
            ..Default::default()
        };

        solve(&mut wpt);
        assert_approx_eq(wpt.ground_speed, 120.0, 1e-5);
        assert_approx_eq(wpt.true_heading, 180.0, 1e-5);
        assert_approx_eq(wpt.flight_time, 3600.0, 1e-5);
    }

    #[test]
    fn test_crosswind() {
        let mut wpt = Waypoint {
            indicated_air_speed: 100.0,
            altitude: 0.0,
            wind_direction: 90.0,
            course: 0.0,
            wind_speed: 15.0,
            ..Default::default()
        };

        solve(&mut wpt);

        // Explicit type handling
        let ratio: f64 = 15.0 / 100.0;
        let expected_wca_rad = ratio.asin();
        let expected_wca_deg = expected_wca_rad.to_degrees();

        assert_approx_eq(wpt.true_heading, expected_wca_deg, 1e-5);
        assert_approx_eq(wpt.ground_speed, 100.0 * expected_wca_rad.cos(), 1e-5);
    }

    #[test]
    fn test_angle_normalization() {
        let mut wpt = Waypoint {
            indicated_air_speed: 100.0,
            wind_direction: 10.0,
            course: 350.0,
            wind_speed: 20.0,
            altitude: 0.0,
            ..Default::default()
        };

        solve(&mut wpt);

        // Explicit type handling
        let delta_deg: f64 = (10.0 - 350.0 + 360.0) % 360.0;
        let delta_rad = delta_deg.to_radians();
        let sin_wca: f64 = (20.0 / 100.0) * delta_rad.sin();
        let wca_rad = sin_wca.asin();
        let expected_gs = 100.0 * wca_rad.cos() - 20.0 * delta_rad.cos();

        assert_approx_eq(wpt.ground_speed, expected_gs, 1e-5);
    }

    #[test]
    fn test_zero_wind() {
        let mut wpt = Waypoint {
            indicated_air_speed: 150.0,
            altitude: 2000.0,
            wind_speed: 0.0,
            distance: 156.0,
            ..Default::default()
        };

        solve(&mut wpt);
        assert_approx_eq(wpt.true_air_speed, 156.0, 1e-5);
        assert_approx_eq(wpt.ground_speed, 156.0, 1e-5);
        assert_approx_eq(wpt.true_heading, wpt.course, 1e-5);
        assert_approx_eq(wpt.flight_time, 3600.0, 1e-5);
    }

    #[test]
    fn test_tailwind() {
        let mut wpt = Waypoint {
            indicated_air_speed: 200.0,
            altitude: 1000.0,
            wind_direction: 180.0,
            course: 0.0,
            wind_speed: 30.0,
            ..Default::default()
        };

        solve(&mut wpt);
        assert_approx_eq(wpt.true_air_speed, 204.0, 1e-5);
        assert_approx_eq(wpt.ground_speed, 234.0, 1e-5);
        assert_approx_eq(wpt.true_heading, 0.0, 1e-5);
    }
}

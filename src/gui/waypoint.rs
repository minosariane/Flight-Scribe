pub struct Waypoint {
    pub name: String,
    pub course: f64,
    pub distance: f64,
    pub altitude: f64,
    pub indicated_air_speed: f64,
    pub true_air_speed: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub true_heading: f64,
    pub ground_speed: f64,
    pub flight_time: f64,
}

impl Waypoint {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self {
            name: String::from(""),
            course: 0.0,
            distance: 0.0,
            altitude: 0.0,
            indicated_air_speed: 0.0,
            true_air_speed: 0.0,
            wind_speed: 0.0,
            wind_direction: 0.0,
            true_heading: 0.0,
            ground_speed: 0.0,
            flight_time: 0.0,
        }
    }
}

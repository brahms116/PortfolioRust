pub struct Config {
	base_speed_limit: f64,
	lane_width: i32,
	curb_width: i32,
	lane_marker_width: i32,
	lane_marker_length: i32,
	lane_marker_gap: i32,
	car_width: i32,
	car_max_length: i32,
	curb_color: String,
	road_color: String,
	lane_marker_color: String,
	camera_acceleration: f64,
	camera_trigger_area: i32,
}

impl Default for Config {
	fn default() -> Config {
		Config {
			base_speed_limit: 10.0,
			lane_width: 16,
			curb_width: 2,
			lane_marker_width: 2,
			lane_marker_length: 10,
			lane_marker_gap: 10,
			car_width: 8,
			car_max_length: 16,
			camera_acceleration: 0.5,
			camera_trigger_area: 20,
			road_color: String::from("#393939"),
			curb_color: String::from("#9f9f9f"),
			lane_marker_color: String::from("#4b4b4b"),
		}
	}
}

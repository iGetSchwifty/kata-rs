use std::cmp::Reverse;
use super::driver::Driver;
use super::super::time_service::calculate_seconds;

pub struct AppState {
    pub drivers: Vec<Driver>
}

impl AppState {
    pub fn new() -> AppState {
        AppState { drivers: Vec::new() }
    }

    pub fn add_driver(self: &mut Self, driver_name: &str) -> bool {
        if self.drivers.iter().any(|item| item.name == driver_name) == false {
            self.drivers.push(Driver::new(&driver_name));
            true
        } else {
            false
        }
    }

    pub fn process_trip(self: &mut Self, driver_name: &str, line_tokens: Vec<&str>) {
        if let Some(found_position) = self.drivers.iter().position(|item| item.name == driver_name) {
            let time_diff = calculate_seconds(line_tokens[3]) - calculate_seconds(line_tokens[2]);
            let mut potential_new_distance: f64 = 0.0;

            if let Ok(new_distance) = line_tokens[4].parse::<f64>() {
                potential_new_distance = new_distance;
            }
            
            if time_diff > 0 && potential_new_distance > 0.0 {
                let hours = time_diff as f64 / 3600 as f64;
                let new_mph = potential_new_distance / hours;
                if new_mph >= 5.0 && new_mph <= 100.0 {
                    self.drivers[found_position].add_time(time_diff);
                    self.drivers[found_position].add_distance(potential_new_distance); 
                }
            }
        }
    }

    pub fn display_data(self: &mut Self) -> Vec<String> {
        self.drivers.sort_by_key(|item| Reverse(item.total_distance as i64));
        self.drivers.iter().map(|item| item.details()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let app_state = AppState::new();
        assert!(app_state.drivers.len() == 0);
    }

    #[test]
    fn test_add_true() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("AAAA"));
        assert!(app_state.drivers.len() == 1);
    }

    #[test]
    fn test_add_returns_false_for_duplicates() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("AAAA"));
        assert!(app_state.add_driver("AAAA") == false);
        assert!(app_state.drivers.len() == 1);
    }

    #[test]
    fn test_add_more_than_one() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("AAAA"));
        assert!(app_state.add_driver("BBBB"));
        assert!(app_state.drivers.len() == 2);
    }

    #[test]
    fn test_process_trip() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("AAAA"));
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "17.3"].to_vec());
        assert_eq!(app_state.drivers.len(), 1);
        assert_eq!(app_state.drivers[0].total_distance, 17.3);
        assert_eq!(app_state.drivers[0].total_time, 1800);
    }

    #[test]
    fn test_process_trip_exclude_less_than_5() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("AAAA"));
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "17.3"].to_vec());
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "1.3"].to_vec());
        assert_eq!(app_state.drivers.len(), 1);
        assert_eq!(app_state.drivers[0].total_distance, 17.3);
        assert_eq!(app_state.drivers[0].total_time, 1800);
    }

    #[test]
    fn test_process_trip_exclude_more_than_100() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("AAAA"));
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "17.3"].to_vec());
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "1000.3"].to_vec());
        assert_eq!(app_state.drivers.len(), 1);
        assert_eq!(app_state.drivers[0].total_distance, 17.3);
        assert_eq!(app_state.drivers[0].total_time, 1800);
    }

    #[test]
    fn test_process_trip_does_nothing_for_bad_driver() {
        let mut app_state = AppState::new();
        assert!(app_state.add_driver("BBBB"));
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "17.3"].to_vec());
        assert_eq!(app_state.drivers.len(), 1);
        assert_eq!(app_state.drivers[0].total_distance, 0.0);
        assert_eq!(app_state.drivers[0].total_time, 0);
    }

    #[test]
    fn test_display_data() {
        let mut app_state = AppState::new();
        app_state.add_driver("AAAA");
        app_state.add_driver("BBBB");
        app_state.add_driver("CCCC");
        app_state.process_trip("AAAA", ["Trip", "AAAA", "07:15", "07:45", "17.3"].to_vec());
        app_state.process_trip("AAAA", ["Trip", "AAAA", "06:12", "06:32", "17.3"].to_vec());
        app_state.process_trip("BBBB", ["Trip", "BBBB", "12:01", "13:16", "42.0"].to_vec());
        assert_eq!(app_state.drivers.len(), 3);
        assert_eq!(app_state.display_data(), ["BBBB: 42 miles @ 34 mph",
                                              "AAAA: 35 miles @ 42 mph",
                                              "CCCC: 0 miles"].to_vec());
    }
}
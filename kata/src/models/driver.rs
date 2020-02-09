#[derive(Debug)]
pub struct Driver {
    pub name: String,
    pub total_time: u64,
    pub total_distance: f64,
    time_overflow: u64,
    distane_overflow: u64
}

impl Driver {
    pub fn new(name: &str) -> Driver {
        Driver {
            name: name.to_string(),
            total_time: 0,
            total_distance: 0.0,
            time_overflow: 0,
            distane_overflow: 0
        }
    }

    pub fn details(self: &Self) -> String {
        if self.total_time != 0 && self.total_distance != 0.0 {
            format!("{}: {} miles @ {} mph", self.name,
                                             self.total_distance.round() as u64,
                                             self.average_mph())
        } else {
            format!("{}: 0 miles", self.name)
        }                                 
    }
    
    pub fn add_time(&mut self, time: u64) {
        //  TODO: Check for overflow
        //  Not gonna implement for this project
        //  But ideally everytime you overflow go up by one in the overflow
        //  You could keep track of really large numbers that way
        self.total_time += time;
    }

    pub fn add_distance(&mut self, distance: f64) {
        //  TODO: Check for overflow
        //  See above comment in add_time
        self.total_distance += distance;
    }

    fn average_mph(self: &Self) -> u64 {
        if self.total_time < 1800 {
            0
        } else {
            let hours = self.total_time as f64 / 3600 as f64;
            (self.total_distance / hours).round() as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Driver;

    fn init_driver() -> Driver {
        let test_name = "TestName";
        super::Driver::new(test_name)
    }

    #[test]
    fn test_new() {
        let new_driver = init_driver();
        assert_eq!(new_driver.name, "TestName");
        assert_eq!(new_driver.total_time, 0);
        assert_eq!(new_driver.total_distance, 0.0);
        assert_eq!(new_driver.time_overflow, 0);
        assert_eq!(new_driver.distane_overflow, 0);
    }

    #[test]
    fn test_default_details() {
        let new_driver = init_driver();
        assert_eq!(new_driver.details(), "TestName: 0 miles");
    }

    #[test]
    fn test_add_time() {
        let mut new_driver = init_driver();
        new_driver.add_time(42);
        assert_eq!(new_driver.total_time, 42);
    }

    #[test]
    fn test_add_distance() {
        let mut new_driver = init_driver();
        new_driver.add_distance(42.0);
        assert_eq!(new_driver.total_distance, 42.0);
    }

    #[test]
    fn test_details_after_manipulation() {
        let mut new_driver = init_driver();
        new_driver.add_time(7200);
        new_driver.add_distance(21.0);
        assert_eq!(new_driver.details(), "TestName: 21 miles @ 11 mph");
    }

    #[test]
    fn test_average_mph() {
        let mut new_driver = init_driver();
        new_driver.add_time(3600);
        new_driver.add_distance(21.0);
        assert_eq!(new_driver.average_mph(), 21);
    }
}

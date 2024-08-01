#[derive(Debug)]
pub struct StationInfo {
    pub average: f64,
    pub count: f64,
    pub min: f64,
    pub max: f64,
}

impl Default for StationInfo {
    fn default() -> Self {
        Self {
            average: f64::default(),
            count: f64::default(),
            min: f64::MAX,
            max: f64::default(),
        }
    }
}

impl StationInfo {
    pub fn init_build(average: f64) -> Self {
        Self {
            average,
            count: 1.0,
            ..Default::default()
        }
    }

    pub fn update_average(&mut self, new_value: f64) {
        if self.count == 0.0 {
            self.average = new_value;
        } else {
            let new_average = ((self.count * self.average) + new_value) / (self.count + 1_f64);
            self.average = new_average;
        }
        self.count += 1.0;
    }

    pub fn update_min(&mut self, new_value: f64) {
        if self.min > new_value {
            self.min = new_value;
        }
    }

    pub fn update_max(&mut self, new_value: f64) {
        if self.max < new_value {
            self.max = new_value;
        }
    }
}

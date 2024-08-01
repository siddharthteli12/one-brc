use crate::StationInfo;
use std::collections::HashMap;

pub fn update_values(value: String, station_map: &mut HashMap<String, StationInfo>) {
    let (station_name, temperature) = value.rsplit_once(';').unwrap();
    let index = temperature.find('.').unwrap();
    let temperature = temperature[..(index + 3)].parse::<f64>().unwrap();
    station_map
        .entry(station_name.to_string())
        .and_modify(|info| {
            info.update_average(temperature);
            info.update_max(temperature);
            info.update_min(temperature);
        })
        .or_insert(StationInfo::init_build(temperature));
}

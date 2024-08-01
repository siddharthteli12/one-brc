use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

mod station_info;
use station_info::*;
mod utility;
use utility::update_values;

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    // Hashmap to store station temp info.
    let mut stationmap = HashMap::<String, StationInfo>::new();

    let f = File::open("test1.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines().map_while(Result::ok) {
        update_values(line, &mut stationmap);
    }

    print!("{{");
    for (name, info) in stationmap.iter() {
        print!(
            "{:?}={:?}/{:?}/{:?}, ",
            name, info.min, info.average, info.max
        );
    }

    print!("}}");
    println!("\n\nTime - {:?}", now.elapsed());
    Ok(())
}

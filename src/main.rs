use battery::Battery;
use std::{thread, time};

use crate::batt::update_battery;

pub mod batt;

/*
    The whole error management thing is a bit weird, since for once we actually
    want to silently ignore errors: it's ok if some things aren't here, just
    print the rest...

    TODO while this is called from the script, it should be a oneshot, but
    ideally I would like to update different parts of the bar at different intervals:

    - print every 5 seconds
    - reload battery only once every minute
    - ...
*/

fn main() {
    println!("Hello, world");
}

fn oldmain() -> Result<(), battery::Error> {
    let delay = time::Duration::from_secs(5);
    let battery_manager = battery::Manager::new()?;
    let mut my_batteries: Vec<Battery> = battery_manager.batteries()?.flatten().collect();

    loop {
        println!(
            "{}",
            update_battery(&battery_manager, my_batteries.iter_mut())
        );
        thread::sleep(delay);
    }
}

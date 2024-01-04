use battery::Battery;
use std::io::Write;
use std::{io, thread, time};

use crate::batt::update_battery;
use crate::wifi::get_connected_network;

pub mod batt;
pub mod wifi;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let delay = time::Duration::from_secs(5);
    let battery_manager = battery::Manager::new()?;
    let mut my_batteries: Vec<Battery> = battery_manager.batteries()?.flatten().collect();

    loop {
        if let Some(network_name) = get_connected_network()? {
            print!("{}", network_name);
        }
        print!(
            "{}",
            update_battery(&battery_manager, my_batteries.iter_mut())
        );
        io::stdout().flush()?;
        thread::sleep(delay);
    }
}

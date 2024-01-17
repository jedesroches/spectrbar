use battery::Battery;
use debug_print::debug_println;
use std::fmt::Display;
use std::{thread, time};

use crate::batt::update_battery;
use crate::wifi::get_connected_network;

pub mod batt;
pub mod wifi;

/*
    Somehow this is about 10x more SLOC than the shell script that it is
    replacing. Either my rust is terrible, or my shell is great, or something
    is weird somewhere.
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    debug_println!("Starting main function");
    let update_delay = time::Duration::from_secs(5);

    let battery_manager: battery::Manager = battery::Manager::new()?;
    let mut my_batteries: Vec<Battery> = battery_manager.batteries()?.flatten().collect();

    loop {
        if let Some(network_name) = get_connected_network()? {
            print!("{}", network_name);
        }

        print_section(update_battery(&battery_manager, my_batteries.iter_mut()));

        println!(); // Spectrwm requires a newline at the end of the printed string.
        thread::sleep(update_delay);
    }
}

fn print_section(data: impl Display) {
    print!("{} | ", data);
}

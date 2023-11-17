use std::{fmt::Display, slice::IterMut};

use battery::{units::ratio::percent, Battery};

/*
    The whole error management thing is a bit weird, since for once we actually
    want to silently ignore errors: it's ok if some things aren't here, just
    print the rest...

    XXX somehow less flexible / safe than the shell script, since removing a
    battery won't be detected

    TODO while this is called from the script, it should be a oneshot, but
    ideally I would like to update different parts of the bar at different intervals:

    - reprint a line every second to get accurate time from strftime(3)
    - reload battery only once every minute
    - ...
*/

fn main() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;
    let mut my_batteries: Vec<Battery> = manager.batteries()?.flatten().collect();

    println!("{}", update_battery(&manager, my_batteries.iter_mut()));
    Ok(())
}

#[derive(Debug)]
struct BatteryStatus {
    charging: bool,
    charges: Vec<f32>,
}

impl Display for BatteryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.charging && self.charges.is_empty() {
            return Ok(());
        }

        if self.charging {
            f.write_str("⚇ ")?;
        }

        for charge in &self.charges {
            if *charge <= 15.0 {
                write!(f, "+@fg=1;{:.0}+@fg=0; ", charge)?;
            } else {
                write!(f, "{:.0} ", charge)?;
            };
        }

        f.write_str("| ")?;

        Ok(())
    }
}

fn update_battery(manager: &battery::Manager, batteries: IterMut<Battery>) -> BatteryStatus {
    let mut charging: bool = false;
    let mut charges: Vec<f32> = Vec::new();

    for battery in batteries {
        match manager.refresh(battery) {
            Ok(()) => {
                match battery.state() {
                    battery::State::Charging => charging = true,
                    _ => (),
                }
                charges.push(battery.state_of_charge().get::<percent>())
            }
            Err(_) => (),
        }
    }
    BatteryStatus { charging, charges }
}

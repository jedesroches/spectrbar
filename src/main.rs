use crate::batt::update_battery;
use battery::Battery;

pub mod batt;

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

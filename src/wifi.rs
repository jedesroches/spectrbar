use std::time::Duration;

use dbus::{
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
    Path,
};

const IWD_BUS: &str = "net.connman.iwd";
const IWD_STATION_INTERFACE: &str = "net.connman.iwd.Station";
const IWD_NETWORK_INTERFACE: &str = "net.connman.iwd.Network";

// FIXME: How can I find this out without hardcoding it ?
const IWD_DEVICE_PATH: &str = "/net/connman/iwd/0/5";

pub fn get_connected_network() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let sys_bus_conn = Connection::new_system()?;
    let station_proxy =
        sys_bus_conn.with_proxy(IWD_BUS, IWD_DEVICE_PATH, Duration::from_millis(500));

    if let Ok(connected_path) = station_proxy.get::<Path>(IWD_STATION_INTERFACE, "ConnectedNetwork")
    {
        let network_proxy =
            sys_bus_conn.with_proxy(IWD_BUS, connected_path, Duration::from_millis(500));

        if network_proxy.get::<bool>(IWD_NETWORK_INTERFACE, "Connected")? {
            return Ok(Some(format!(
                "{} | ",
                network_proxy.get::<String>(IWD_NETWORK_INTERFACE, "Name")?,
            )));
        }
    }

    Ok(None)
}

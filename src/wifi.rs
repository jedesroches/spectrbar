use std::{collections::HashMap, time::Duration};

use dbus::{
    arg::{prop_cast, PropMap},
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
    Path,
};

const IWD_BUS: &str = "net.connman.iwd";
const TIMEOUT: Duration = Duration::from_millis(500);

// Interfaces
const IWD_STATION_INTERFACE: &str = "net.connman.iwd.Station";
const IWD_NETWORK_INTERFACE: &str = "net.connman.iwd.Network";
const DBUS_OBJ_MANAGER_INTERFACE: &str = "org.freedesktop.DBus.ObjectManager";

// Methods
const DBUS_GET_MANAGED_OBJECTS: &str = "GetManagedObjects";

// Properties
const CONNECTED_NETWORK_PROPERTY: &str = "ConnectedNetwork";

type InterfaceToPropertiesMap = HashMap<String, PropMap>;
type ManagedObjects = HashMap<Path<'static>, InterfaceToPropertiesMap>;

pub fn get_connected_network(sys_bus_conn: &Connection) -> Result<Option<String>, dbus::Error> {
    get_device_station_properties(sys_bus_conn)?
        .as_ref()
        .and_then(|props| get_connected_network_path(props))
        .map(|path| get_network_name(sys_bus_conn, path))
        .transpose()
}

fn get_device_station_properties(
    connection: &Connection,
) -> Result<Option<InterfaceToPropertiesMap>, dbus::Error> {
    let iwd_root_proxy = connection.with_proxy(IWD_BUS, "/", TIMEOUT);
    let (managed_objects,): (ManagedObjects,) =
        iwd_root_proxy.method_call(DBUS_OBJ_MANAGER_INTERFACE, DBUS_GET_MANAGED_OBJECTS, ())?;

    Ok(managed_objects
        .into_iter()
        .find(|(_, properties)| properties.contains_key(IWD_STATION_INTERFACE))
        .map(|(_, properties)| properties))
}

fn get_network_name(connection: &Connection, network_path: &Path) -> Result<String, dbus::Error> {
    connection
        .with_proxy(IWD_BUS, network_path, TIMEOUT)
        .get::<String>(IWD_NETWORK_INTERFACE, "Name")
}

fn get_connected_network_path(
    station_properties: &InterfaceToPropertiesMap,
) -> Option<&Path<'static>> {
    station_properties
        .get(IWD_STATION_INTERFACE)
        .and_then(|state| prop_cast(state, CONNECTED_NETWORK_PROPERTY))
}

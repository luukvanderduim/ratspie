#![cfg(test)]

use crate::a11y_bus_connection_blocking;

use super::a11y_bus_connection;
use tokio::runtime::Runtime;

/// This tests GetAddress of a11y.Bus service on the session bus.
#[test]
fn init_a11y_conn_async() {
    let rt = Runtime::new().unwrap();

    let conn: zbus::Result<zbus::Connection> = rt.block_on(a11y_bus_connection());
    assert!(conn.is_ok());
    let conn = conn.unwrap();
    assert!(conn.is_bus());
}

/// This tests GetAddress of a11y.Bus service on the session bus.
#[test]
fn init_a11y_conn_blocking() {
    let conn: zbus::Result<zbus::blocking::Connection> = a11y_bus_connection_blocking();
    assert!(conn.is_ok());
    let conn = conn.unwrap();
    assert!(conn.is_bus());
}

#[test]
fn test_a11y_bus_status_is_present() {
    // Accessibility bus service lives on the 'session bus'.
    let conn = zbus::blocking::Connection::session().expect("connection aqcuire failed");
    let status_proxy = super::a11y_bus::StatusProxyBlocking::new(&conn).unwrap();
    if let Err(e) = status_proxy.is_enabled() {
        println!("Problem: {:?}", &e);
    }
    assert!(status_proxy.is_enabled().is_ok());
}

#[test]
fn test_a11y_atspi_registry_events_registered() {
    let conn: zbus::blocking::Connection = a11y_bus_connection_blocking().unwrap();
    let registry_proxy = super::registry::RegistryProxyBlocking::new(&conn).unwrap();
    if let Err(e) = registry_proxy.get_registered_events() {
        println!("Problem: {:?}", &e);
    }
    assert!(registry_proxy
        .get_registered_events()
        .unwrap()
        .eq(&Vec::new()));
}

#[test]
fn test_a11y_atspi_registry_root_accessible_get_application() {
    let conn: zbus::blocking::Connection = a11y_bus_connection_blocking().unwrap();
    let root_registry_proxy = super::registry_root::AccessibleProxyBlocking::new(&conn).unwrap();
    let answer = root_registry_proxy.get_application();

    assert!(answer.is_ok());
}

#[test]
fn test_a11y_atspi_registry_root_component_get_size() {
    let conn: zbus::blocking::Connection = a11y_bus_connection_blocking().unwrap();

    let root_registry_proxy = super::registry_root::ComponentProxyBlocking::new(&conn).unwrap();
    let answer = root_registry_proxy.get_size();
    assert!(answer.is_err());
}

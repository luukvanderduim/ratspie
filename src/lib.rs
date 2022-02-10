#![allow(clippy::type_complexity, non_snake_case, clippy::too_many_arguments)]
pub mod a11y_bus;
// mod event_type;
pub mod proxies;
pub mod registry;
pub mod registry_root;
mod tests;

use a11y_bus::{BusProxy, BusProxyBlocking};
use zbus::{Address, Connection};
use zvariant::OwnedObjectPath;

/// Creates `Connection` in async fashion.
pub async fn a11y_bus_connection() -> zbus::Result<Connection> {
    let a11y_bus_address = {
        let connection = Connection::session().await?;
        let org_a11y_bus_proxy = BusProxy::new(&connection).await?;
        org_a11y_bus_proxy.get_address().await?
    };

    let a11y_bus_address = a11y_bus_address.as_str();
    let a11y_bus_address: Address = Address::try_from(a11y_bus_address)?;
    zbus::ConnectionBuilder::address(a11y_bus_address)?
        .build()
        .await
}

/// Creates `Connection` in blocking fashion.
pub fn a11y_bus_connection_blocking() -> zbus::Result<zbus::blocking::Connection> {
    let a11y_bus_address: String = {
        let connection = zbus::blocking::Connection::session()?;
        let org_a11y_bus_proxy = BusProxyBlocking::new(&connection)?;
        org_a11y_bus_proxy.get_address()?
    };

    let a11y_bus_address: Address = Address::try_from(a11y_bus_address.as_str())?;
    zbus::blocking::ConnectionBuilder::address(a11y_bus_address)?.build()
}

/// Set the session's (per-user) accessibility status on the `org.a11y.Bus` service, on the session bus.
/// async
pub async fn set_session_accessibility_status(
    status: bool,
) -> std::result::Result<(), zbus::Error> {
    let connection = Connection::session().await?;
    let status_proxy = a11y_bus::StatusProxy::new(&connection).await?;

    if status_proxy.is_enabled().await? != status {
        if status {
            status_proxy.set_is_enabled(true).await?;
        } else {
            status_proxy.set_is_enabled(false).await?;
        }
    }
    Ok(())
}

/// Set the session's / per-user accessibility status on the `org.a11y.Bus` service, on the session bus.
/// blocking
pub fn set_session_accessibility_status_blocking(
    status: bool,
) -> std::result::Result<(), zbus::Error> {
    let connection = zbus::blocking::Connection::session()?;
    let status_proxy = a11y_bus::StatusProxyBlocking::new(&connection)?;

    if status_proxy.is_enabled()? != status {
        if status {
            status_proxy.set_is_enabled(true)?;
        } else {
            status_proxy.set_is_enabled(false)?;
        }
    }
    Ok(())
}

/// Request list of children from a11y Registry.
pub async fn get_accessible_children(
    conn: &zbus::Connection,
) -> zbus::Result<Vec<(String, OwnedObjectPath)>> {
    let root_accessible = crate::registry_root::AccessibleProxy::new(conn).await?;
    root_accessible.get_children().await
}

/// Proxy type parameterized function to map a list of (bus_names, paths) to typed AT-SPI proxies.
/// Returns a list of typed accessibility proxies
pub async fn get_proxies<'a, T: zbus::ProxyDefault + From<zbus::Proxy<'a>>>(
    conn: &Connection,
    list: &'a [(String, OwnedObjectPath)],
) -> zbus::Result<Vec<T>> {
    let mut res: Vec<T> = Vec::new();
    for (name, path) in list.iter() {
        let bus_name = zbus_names::BusName::try_from(&**name)?;
        let pb: zbus::ProxyBuilder<'a, T> = zbus::ProxyBuilder::new(conn)
            .destination(bus_name)?
            // interface is taken from `T` associated  const
            .path(path)?;
        let p = zbus::ProxyBuilder::build(pb).await?;
        res.push(p);
    }
    Ok(res)
}

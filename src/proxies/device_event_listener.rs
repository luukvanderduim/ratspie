//! # DBus interface proxy for: `org.a11y.atspi.DeviceEventListener`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `DeviceEventListener.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.DeviceEventListener")]
trait DeviceEventListener {
    /// NotifyEvent method
    fn notify_event(&self, event: &(u32, i32, u32, u32, i32, &str, bool)) -> zbus::Result<bool>;
}

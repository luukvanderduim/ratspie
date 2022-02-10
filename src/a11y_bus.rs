//! # DBus interface proxies for: `org.a11y.Status`, `org.a11y.Bus`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/a11y/bus' from service 'org.a11y.Bus'`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.a11y.Status",
    default_service = "org.a11y.Bus",
    default_path = "/org/a11y/bus"
)]
trait Status {
    /// `IsEnabled` property
    ///
    /// Property inicating whether session Accessibility has been enabled
    /// Applications should monitor and, if set, enable accessibility.
    #[dbus_proxy(property)]
    fn is_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_is_enabled(&self, value: bool) -> zbus::Result<()>;

    /// ScreenReaderEnabled property
    /// Whether a screen reader will be started automatically by the session.
    #[dbus_proxy(property)]
    fn screen_reader_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_screen_reader_enabled(&self, value: bool) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.a11y.Bus",
    default_service = "org.a11y.Bus",
    default_path = "/org/a11y/bus"
)]
trait Bus {
    /// GetAddress method
    /// Returns the a11y DBus address, if available.
    fn get_address(&self) -> zbus::Result<String>;
}

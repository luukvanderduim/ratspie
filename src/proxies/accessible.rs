//! # DBus interface proxy for: `org.a11y.atspi.Accessible`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Accessible.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Accessible")]
trait Accessible {
    /// GetApplication method
    fn get_application(&self) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;

    /// GetAttributes method
    fn get_attributes(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// GetChildAtIndex method
    fn get_child_at_index(
        &self,
        index: i32,
    ) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;

    /// GetChildren method
    fn get_children(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// GetIndexInParent method
    fn get_index_in_parent(&self) -> zbus::Result<i32>;

    /// GetInterfaces method
    fn get_interfaces(&self) -> zbus::Result<Vec<String>>;

    /// GetLocalizedRoleName method
    fn get_localized_role_name(&self) -> zbus::Result<String>;

    /// GetRelationSet method
    fn get_relation_set(
        &self,
    ) -> zbus::Result<Vec<(u32, Vec<(String, zbus::zvariant::OwnedObjectPath)>)>>;

    /// GetRole method
    fn get_role(&self) -> zbus::Result<u32>;

    /// GetRoleName method
    fn get_role_name(&self) -> zbus::Result<String>;

    /// GetState method
    fn get_state(&self) -> zbus::Result<Vec<u32>>;

    /// AccessibleId property
    #[dbus_proxy(property)]
    fn accessible_id(&self) -> zbus::Result<String>;

    /// ChildCount property
    #[dbus_proxy(property)]
    fn child_count(&self) -> zbus::Result<i32>;

    /// Description property
    #[dbus_proxy(property)]
    fn description(&self) -> zbus::Result<String>;

    /// Locale property
    #[dbus_proxy(property)]
    fn locale(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Parent property
    #[dbus_proxy(property)]
    fn parent(&self) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;
}

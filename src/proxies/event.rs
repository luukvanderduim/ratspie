//! # DBus interface proxies for: `org.a11y.atspi.Event.Object`, `org.a11y.atspi.Event.Window`, `org.a11y.atspi.Event.Mouse`, `org.a11y.atspi.Event.Keyboard`, `org.a11y.atspi.Event.Terminal`, `org.a11y.atspi.Event.Document`, `org.a11y.atspi.Event.Focus`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Event.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Object")]
trait Object {
    /// ActiveDescendantChanged signal
    #[dbus_proxy(signal)]
    fn active_descendant_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// AttributesChanged signal
    #[dbus_proxy(signal, name = "object_attributes_changed")]
    fn attributes_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// BoundsChanged signal
    #[dbus_proxy(signal)]
    fn bounds_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// ChildrenChanged signal
    #[dbus_proxy(signal)]
    fn children_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// ColumnDeleted signal
    #[dbus_proxy(signal)]
    fn column_deleted(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// ColumnInserted signal
    #[dbus_proxy(signal)]
    fn column_inserted(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// ColumnReordered signal
    #[dbus_proxy(signal)]
    fn column_reordered(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// LinkSelected signal
    #[dbus_proxy(signal)]
    fn link_selected(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>))
        -> zbus::Result<()>;

    /// ModelChanged signal
    #[dbus_proxy(signal)]
    fn model_changed(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>))
        -> zbus::Result<()>;

    /// PropertyChange signal
    #[dbus_proxy(signal, name = "object_property_changed")]
    fn property_change(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// RowDeleted signal
    #[dbus_proxy(signal)]
    fn row_deleted(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// RowInserted signal
    #[dbus_proxy(signal)]
    fn row_inserted(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// RowReordered signal
    #[dbus_proxy(signal)]
    fn row_reordered(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>))
        -> zbus::Result<()>;

    /// SelectionChanged signal
    #[dbus_proxy(signal)]
    fn selection_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// StateChanged signal
    #[dbus_proxy(signal)]
    fn state_changed(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>))
        -> zbus::Result<()>;

    /// TextAttributesChanged signal
    #[dbus_proxy(signal)]
    fn text_attributes_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// TextBoundsChanged signal
    #[dbus_proxy(signal)]
    fn text_bounds_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// TextCaretMoved signal
    #[dbus_proxy(signal)]
    fn text_caret_moved(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// TextChanged signal
    #[dbus_proxy(signal)]
    fn text_changed(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// TextSelectionChanged signal
    #[dbus_proxy(signal)]
    fn text_selection_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// VisibleDataChanged signal
    #[dbus_proxy(signal)]
    fn visible_data_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.a11y.atspi.Event.Window")]
trait Window {
    /// Activate signal
    #[dbus_proxy(signal)]
    fn activate(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Close signal
    #[dbus_proxy(signal)]
    fn close(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Create signal
    #[dbus_proxy(signal)]
    fn create(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Deactivate signal
    #[dbus_proxy(signal)]
    fn deactivate(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// DesktopCreate signal
    #[dbus_proxy(signal)]
    fn desktop_create(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// DesktopDestroy signal
    #[dbus_proxy(signal)]
    fn desktop_destroy(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// Destroy signal
    #[dbus_proxy(signal)]
    fn destroy(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Lower signal
    #[dbus_proxy(signal)]
    fn lower(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Maximize signal
    #[dbus_proxy(signal)]
    fn maximize(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Minimize signal
    #[dbus_proxy(signal)]
    fn minimize(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Move signal
    #[dbus_proxy(signal)]
    fn move_(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// PropertyChange signal
    #[dbus_proxy(signal)]
    fn property_change(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// Raise signal
    #[dbus_proxy(signal)]
    fn raise(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Reparent signal
    #[dbus_proxy(signal)]
    fn reparent(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Resize signal
    #[dbus_proxy(signal)]
    fn resize(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Restore signal
    #[dbus_proxy(signal)]
    fn restore(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Restyle signal
    #[dbus_proxy(signal)]
    fn restyle(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Shade signal
    #[dbus_proxy(signal)]
    fn shade(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// uUshade signal
    #[dbus_proxy(signal)]
    fn u_ushade(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.a11y.atspi.Event.Mouse")]
trait Mouse {
    /// Abs signal
    #[dbus_proxy(signal)]
    fn abs(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Button signal
    #[dbus_proxy(signal)]
    fn button(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Rel signal
    #[dbus_proxy(signal)]
    fn rel(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.a11y.atspi.Event.Keyboard")]
trait Keyboard {
    /// Modifiers signal
    #[dbus_proxy(signal)]
    fn modifiers(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.a11y.atspi.Event.Terminal")]
trait Terminal {
    /// ApplicationChanged signal
    #[dbus_proxy(signal)]
    fn application_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// CharwidthChanged signal
    #[dbus_proxy(signal)]
    fn charwidth_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// ColumncountChanged signal
    #[dbus_proxy(signal)]
    fn columncount_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// LineChanged signal
    #[dbus_proxy(signal)]
    fn line_changed(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// LinecountChanged signal
    #[dbus_proxy(signal)]
    fn linecount_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.a11y.atspi.Event.Document")]
trait Document {
    /// AttributesChanged signal
    #[dbus_proxy(signal)]
    fn attributes_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// ContentChanged signal
    #[dbus_proxy(signal)]
    fn content_changed(
        &self,
        arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// LoadComplete signal
    #[dbus_proxy(signal)]
    fn load_complete(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>))
        -> zbus::Result<()>;

    /// LoadStopped signal
    #[dbus_proxy(signal)]
    fn load_stopped(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// PageChanged signal
    #[dbus_proxy(signal)]
    fn page_changed(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;

    /// Reload signal
    #[dbus_proxy(signal)]
    fn reload(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.a11y.atspi.Event.Focus")]
trait Focus {
    /// Focus signal
    #[dbus_proxy(signal)]
    fn focus(&self, arg_1: (&str, u32, u32, zbus::zvariant::Value<'_>)) -> zbus::Result<()>;
}

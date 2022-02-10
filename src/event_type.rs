// /// The AT-SPI2 event type represented in Rust enums.
// ///
// ///     As is described in the AT-SPI2 source for [evemt+;ostemer.c](https://gitlab.gnome.org/GNOME/at-spi2-core/-/blob/master/atspi/atspi-event-listener.c)
// ///
// ///     The event type is a character string indicating the type of events for which
// ///    notification is requested.  Format is `EventClass`:`major_type`:`minor_type`:`detail`
// ///

// trait EventType {}

// pub enum ObjectPropertyEventType {
//     Change,
//     ChangeAccessibleName,
//     ChangeAccessibleDescription,
//     ChangeAccessibleParent,
//     ChangeAccessibleValue,
//     ChangeAccessibleRole,
//     ChangeAccessibleTableCaption,
//     ChangeAccessibleColumnDescription,
//     ChangeAccessibleColumnHeader,
//     ChangeAccessibleTableRowDescription,
//     ChangeAccessibleTableRowHeader,
//     ChangeAccessibleTableSummary,
// }

// impl EventType for ObjectPropertyEventType {}

// impl From<ObjectPropertyEventType> for &str {
//     fn from(et: ObjectPropertyEventType) -> Self {
//         use ObjectPropertyEventType::*;
//         match et {
//             Change => "object:property-change",
//             ChangeAccessibleName => "object:property-change:accessible-name",
//             ChangeAccessibleDescription => "object:property-change:accessible-description",
//             ChangeAccessibleParent => "object:property-change:accessible-parent",
//             ChangeAccessibleValue => "object:property-change:accessible-value",
//             ChangeAccessibleRole => "object:property-change:accessible-role",
//             ChangeAccessibleTableCaption => "object:property-change:accessible-table-caption",
//             ChangeAccessibleColumnDescription => {
//                 "object:property-change:accessible-table-column-description"
//             }
//             ChangeAccessibleColumnHeader => "object:property-change:accessible-table-column-header",
//             ChangeAccessibleTableRowDescription => {
//                 "object:property-change:accessible-table-row-description"
//             }
//             ChangeAccessibleTableRowHeader => "object:property-change:accessible-table-row-header",
//             ChangeAccessibleTableSummary => "object:property-change:accessible-table-summary",
//         }
//     }
// }

// /// The format for an EventType is:
// /// EventClass:major_type:minor_type:detail
// /// Apart from EventClass, all subfields are optional.
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum EventClass {
//     Document(Option<LoadEvent>),
//     Focus,
//     Mouse(Option<MouseEvent>),
//     Object,
//     ScreenReader,
//     Terminal,
//     Window,
// }

// impl Display for EventClass {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             EventClass::Document(le) => {
//                 if let Some(le) = le {
//                     write!(f, "document:{}", le)
//                 } else {
//                     write!(f, "document")
//                 }
//             }
//             EventClass::Focus => write!(f, "focus"),
//             EventClass::Mouse(me) => {
//                 if let Some(me) = me {
//                     write!(f, "mouse:{}", me)
//                 } else {
//                     write!(f, "mouse")
//                 }
//             }
//             EventClass::Object => write!(f, "object"),
//             EventClass::ScreenReader => write!(f, "screen-reader"),
//             EventClass::Terminal => write!(f, "terminal"),
//             EventClass::Window => write!(f, "window"),
//         }
//     }
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum LoadEvent {
//     LoadComplete,
//     LoadStopped,
//     Reload,
// }

// impl Display for LoadEvent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             LoadEvent::LoadComplete => write!(f, "load-complete"),
//             LoadEvent::LoadStopped => write!(f, "load-stopped"),
//             LoadEvent::Reload => write!(f, "reload"),
//         }
//     }
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum MouseEvent {
//     Absolute,
//     Button(Option<ButtonEvType>),
//     Relative,
// }

// impl Display for MouseEvent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             MouseEvent::Absolute => write!(f, "absolute"),
//             MouseEvent::Button(mbe) => {
//                 if let Some(mbe) = mbe {
//                     let mbe = mbe.to_string();
//                     write!(f, "button:{}", mbe)
//                 } else {
//                     write!(f, "button")
//                 }
//             }
//             MouseEvent::Relative => write!(f, "relative"),
//         }
//     }
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// enum ButtonEvType {
//     B1p,
//     B1r,
//     B2p,
//     B2r,
//     B3p,
//     B3r,
// }

// impl Display for ButtonEvType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ButtonEvType::B1p => write!(f, "b1p"),
//             ButtonEvType::B1r => write!(f, "b1r"),
//             ButtonEvType::B2p => write!(f, "b2p"),
//             ButtonEvType::B2r => write!(f, "b2r"),
//             ButtonEvType::B3p => write!(f, "b3p"),
//             ButtonEvType::B3r => write!(f, "b3r"),
//         }
//     }
// }

// // ```ignore
// // /**
// //  * atspi_event_listener_register:
// //  * @listener: The #AtspiEventListener to register against an event type.
// //  * @event_type: a character string indicating the type of events for which
// //  *            notification is requested.  Format is
// //  *            EventClass:major_type:minor_type:detail
// //  *            where all subfields other than EventClass are optional.
// //  *            EventClasses include "object", "window", "mouse",
// //  *            and toolkit events (e.g. "Gtk", "AWT").
// //  *            Examples: "focus:", "Gtk:GtkWidget:button_press_event".
// //  *
// //  * Adds an in-process callback function to an existing #AtspiEventListener.
// //  *
// //  * Legal object event types:
// //  *
// //  *    (property change events)
// //  *
// //  *            object:property-change
// //  *            object:property-change:accessible-name
// //  *            object:property-change:accessible-description
// //  *            object:property-change:accessible-parent
// //  *            object:property-change:accessible-value
// //  *            object:property-change:accessible-role
// //  *            object:property-change:accessible-table-caption
// //  *            object:property-change:accessible-table-column-description
// //  *            object:property-change:accessible-table-column-header
// //  *            object:property-change:accessible-table-row-description
// //  *            object:property-change:accessible-table-row-header
// //  *            object:property-change:accessible-table-summary
// //  *
// //  *    (other object events)
// //  *
// //  *            object:state-changed
// //  *            object:children-changed
// //  *            object:visible-data-changed
// //  *            object:selection-changed
// //  *            object:text-selection-changed
// //  *            object:text-changed
// //  *            object:text-caret-moved
// //  *            object:row-inserted
// //  *            object:row-reordered
// //  *            object:row-deleted
// //  *            object:column-inserted
// //  *            object:column-reordered
// //  *            object:column-deleted
// //  *            object:model-changed
// //  *            object:active-descendant-changed
// //  *
// //  *  (screen reader events)
// // *             screen-reader:region-changed
// //  *
// //  *  (window events)
// //  *
// //  *            window:minimize
// //  *            window:maximize
// //  *            window:restore
// //  *            window:close
// //  *            window:create
// //  *            window:reparent
// //  *            window:desktop-create
// //  *            window:desktop-destroy
// //  *            window:activate
// //  *            window:deactivate
// //  *            window:raise
// //  *            window:lower
// //  *            window:move
// //  *            window:resize
// //  *            window:shade
// //  *            window:unshade
// //  *            window:restyle
// //  *
// //  *  (other events)
// //  *
// //  *            focus:
// //  *            mouse:abs
// //  *            mouse:rel
// //  *            mouse:b1p
// //  *            mouse:b1r
// //  *            mouse:b2p
// //  *            mouse:b2r
// //  *            mouse:b3p
// //  *            mouse:b3r
// //  *
// //  * NOTE: this character string may be UTF-8, but should not contain byte
// //  * value 56
// //  *            (ascii ':'), except as a delimiter, since non-UTF-8 string
// //  *            delimiting functions are used internally.
// //  *            In general, listening to
// //  *            toolkit-specific events is not recommended.
// //  *
// //  * Currently, object:text-reading-position needs to be specified explicitly
// //  * (it is not implied by object:text), since it is generated by the screen
// //  * reader and is thus a special case internally.
// //  *
// //  * Returns: #TRUE if successful, otherwise #FALSE.
// // ```

// use std::fmt::Display;

use std::{error::Error, time::Duration};

use dbus::{arg::messageitem::MessageItem, blocking::Connection, Message};

const DBUS_SERVICE_NAME: &'static str = "org.gnome.Shell";
const DBUS_INTERFACE: &'static str = "org.gnome.Shell";
const DBUS_OBJECT_PATH: &'static str = "/org/gnome/Shell";
const TIMEOUT: Duration = Duration::from_secs(2);

pub struct GnomeShell {
    connection: Connection,
}

impl GnomeShell {
    pub fn new_session() -> Self {
        let connection = Connection::new_session().unwrap();
        Self { connection }
    }

    pub fn show_osd(&self, icon: String, label: Option<String>, level: Option<f64>) -> Result<(), Box<dyn Error>> {
        let message = Message::new_method_call(
            DBUS_SERVICE_NAME,
            DBUS_OBJECT_PATH,
            DBUS_INTERFACE,
            "ShowOSD",
        )?;
        let args: Vec<(MessageItem, MessageItem)> = if label.is_none() && level.is_none() {
            vec![(
                MessageItem::Str(String::from("icon")),
                MessageItem::Variant(Box::new(MessageItem::Str(icon))),
            )]
        } else if label.is_none() && level.is_some() {
            vec![
                (
                    MessageItem::Str(String::from("icon")),
                    MessageItem::Variant(Box::new(MessageItem::Str(icon))),
                ),
                (
                    MessageItem::Str(String::from("level")),
                    MessageItem::Variant(Box::new(MessageItem::Double(level.unwrap()))),
                ),
            ]
        } else if level.is_none() && label.is_some() {
            vec![
                (
                    MessageItem::Str(String::from("icon")),
                    MessageItem::Variant(Box::new(MessageItem::Str(icon))),
                ),
                (
                    MessageItem::Str(String::from("label")),
                    MessageItem::Variant(Box::new(MessageItem::Str(label.unwrap()))),
                ),
            ]
        } else {
            vec![
                (
                    MessageItem::Str(String::from("icon")),
                    MessageItem::Variant(Box::new(MessageItem::Str(icon))),
                ),
                (
                    MessageItem::Str(String::from("label")),
                    MessageItem::Variant(Box::new(MessageItem::Str(label.unwrap()))),
                ),
                (
                    MessageItem::Str(String::from("level")),
                    MessageItem::Variant(Box::new(MessageItem::Double(level.unwrap()))),
                ),
            ]
        };

        self.connection
            .channel()
            .send_with_reply_and_block(message.append1(MessageItem::new_dict(args).unwrap()), TIMEOUT)?;
        Ok(())
    }
}

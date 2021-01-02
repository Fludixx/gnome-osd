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

    pub fn show_osd(
        &self,
        icon: String,
        label: Option<String>,
        level: Option<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let message = Message::new_method_call(
            DBUS_SERVICE_NAME,
            DBUS_OBJECT_PATH,
            DBUS_INTERFACE,
            "ShowOSD",
        )?;
        let mut args: Vec<(MessageItem, MessageItem)> = Vec::new();
        args.push((
            MessageItem::Str(String::from("icon")),
            MessageItem::Variant(Box::new(MessageItem::Str(icon))),
        ));
        if label.is_some() {
            args.push((
                MessageItem::Str(String::from("label")),
                MessageItem::Variant(Box::new(MessageItem::Str(label.unwrap()))),
            ))
        }
        if level.is_some() {
            args.push((
                MessageItem::Str(String::from("level")),
                MessageItem::Variant(Box::new(MessageItem::Double(level.unwrap()))),
            ))
        }
        self.connection.channel().send_with_reply_and_block(
            message.append1(MessageItem::new_dict(args).unwrap()),
            TIMEOUT,
        )?;
        Ok(())
    }
}

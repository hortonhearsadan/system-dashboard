use crate::fmt::get_session_name;
use crate::system::SystemInfo;
use gtk::prelude::*;
use gtk::{Align, Orientation, Widget};

pub struct HeaderView {
    container: gtk::Box,
    session_info: gtk::Label,
    os_info: gtk::Label,
    session_time: gtk::Label,
}

impl HeaderView {
    pub(crate) fn new() -> Self {
        let container = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .build();
        let session_info = create_label("session_info");
        let os_info = create_label("os_info");
        let session_time = create_label("session_time");

        container.pack_start(&session_info, false, false, 0);
        container.pack_start(&os_info, false, false, 0);
        container.pack_start(&session_time, false, false, 0);

        // Dummy Data
        session_info.set_label("User@Host");
        os_info.set_label("Ubuntu 20.04");
        session_time.set_label("The Singularity");

        Self {
            container,
            session_info,
            os_info,
            session_time,
        }
    }

    pub fn update(&self, system_info: &SystemInfo) {
        self.session_info
            .set_label(&get_session_name(&system_info.user, &system_info.host));
        self.session_time.set_label(&*system_info.datetime);
        self.os_info.set_label(&*system_info.os);

    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn create_label(name: &str) -> gtk::Label {
    gtk::LabelBuilder::new()
        .name(name)
        .halign(Align::Center)
        .build()
}

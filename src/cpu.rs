use crate::fmt::{Celcify, Percentify};
use crate::system::SystemInfo;
use gdk::prelude::IsA;
use gtk::{Align, GridExt, LabelExt, StyleContextExt, Widget, WidgetExt};

pub struct CPUView {
    container: gtk::Grid,
    cpu_lbl: gtk::Label,
    cpu_usage: gtk::Label,
    cpu_temp: gtk::Label,
}

impl CPUView {
    pub fn new() -> Self {
        let cpu_lbl = create_label("CPU");

        let cpu_usage = create_label("cpu_usage");
        let cpu_temp = create_label("cpu_temp");

        let container = gtk::GridBuilder::new()
            .row_spacing(12)
            .column_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.get_style_context().add_class("cpu");
        container.attach(&cpu_lbl, 0, 0, 1, 1);
        container.attach(&cpu_usage, 0, 1, 1, 1);
        container.attach(&cpu_temp, 1, 1, 1, 1);

        cpu_usage.set_text(&*100u8.as_percentage());
        cpu_temp.set_text(&*100u8.as_celcius());

        Self {
            container,
            cpu_lbl,
            cpu_usage,
            cpu_temp,
        }
    }

    pub fn update(&self, _system_info: &SystemInfo) {
        1;
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn create_label(label: &str) -> gtk::Label {
    gtk::LabelBuilder::new()
        .label(label)
        .halign(Align::Start)
        .build()
}

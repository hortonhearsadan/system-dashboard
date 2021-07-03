use crate::fmt::{create_label, Celcify, Name, Percentify};
use crate::gpu::update_usage;
use crate::system::SystemInfo;
use cairo::{Context, Format, ImageSurface};
use gdk::prelude::IsA;
use gtk::{Align, BoxExt, GridExt, LabelExt, Orientation, StyleContextExt, Widget, WidgetExt};

pub struct CPUView {
    container: gtk::Grid,
    cpu_usage: gtk::Label,
    cpu_temp: gtk::Label,
    cpu_name: gtk::Label,
    cpu_usage_arc: Context,
}

impl CPUView {
    pub fn new() -> Self {
        let cpu_name = create_label("cpu_name", Align::Start);
        cpu_name.set_text("XXXXXXX");

        let cpu_usage = create_label("cpu_usage", Align::Center);
        let cpu_temp = create_label("cpu_temp", Align::Start);

        let arc_box = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 300, 250).expect("Can't create surface");
        let cr = Context::new(&surface);

        let arc = gtk::ImageBuilder::new().surface(&surface).build();

        arc_box.pack_start(&arc, false, false, 0);

        let container = gtk::GridBuilder::new()
            .row_spacing(12)
            .column_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        container.get_style_context().add_class("cpu");
        container.attach(&cpu_name, 0, 0, 1, 1);
        container.attach(&cpu_usage, 0, 1, 1, 1);
        container.attach(&cpu_temp, 1, 1, 1, 1);
        container.attach(&arc_box, 0, 1, 1, 1);

        cpu_usage.set_text(&*100u8.as_percentage());
        cpu_temp.set_text(&*100u8.as_celcius());

        Self {
            container,
            cpu_usage,
            cpu_temp,
            cpu_name,
            cpu_usage_arc: cr,
        }
    }

    pub fn update(&self, system_info: &SystemInfo) {
        self.cpu_name
            .set_text(&system_info.cpu_name.trim().as_field_name("CPU"));
        self.cpu_temp.set_text(&system_info.cpu_temp.as_celcius());
        self.cpu_usage
            .set_text(&(system_info.cpu_usage as u8).as_percentage());
        update_usage(&self.cpu_usage_arc, system_info.cpu_usage as u8);
        self.container.queue_draw()
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

use crate::fmt::{Celcify, Percentify};
use crate::system::SystemInfo;
use cairo::{Context, Format, ImageSurface};
use gdk::prelude::IsA;
use gtk::{Align, BoxExt, GridExt, LabelExt, Orientation, StyleContextExt, Widget, WidgetExt};
use log::info;
use std::f64::consts::PI;

pub struct GPUView {
    container: gtk::Grid,
    gpu_lbl: gtk::Label,
    gpu_name: gtk::Label,
    gpu_usage: gtk::Label,
    gpu_usage_arc: Context,
    gpu_temp: gtk::Label,
}

impl GPUView {
    pub fn new() -> Self {
        let gpu_lbl = create_label("GPU :");
        let gpu_name = create_label("XXXXX");

        let gpu_usage = create_label("gpu_usage");
        let gpu_temp = create_label("gpu_temp");

        let arc_box = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 350, 600).expect("Can't create surface");
        let cr = Context::new(&surface);

        let arc = gtk::ImageBuilder::new().surface(&surface).build();

        let container = gtk::GridBuilder::new()
            .row_spacing(12)
            .column_spacing(12)
            .vexpand(true)
            .hexpand(true)
            .build();

        arc_box.pack_start(&arc, false, false, 0);

        container.get_style_context().add_class("gpu");
        container.attach(&gpu_lbl, 0, 0, 1, 1);
        container.attach(&gpu_name, 1, 0, 1, 1);
        container.attach(&gpu_usage, 0, 1, 1, 1);
        container.attach(&gpu_temp, 1, 1, 1, 1);
        container.attach(&arc_box, 0, 2, 1, 1);
        gpu_usage.set_text(&*100u8.as_percentage());
        gpu_temp.set_text(&*100u8.as_celcius());

        Self {
            container,
            gpu_usage_arc: cr,
            gpu_lbl,
            gpu_usage,
            gpu_temp,
            gpu_name,
        }
    }

    pub fn update(&self, system_info: &SystemInfo) {
        self.gpu_temp.set_text(&system_info.gpu_temp);
        self.gpu_usage.set_text(&system_info.gpu_usage);
        self.gpu_name.set_text(&system_info.gpu_name);
        let usages = &system_info.gpu_usage.replace("%", "");
        if let Ok(usage) = usages.trim().parse::<u8>() {
            update_usage(&self.gpu_usage_arc, usage);
            self.container.queue_draw()
        } else {
            info!("{:?}", usages.parse::<u8>());
        }
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

fn update_usage(ctx: &Context, usage: u8) {
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.paint();
    ctx.new_path();
    let start = -PI * 7.0 / 6.0;
    let end = PI / 6.0;
    ctx.arc(
        150.0,
        250.0,
        100.0,
        start,
        start + usage as f64 / 100.0 * (end - start),
    );
    ctx.set_line_width(60.0);
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.stroke()
}

fn create_label(label: &str) -> gtk::Label {
    gtk::LabelBuilder::new()
        .label(label)
        .halign(Align::Start)
        .build()
}

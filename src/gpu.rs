use crate::fmt::{create_label, Celcify, Name, Percentify};
use crate::system::SystemInfo;
use cairo::{Context, Format, ImageSurface};
use gdk::prelude::IsA;
use gtk::{Align, BoxExt, GridExt, LabelExt, Orientation, StyleContextExt, Widget, WidgetExt};
use std::f64::consts::PI;

pub struct GPUView {
    container: gtk::Grid,
    gpu_name: gtk::Label,
    gpu_usage: gtk::Label,
    gpu_usage_arc: Context,
    gpu_temp: gtk::Label,
    power_draw: gtk::Label,
    power_limit: gtk::Label,
    memory_used: gtk::Label,
    memory_total: gtk::Label,
}

impl GPUView {
    pub fn new() -> Self {
        let gpu_name = create_label("gpu_name", Align::Start);
        gpu_name.set_text("XXXXXX");

        let gpu_usage = create_label("gpu_usage", Align::Center);
        let gpu_temp = create_label("gpu_temp", Align::Start);

        let power_draw = create_label("power_draw", Align::Start);
        let power_limit = create_label("power_limit", Align::Start);
        let memory_used = create_label("memory_used", Align::Start);
        let memory_total = create_label("memory_total", Align::Start);

        let arc_box = gtk::BoxBuilder::new()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .build();

        let surface = ImageSurface::create(Format::ARgb32, 300, 250).expect("Can't create surface");
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
        container.attach(&gpu_name, 0, 0, 1, 1);
        container.attach(&gpu_usage, 0, 1, 1, 5);
        container.attach(&gpu_temp, 1, 1, 1, 5);

        container.attach(&power_draw, 2, 1, 1, 1);
        container.attach(&power_limit, 2, 2, 1, 1);
        container.attach(&memory_used, 2, 3, 1, 1);
        container.attach(&memory_total, 2, 4, 1, 1);
        container.attach(&arc_box, 0, 1, 1, 5);
        gpu_usage.set_text(&*100u8.as_percentage());
        gpu_temp.set_text(&*100u8.as_celcius());

        Self {
            container,
            gpu_usage_arc: cr,
            gpu_usage,
            gpu_temp,
            gpu_name,
            power_draw,
            power_limit,
            memory_used,
            memory_total,
        }
    }

    pub fn update(&self, system_info: &SystemInfo) {
        let gpu_info = &system_info.gpu_info;
        self.gpu_temp.set_text(&gpu_info.temperature.as_celcius());
        self.gpu_usage
            .set_text(&gpu_info.utilization.as_percentage());
        self.gpu_name
            .set_text(&gpu_info.name.as_long_field_name("GPU"));
        self.power_draw
            .set_text(&gpu_info.power_draw.as_field_name("Power Draw (W)"));
        self.power_limit
            .set_text(&gpu_info.power_limit.as_field_name("Power Limit (W)"));
        self.memory_used
            .set_text(&gpu_info.used_memory.as_field_name("Memory Used (MiB)"));
        self.memory_total
            .set_text(&gpu_info.total_memory.as_field_name("Memory Total (MiB)"));
        update_usage(&self.gpu_usage_arc, gpu_info.utilization);
        self.container.queue_draw()
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.container
    }
}

pub fn update_usage(ctx: &Context, usage: u8) {
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.paint();
    ctx.new_path();
    let start = -PI * 7.0 / 6.0;
    let end = PI / 6.0;
    ctx.arc(
        150.0,
        150.0,
        100.0,
        start,
        start + usage as f64 / 100.0 * (end - start),
    );
    ctx.set_line_width(60.0);
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.stroke()
}

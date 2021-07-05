use gtk::Align;
use std::fmt::Display;

pub fn get_session_name(user: &str, host: &str) -> String {
    format!("{}@{}", user, host)
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}

pub trait Percentify {
    fn as_percentage(&self) -> String;
}

impl<T: Display> Percentify for T {
    fn as_percentage(&self) -> String {
        format!("{}%", self)
    }
}

pub trait Celcify {
    fn as_celcius(&self) -> String;
}

impl<T: Display> Celcify for T {
    fn as_celcius(&self) -> String {
        format!("{:>3}C", self)
    }
}

pub fn create_label(label: &str, align: Align) -> gtk::Label {
    gtk::LabelBuilder::new()
        .name(label)
        .label(label)
        .halign(align)
        .build()
}

pub trait Name {
    fn as_long_field_name(&self, field: &str) -> String;
    fn as_field_name(&self, field: &str) -> String;
}

impl<T: Display> Name for T {
    fn as_long_field_name(&self, field: &str) -> String {
        format!("{} : {:<40}", field, self)
    }
    fn as_field_name(&self, field: &str) -> String {
        format!("{} : {}", field, self)
    }
}

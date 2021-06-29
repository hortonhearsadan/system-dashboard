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

impl Percentify for u8 {
    fn as_percentage(&self) -> String {
        format!("{}%", self)
    }
}

pub trait Celcify {
    fn as_celcius(&self) -> String;
}

impl Celcify for u8 {
    fn as_celcius(&self) -> String {
        format!("{}C", self)
    }
}

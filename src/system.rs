use crate::fmt::trim_newline;
use std::process::Command;

#[derive(Default)]
pub struct SystemInfo {
    pub(crate) user: String,
    pub(crate) host: String,
    pub datetime: String,
}

impl SystemInfo {
    pub fn update(&mut self) {
        if let Some(user) = get_user() {
            self.user = user
        }
        if let Some(host) = get_host() {
            self.host = host
        }
        if let Some(datetime) = get_datetime() {
            self.datetime = datetime
        }
    }
}

fn get_datetime() -> Option<String> {
    get_command_output("date")
}

fn get_user() -> Option<String> {
    get_command_output("whoami")
}

fn get_host() -> Option<String> {
    get_command_output("hostname")
}

fn get_command_output(command: &str) -> Option<String> {
    if let Ok(output) = Command::new(command).output() {
        let mut raw = output.stdout.to_string();
        trim_newline(&mut raw);
        Some(raw)
    } else {
        None
    }
}

pub trait Stringify {
    fn to_string(&self) -> String;
}

impl Stringify for Vec<u8> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self).parse().unwrap()
    }
}

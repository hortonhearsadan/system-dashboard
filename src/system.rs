use crate::fmt::trim_newline;
use std::process::Command;
use os_release::OsRelease;

#[derive(Default)]
pub struct SystemInfo {
    pub(crate) user: String,
    pub(crate) host: String,
    pub os: String,
    pub datetime: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system_info = Self::default();
        if let Some(user) = get_user() {
            system_info.user = user
        }
        if let Some(host) = get_host() {
            system_info.host = host
        }
        if let Some(os) = get_os() {
            system_info.os =os
        }

        system_info
    }


    pub fn update(&mut self) {
        if let Some(datetime) = get_datetime() {
            self.datetime = datetime
        }
    }
}

fn get_os() -> Option<String> {
    if let Ok(osr) = OsRelease::new() {
        Some(osr.pretty_name)
    }
    else{
        None
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

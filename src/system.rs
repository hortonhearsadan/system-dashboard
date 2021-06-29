use crate::fmt::trim_newline;
use os_release::OsRelease;
use std::process::Command;

#[derive(Default)]
pub struct SystemInfo {
    pub(crate) user: String,
    pub(crate) host: String,
    pub os: String,
    pub datetime: String,
    pub gpu_temp: String,
    pub gpu_usage: String,
    pub gpu_name: String,
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
            system_info.os = os
        }
        if let Some(gpu_name) = get_gpu_name() {
            system_info.gpu_name = gpu_name
        }

        system_info
    }

    pub fn update(&mut self) {
        if let Some(datetime) = get_datetime() {
            self.datetime = datetime
        }
        if let Some(gpu_temp) = get_gpu_temp() {
            self.gpu_temp = gpu_temp
        }
        if let Some(gpu_usage) = get_gpu_usage() {
            self.gpu_usage = gpu_usage
        }
    }
}

fn get_gpu_name() -> Option<String> {
    let args = ["--query-gpu=name", "--format=csv,noheader"];
    get_command_output("nvidia-smi", Some(&args))
}

fn get_os() -> Option<String> {
    if let Ok(osr) = OsRelease::new() {
        Some(osr.pretty_name)
    } else {
        None
    }
}

fn get_gpu_temp() -> Option<String> {
    let args = ["--query-gpu=temperature.gpu", "--format=csv,noheader"];
    get_command_output("nvidia-smi", Some(&args))
}

fn get_gpu_usage() -> Option<String> {
    let args = ["--query-gpu=utilization.gpu", "--format=csv,noheader"];
    get_command_output("nvidia-smi", Some(&args))
}

fn get_datetime() -> Option<String> {
    get_command_output("date", None)
}

fn get_user() -> Option<String> {
    get_command_output("whoami", None)
}

fn get_host() -> Option<String> {
    get_command_output("hostname", None)
}

fn get_command_output(command: &str, args: Option<&[&str]>) -> Option<String> {
    let mut c = Command::new(command);
    if let Some(arguments) = args {
        c.args(arguments);
    }
    if let Ok(output) = c.output() {
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

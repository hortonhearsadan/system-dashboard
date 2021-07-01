use crate::fmt::trim_newline;
use log::info;
use os_release::OsRelease;
use regex::Regex;
use std::process::Command;
use systemstat::{Platform, System};

#[derive(Default)]
pub struct SystemInfo {
    pub(crate) user: String,
    pub(crate) host: String,
    pub os: String,
    pub datetime: String,
    pub gpu_temp: String,
    pub gpu_usage: String,
    pub gpu_name: String,
    pub cpu_temp: f32,
    pub cpu_usage: f32,
    pub cpu_name: String,
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
        if let Some(cpu_name) = get_cpu_name() {
            system_info.cpu_name = cpu_name;
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
        let cpu_data = System::new();
        if let Ok(cpu_temp) = cpu_data.cpu_temp() {
            self.cpu_temp = cpu_temp
        }
        if let Ok(_cpu_usage) = cpu_data.cpu_load_aggregate() {
            // let cpu = cpu_usage.done().unwrap();
            self.cpu_usage = 70.5;
            info!("{}", self.cpu_usage);
        }
    }
}

fn get_cpu_name() -> Option<String> {
    let output = get_command_output("lscpu", None);
    if let Some(cpu) = output {
        parse_cpu_name(cpu)
    } else {
        None
    }
}

fn parse_cpu_name(cpu_data: String) -> Option<String> {
    let re = Regex::new(r"AMD Ry.*?\n").unwrap();
    let cpu = re.find(&cpu_data).unwrap().as_str();
    Some(cpu.to_string())
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

use crate::fmt::trim_newline;
use csv::Reader;
use log::info;
use os_release::OsRelease;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::process::Command;
use systemstat::{Platform, System};

pub struct SystemInfo {
    pub(crate) user: String,
    pub(crate) host: String,
    pub os: String,
    pub datetime: String,
    pub cpu_usage: f32,
    pub cpu_name: String,
    pub system: System,
    pub gpu_info: GPUInfo,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system_info = Self {
            user: "".to_string(),
            host: "".to_string(),
            os: "".to_string(),
            datetime: "".to_string(),
            cpu_usage: 0.0,
            cpu_name: "".to_string(),
            system: System::new(),
            gpu_info: Default::default(),
        };

        if let Ok(gpu_info) = get_gpu_info() {
            system_info.gpu_info = gpu_info;
            info!("yooo {:?} ", system_info.gpu_info)
        }

        if let Some(cpu_name) = get_cpu_name() {
            system_info.cpu_name = cpu_name;
        }
        if let Some(user) = get_user() {
            system_info.user = user
        }
        if let Some(host) = get_host() {
            system_info.host = host
        }
        if let Some(os) = get_os() {
            system_info.os = os
        }
        system_info.system = System::new();
        system_info
    }

    pub fn update(&mut self) {
        if let Some(datetime) = get_datetime() {
            self.datetime = datetime
        }
        if let Ok(gpu_info) = get_gpu_info() {
            self.gpu_info = gpu_info;
        }

        if let Ok(_cpu_usage) = self.system.cpu_load_aggregate() {
            // let cpu = cpu_usage.done().unwrap();
            self.cpu_usage = 70.5;
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Default)]
pub struct GPUInfo {
    pub name: String,
    #[serde(rename = "temperature.gpu")]
    pub(crate) temperature: u8,
    #[serde(rename = "utilization.gpu [%]")]
    pub utilization: u8,
    #[serde(rename = "memory.total [MiB]")]
    pub total_memory: u32,
    #[serde(rename = "memory.used [MiB]")]
    pub used_memory: u32,
}

fn get_gpu_info() -> Result<GPUInfo, Box<dyn Error>> {
    let args = [
        "--query-gpu=name,temperature.gpu,utilization.gpu,memory.total,memory.used",
        "--format=csv,nounits",
    ];
    if let Some(data) = get_command_output("nvidia-smi", Some(&args)) {
        let data = data.replace(", ", ",");
        let mut rdr = Reader::from_reader(data.as_bytes());
        let mut iter = rdr.deserialize();
        info!("passed_iter");
        if let Some(result) = iter.next() {
            info!("unrip2");
            let gpu_info: GPUInfo = result?;
            info!("unrip");

            Ok(gpu_info)
        } else {
            info!("rip");
            Ok(GPUInfo::default())
        }
    } else {
        Ok(GPUInfo::default())
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

fn get_os() -> Option<String> {
    if let Ok(osr) = OsRelease::new() {
        Some(osr.pretty_name)
    } else {
        None
    }
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

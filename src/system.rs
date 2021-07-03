use crate::fmt::trim_newline;
use csv::{Reader, ReaderBuilder};
use log::info;
use regex::Regex;
use serde::Deserialize;
use std::cmp::min;
use std::error::Error;
use std::process::Command;
use sysinfo::System;
use sysinfo::{ProcessorExt, SystemExt};
use toml::Value;

#[derive(Default)]
pub struct SystemInfo {
    pub(crate) user: String,
    pub(crate) host: String,
    pub os: String,
    pub datetime: String,
    pub cpu_usage: u64,
    pub cpu_temp: u8,
    pub cpu_name: String,
    pub gpu_info: GPUInfo,
    pub cpu_usage_info: CPUUsageInfo,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system_info = Self::default();

        if let Ok(gpu_info) = get_gpu_info() {
            system_info.gpu_info = gpu_info;
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
        system_info
    }

    pub fn update(&mut self) {
        if let Some(datetime) = get_datetime() {
            self.datetime = datetime
        }
        if let Ok(gpu_info) = get_gpu_info() {
            self.gpu_info = gpu_info;
        }

        if let Some(cpu_temp) = get_cpu_temp() {
            self.cpu_temp = cpu_temp as u8
        }

        if let Ok(cpu_time) = get_cpu_time() {
            self.cpu_usage_info.update(cpu_time);
            self.cpu_usage = self.cpu_usage_info.get_cpu_usage();
        }
        // let cpu_usage = self.system.global_processor_info().cpu_usage();
    }
}

fn get_cpu_time() -> Result<CPUTime, Box<dyn Error>> {
    let args = ["/proc/stat", "-n", "1"];
    if let Some(output) = get_command_output("head", Some(&args)) {
        let output = output.replace("cpu ", "");
        let output = output.trim().to_string();
        let output = output.replace(" ", ",");
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(output.as_bytes());
        let mut iter = rdr.deserialize();
        if let Some(result) = iter.next() {
            let cpu_time: CPUTime = result?;
            return Ok(cpu_time);
        } else {
            return Ok(CPUTime::default());
        }
    }
    Ok(CPUTime::default())
}

#[derive(Default, Copy, Clone, Deserialize, Debug)]
pub struct CPUTime {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    _guest: u64,
    _guest_nice: u64,
}

impl CPUTime {
    fn work_time(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq + self.steal
    }

    fn total_time(&self) -> u64 {
        self.work_time() + self.idle + self.iowait
    }
    pub fn set(
        &mut self,
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
        guest: u64,
        guest_nice: u64,
    ) {
        self.user = user;
        self.nice = nice;
        self.system = system;
        self.idle = idle;
        self.iowait = iowait;
        self.irq = irq;
        self.softirq = softirq;
        self.steal = steal;
        self._guest = guest;
        self._guest_nice = guest_nice;
    }
}

#[derive(Default)]
pub struct CPUUsageInfo {
    old: CPUTime,
    new: CPUTime,
}

impl CPUUsageInfo {
    fn get_cpu_usage(&self) -> u64 {
        let total = min(self.old.total_time(), self.new.total_time());
        let work = min(self.new.work_time(), self.old.work_time());

        if total == 0 {
            0
        } else {
            info!("{:?}\n{:?}, {}", total, work, work / total);
            min(work * 100 / total, 100)
        }
    }

    fn update(&mut self, cpu_time: CPUTime) {
        self.old = self.new;
        self.new = cpu_time;
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
        if let Some(result) = iter.next() {
            let gpu_info: GPUInfo = result?;

            Ok(gpu_info)
        } else {
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

fn get_cpu_temp() -> Option<f32> {
    let args = ["/sys/class/thermal/thermal_zone0/temp"];
    get_command_output("cat", Some(&args))
        .map(|output| output.trim().parse::<f32>().unwrap() / 1000.)
}

fn parse_cpu_name(cpu_data: String) -> Option<String> {
    let re = Regex::new(r"Model name:.*?\n").unwrap();
    let cpu = re.find(&cpu_data).unwrap().as_str();
    let cpu = cpu.replace("Model name:", "");
    Some(cpu.trim().to_string())
}

fn get_os() -> Option<String> {
    let args = ["/etc/os-release"];

    let mut output = get_command_output("cat", Some(&args)).unwrap();
    if !output.ends_with('\n') {
        output.push('\n')
    }
    let output = output.replace("=", "=\"");
    let output = output.replace("\n", "\"\n");
    let output = output.replace("\"\"", "\"");
    let name = output.parse::<Value>().unwrap()["PRETTY_NAME"].to_string();
    let os_name = name.replace("\"", "");
    Some(os_name)
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

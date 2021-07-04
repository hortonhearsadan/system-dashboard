use crate::fmt::trim_newline;
use csv::{Reader, ReaderBuilder};
use log::info;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::process::Command;
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
    pub memory_info: MemInfo,
    pub cpu_freq: f32,
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

        if let Some(mem_info) = get_memory_info() {
            self.memory_info = mem_info
        }

        if let Some(cpu_freq) = get_cpu_freq() {
            self.cpu_freq = cpu_freq
        }
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
}

#[derive(Default)]
pub struct CPUUsageInfo {
    old: CPUTime,
    new: CPUTime,
}

impl CPUUsageInfo {
    fn get_cpu_usage(&self) -> u64 {
        let total = fudge(self.new.total_time(), self.old.total_time());
        let work = fudge(self.new.work_time(), self.old.work_time());

        if total == 0. {
            0
        } else {
            let usage = work / total * 100.;
            if usage > 100. {
                100
            } else {
                usage as u64
            }
        }
    }

    fn update(&mut self, cpu_time: CPUTime) {
        self.old = self.new;
        self.new = cpu_time;
    }
}

fn fudge(a: u64, b: u64) -> f32 {
    if a > b {
        (a - b) as f32
    } else {
        1.
    }
}

#[derive(Debug, Deserialize, PartialEq, Default)]
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
    #[serde(rename = "power.draw [W]")]
    pub power_draw: f32,
    #[serde(rename = "power.limit [W]")]
    pub power_limit: f32,
}

fn get_gpu_info() -> Result<GPUInfo, Box<dyn Error>> {
    let args = [
        "--query-gpu=name,temperature.gpu,utilization.gpu,memory.total,memory.used,power.draw,power.limit",
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
fn get_cpu_freq() -> Option<f32> {
    let args = ["/proc/cpuinfo"];
    get_command_output("cat", Some(&args)).map(|output| parse_cpu_freq(output).unwrap())
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

fn parse_cpu_freq(cpu_data: String) -> Option<f32> {
    let re = Regex::new(r"cpu MHz.*?\n").unwrap();
    let freqs = re.find_iter(&cpu_data);
    let cpu_freqs = freqs
        .into_iter()
        .map(|m| m.as_str().replace("cpu MHz\t\t:", ""))
        .collect::<Vec<String>>();
    info!("{:?}", &cpu_freqs);
    Some(
        cpu_freqs
            .iter()
            .map(|c| c.trim().parse::<f32>().unwrap())
            .sum::<f32>()
            / cpu_freqs.len() as f32,
    )
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

#[derive(Default)]
pub struct MemInfo {
    pub(crate) total: u32,
    available: u32,
}

impl MemInfo {
    pub(crate) fn used_mib(&self) -> u32 {
        (self.total - self.available) / 1049
    }
    pub fn total_mib(&self) -> u32 {
        self.total / 1049
    }
}

fn get_memory_info() -> Option<MemInfo> {
    let args = ["/proc/meminfo", "-n", "3"];

    if let Some(output) = get_command_output("head", Some(&args)) {
        let output = output.replace("kB", "");
        let output = output.replace("\n", ",");
        let output = output.replace("MemTotal:", "");
        let output = output.replace("MemFree:", "");
        let output = output.replace("MemAvailable:", "");
        let data = output.split(',').collect::<Vec<&str>>();
        let mems = &data
            .iter()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mem_info = MemInfo {
            total: mems[0],
            available: mems[2],
        };
        Some(mem_info)
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

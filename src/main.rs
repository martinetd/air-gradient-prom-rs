use clap::Parser;
use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'l', long, default_value = "0.0.0.0:9090")]
    listen: SocketAddr,

    airgradient_ip: String,
}

// comments from https://github.com/airgradienthq/arduino/blob/master/docs/local-server.md
#[derive(Debug, Serialize, Deserialize)]
struct Response {
    /// Serial Number of the monitor
    #[serde(rename = "serialno")]
    serial_no: String,
    /// WiFi signal strength
    wifi: i32,
    /// PM1 in ug/m3 (atmospheric environment)
    pm01: f64,
    /// PM2.5 in ug/m3 (atmospheric environment)
    pm02: f64,
    /// PM10 in ug/m3 (atmospheric environment)
    pm10: f64,
    /// PM2.5 in ug/m3 with correction applied
    #[serde(rename = "pm02Compensated")]
    pm02_compensated: f64,
    /// PM1 in ug/m3 (standard particle)
    #[serde(rename = "pm01Standard")]
    pm01_standard: f64,
    /// PM2.5 in ug/m3 (standard particle)
    #[serde(rename = "pm02Standard")]
    pm02_standard: f64,
    /// PM10 in ug/m3 (stndard particule)
    #[serde(rename = "pm10Standard")]
    pm10_standard: f64,
    /// CO2 in ppm
    rco2: f64,
    /// Particle count per dL
    #[serde(rename = "pm003Count")]
    pm003_count: f64,
    /// Particle count per dL
    #[serde(rename = "pm005Count")]
    pm005_count: f64,
    /// Particle count per dL
    #[serde(rename = "pm01Count")]
    pm01_count: f64,
    /// Particle count per dL
    #[serde(rename = "pm02Count")]
    pm02_count: f64,
    /// Particle count per dL
    #[serde(rename = "pm50Count")]
    pm50_count: f64,
    /// Particle count per dL
    #[serde(rename = "pm10Count")]
    pm10_count: f64,
    /// Temperature in Degrees Celsius
    atmp: f64,
    /// Temperature in Degrees Celsius with correction applied
    #[serde(rename = "atmpCompensated")]
    atmp_compensated: f64,
    /// Relative Humidity
    rhum: f64,
    /// Relative Humidity with correction applied
    #[serde(rename = "rhumCompensated")]
    rhum_compensated: f64,
    /// Senisiron VOC Index
    #[serde(rename = "tvocIndex")]
    tvoc_index: f64,
    /// VOC raw value
    #[serde(rename = "tvocRaw")]
    tvoc_raw: f64,
    /// Senisirion NOx Index
    #[serde(rename = "noxIndex")]
    nox_index: f64,
    /// NOx raw value
    #[serde(rename = "noxRaw")]
    nox_raw: f64,
    /// Counts every measurement cycle. Low boot counts indicate restarts.
    boot: i32,
    /// Same as boot property. Required for Home Assistant compatibility. Will be depreciated.
    #[serde(rename = "bootCount")]
    boot_count: i32,
    /// Current configuration of the LED mode
    #[serde(rename = "ledMode")]
    led_mode: String,
    /// Current firmware version
    firmware: String,
    /// Current model name
    model: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let builder = PrometheusBuilder::new();
    builder
        // If an account disappears for 3 hours, it's probably gone.
        .idle_timeout(MetricKindMask::ALL, Some(Duration::from_secs(60 * 60 * 3)))
        .with_http_listener(args.listen)
        .install()
        .expect("Failed to install Prometheus recorder");

    describe_gauge!("airgradient_wifi", "WiFi signal strength.");
    describe_gauge!(
        "airgradient_pm01",
        "PM1 in ug/m3 (atmospheric environment)."
    );
    describe_gauge!(
        "airgradient_pm02",
        "PM2.5 in ug/m3 (atmospheric environment)."
    );
    describe_gauge!(
        "airgradient_pm10",
        "PM10 in ug/m3 (atmospheric environment)."
    );
    describe_gauge!(
        "airgradient_pm02_compensated",
        "PM2.5 in ug/m3 with correction applied."
    );
    describe_gauge!(
        "airgradient_pm01_standard",
        "PM1 in ug/m3 (standard particle)."
    );
    describe_gauge!(
        "airgradient_pm02_standard",
        "PM2.5 in ug/m3 (standard particle)."
    );
    describe_gauge!(
        "airgradient_pm10_standard",
        "PM10 in ug/m3 (stndard particule)."
    );
    describe_gauge!("airgradient_rco2", "CO2 in ppm.");
    describe_gauge!("airgradient_pm003_count", "Particle count per dL.");
    describe_gauge!("airgradient_pm005_count", "Particle count per dL.");
    describe_gauge!("airgradient_pm01_count", "Particle count per dL.");
    describe_gauge!("airgradient_pm02_count", "Particle count per dL.");
    describe_gauge!("airgradient_pm50_count", "Particle count per dL.");
    describe_gauge!("airgradient_pm10_count", "Particle count per dL.");
    describe_gauge!("airgradient_atmp", "Temperature in Degrees Celsius.");
    describe_gauge!(
        "airgradient_atmp_compensated",
        "Temperature in Degrees Celsius with correction applied."
    );
    describe_gauge!("airgradient_rhum", "Relative Humidity.");
    describe_gauge!(
        "airgradient_rhum_compensated",
        "Relative Humidity with correction applied."
    );
    describe_gauge!("airgradient_tvoc_index", "Senisiron VOC Index.");
    describe_gauge!("airgradient_tvoc_raw", "VOC raw value.");
    describe_gauge!("airgradient_nox_index", "Senisirion NOx Index.");
    describe_gauge!("airgradient_nox_raw", "NOx raw value.");
    describe_gauge!(
        "airgradient_boot",
        "Counts every measurement cycle. Low boot counts indicate restarts."
    );

    let wifi = gauge!("airgradient_wifi");
    let pm01 = gauge!("airgradient_pm01");
    let pm02 = gauge!("airgradient_pm02");
    let pm10 = gauge!("airgradient_pm10");
    let pm02_compensated = gauge!("airgradient_pm02_compensated");
    let pm01_standard = gauge!("airgradient_pm01_standard");
    let pm02_standard = gauge!("airgradient_pm02_standard");
    let pm10_standard = gauge!("airgradient_pm10_standard");
    let rco2 = gauge!("airgradient_rco2");
    let pm003_count = gauge!("airgradient_pm003_count");
    let pm005_count = gauge!("airgradient_pm005_count");
    let pm01_count = gauge!("airgradient_pm01_count");
    let pm02_count = gauge!("airgradient_pm02_count");
    let pm50_count = gauge!("airgradient_pm50_count");
    let pm10_count = gauge!("airgradient_pm10_count");
    let atmp = gauge!("airgradient_atmp");
    let atmp_compensated = gauge!("airgradient_atmp_compensated");
    let rhum = gauge!("airgradient_rhum");
    let rhum_compensated = gauge!("airgradient_rhum_compensated");
    let tvoc_index = gauge!("airgradient_tvoc_index");
    let tvoc_raw = gauge!("airgradient_tvoc_raw");
    let nox_index = gauge!("airgradient_nox_index");
    let nox_raw = gauge!("airgradient_nox_raw");
    let boot = gauge!("airgradient_boot");

    let client = reqwest::blocking::Client::new();

    println!("Started");

    loop {
        let response: Response = client
            .get(format!("http://{}/measures/current", args.airgradient_ip))
            .header("accept", "application/json")
            .send()
            .unwrap()
            .json()
            .unwrap();

        wifi.set(response.wifi);
        pm01.set(response.pm01);
        pm02.set(response.pm02);
        pm10.set(response.pm10);
        pm02_compensated.set(response.pm02_compensated);
        pm01_standard.set(response.pm01_standard);
        pm02_standard.set(response.pm02_standard);
        pm10_standard.set(response.pm10_standard);
        rco2.set(response.rco2);
        pm003_count.set(response.pm003_count);
        pm005_count.set(response.pm005_count);
        pm01_count.set(response.pm01_count);
        pm02_count.set(response.pm02_count);
        pm50_count.set(response.pm50_count);
        pm10_count.set(response.pm10_count);
        atmp.set(response.atmp);
        atmp_compensated.set(response.atmp_compensated);
        rhum.set(response.rhum);
        rhum_compensated.set(response.rhum_compensated);
        tvoc_index.set(response.tvoc_index);
        tvoc_raw.set(response.tvoc_raw);
        nox_index.set(response.nox_index);
        nox_raw.set(response.nox_raw);
        boot.set(response.boot);

        thread::sleep(Duration::from_secs(60));
    }
}

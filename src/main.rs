use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{env, thread};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    /// WiFi signal strength
    wifi: i32,
    /// Serial Number of the monitor
    #[serde(rename = "serialno")]
    serial_no: String,
    /// CO2 in ppm
    rco2: f64,
    /// PM1 in ug/m3
    pm01: f64,
    /// PM2.5 in ug/m3
    pm02: f64,
    /// PM10 in ug/m3
    pm10: f64,
    /// Particle count per dL
    #[serde(rename = "pm003Count")]
    pm003_count: f64,
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
    /// PM2.5 in ug/m3 with correction applied (from fw version 3.1.4 onwards)
    #[serde(rename = "pm02Compensated")]
    pm02_compensated: f64,
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

    let builder = PrometheusBuilder::new();
    builder
        // If an account disappears for 3 hours, it's probably gone.
        .idle_timeout(MetricKindMask::ALL, Some(Duration::from_secs(60 * 60 * 3)))
        .with_http_listener(([0, 0, 0, 0], 9090))
        .install()
        .expect("Failed to install Prometheus recorder");

    describe_gauge!("airgradient_wifi", "The current WiFi signal strength.");
    describe_gauge!("airgradient_rco2", "The current CO2 in ppm.");
    describe_gauge!("airgradient_pm01", "The current PM1 in ug/m3.");
    describe_gauge!("airgradient_pm02", "The current PM2.5 in ug/m3.");
    describe_gauge!("airgradient_pm10", "The current PM10 in ug/m3.");
    describe_gauge!(
        "airgradient_pm003_count",
        "The current particle count per dL."
    );
    describe_gauge!(
        "airgradient_atmp",
        "The current temperature in Degrees Celsius."
    );
    describe_gauge!(
        "airgradient_atmp_compensated",
        "The current temperature in Degrees Celsius with correction applied."
    );
    describe_gauge!("airgradient_rhum", "The current relative humidity.");
    describe_gauge!(
        "airgradient_rhum_compensated",
        "The current relative humidity with correction applied."
    );
    describe_gauge!(
        "airgradient_pm02_compensated",
        "The current PM2.5 in ug/m3 with correction applied."
    );
    describe_gauge!("airgradient_tvoc_index", "The current Senisiron VOC Index.");
    describe_gauge!("airgradient_tvoc_raw", "The current VOC raw value.");
    describe_gauge!("airgradient_nox_index", "The current Senisirion NOx Index.");
    describe_gauge!("airgradient_nox_raw", "The current NOx raw value.");
    describe_gauge!("airgradient_boot", "The current boot count.");
    describe_gauge!("airgradient_boot_count", "The current boot count.");

    let client = reqwest::blocking::Client::new();

    let wifi = gauge!("airgradient_wifi");
    let rco2 = gauge!("airgradient_rco2");
    let pm01 = gauge!("airgradient_pm01");
    let pm02 = gauge!("airgradient_pm02");
    let pm10 = gauge!("airgradient_pm10");
    let pm003_count = gauge!("airgradient_pm003_count");
    let atmp = gauge!("airgradient_atmp");
    let atmp_compensated = gauge!("airgradient_atmp_compensated");
    let rhum = gauge!("airgradient_rhum");
    let rhum_compensated = gauge!("airgradient_rhum_compensated");
    let pm02_compensated = gauge!("airgradient_pm02_compensated");
    let tvoc_index = gauge!("airgradient_tvoc_index");
    let tvoc_raw = gauge!("airgradient_tvoc_raw");
    let nox_index = gauge!("airgradient_nox_index");
    let nox_raw = gauge!("airgradient_nox_raw");
    let boot = gauge!("airgradient_boot");
    let boot_count = gauge!("airgradient_boot_count");

    println!("Started");

    loop {
        let response: Response = client
            .get(format!(
                "http://{}/measures/current",
                env::var("IP_ADDR").expect("IP_ADDR environment variable must be set")
            ))
            .header("accept", "application/json")
            .send()
            .unwrap()
            .json()
            .unwrap();

        wifi.set(response.wifi);
        rco2.set(response.rco2);
        pm01.set(response.pm01);
        pm02.set(response.pm02);
        pm10.set(response.pm10);
        pm003_count.set(response.pm003_count);
        atmp.set(response.atmp);
        atmp_compensated.set(response.atmp_compensated);
        rhum.set(response.rhum);
        rhum_compensated.set(response.rhum_compensated);
        pm02_compensated.set(response.pm02_compensated);
        tvoc_index.set(response.tvoc_index);
        tvoc_raw.set(response.tvoc_raw);
        nox_index.set(response.nox_index);
        nox_raw.set(response.nox_raw);
        boot.set(response.boot);
        boot_count.set(response.boot_count);

        thread::sleep(Duration::from_secs(60));
    }
}

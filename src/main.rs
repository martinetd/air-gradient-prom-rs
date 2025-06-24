use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{env, thread};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    /// WiFi signal strength
    wifi: i64,
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
    boot: i64,
    /// Same as boot property. Required for Home Assistant compatibility. Will be depreciated.
    #[serde(rename = "bootCount")]
    boot_count: i64,
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

        gauge!("airgradient_wifi", response.wifi as f64);
        gauge!("airgradient_rco2", response.rco2 as f64);
        gauge!("airgradient_pm01", response.pm01 as f64);
        gauge!("airgradient_pm02", response.pm02 as f64);
        gauge!("airgradient_pm10", response.pm10 as f64);
        gauge!("airgradient_pm003_count", response.pm003_count as f64);
        gauge!("airgradient_atmp", response.atmp);
        gauge!("airgradient_atmp_compensated", response.atmp_compensated);
        gauge!("airgradient_rhum", response.rhum as f64);
        gauge!(
            "airgradient_rhum_compensated",
            response.rhum_compensated as f64
        );
        gauge!(
            "airgradient_pm02_compensated",
            response.pm02_compensated as f64
        );
        gauge!("airgradient_tvoc_index", response.tvoc_index as f64);
        gauge!("airgradient_tvoc_raw", response.tvoc_raw as f64);
        gauge!("airgradient_nox_index", response.nox_index as f64);
        gauge!("airgradient_nox_raw", response.nox_raw as f64);
        gauge!("airgradient_boot", response.boot as f64);
        gauge!("airgradient_boot_count", response.boot_count as f64);

        thread::sleep(Duration::from_secs(60));
    }
}

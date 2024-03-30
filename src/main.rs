#![windows_subsystem = "windows"]

use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf, process, thread::sleep, time::Duration};

const CONFIG_FILE_PATH: &str = "./config.json";
const ERROR_DUMP_FILE_PATH: &str = "./dump.log";

#[derive(Deserialize)]
struct ApiResponse {
    #[serde(rename = "time")]
    _time: String,
    #[serde(rename = "asset_id_base")]
    _asset_id_base: String,
    #[serde(rename = "asset_id_quote")]
    _asset_id_quote: String,
    rate: f64,
}

#[derive(Serialize, Deserialize)]
struct Config {
    api_key: String,
    currency: String,
    fiat_unit: String,
    notify_at: f64,
    scan_delay_in_min: u64,
    notif_dur_in_secs: i32,
}

fn die(s: &str) -> ! {
    if !PathBuf::from(ERROR_DUMP_FILE_PATH).exists() {
        if let Ok(mut file) = fs::File::create(ERROR_DUMP_FILE_PATH) {
            let _ = file.write_all(s.as_bytes());
        }
    }

    process::exit(0);
}

fn get_config() -> Option<Config> {
    if !PathBuf::from(CONFIG_FILE_PATH).exists() {
        if fs::File::create(CONFIG_FILE_PATH).is_err() {
            die("Failed to create config file.");
        }

        return None;
    }

    if let Ok(config) = fs::read_to_string(CONFIG_FILE_PATH) {
        if config.is_empty() {
            die("Config missing.")
        }

        if let Ok(deserialized) = serde_json::from_str::<Config>(&config) {
            Some(deserialized)
        } else {
            die("Unable to deserialize config file.");
        }
    } else {
        die("Failed to read API key.");
    }
}

fn main() -> anyhow::Result<()> {
    let config = match get_config() {
        Some(c) => c,
        None => die("API key missing"),
    };

    Notification::new()
        .summary(&format!(
            "Monmon is scouting for cheap {} rates!",
            config.currency
        ))
        .body(&format!(
            "You will be notified when {} drops below {:.2} {}.",
            config.currency, config.notify_at, config.fiat_unit
        ))
        .sound_name("Mail")
        .timeout(2000)
        .show()?;

    sleep(Duration::from_secs(3));

    let api_endpoint_url = format!(
        "https://rest.coinapi.io/v1/exchangerate/{}/{}",
        config.currency, config.fiat_unit
    );

    loop {
        if let Ok(response) = ureq::get(&api_endpoint_url)
            .query("apikey", &config.api_key)
            .call()
        {
            if let Ok(response) = response.into_json::<ApiResponse>() {
                if response.rate >= config.notify_at {
                    continue;
                }

                Notification::new()
                    .summary(&format!("{} is cheap right now!", config.currency))
                    .body(&format!(
                        "{} just dropped to {:.2} {}!",
                        config.currency, response.rate, config.fiat_unit
                    ))
                    .sound_name("Mail")
                    .timeout(config.notif_dur_in_secs * 1000)
                    .show()?;
            }
        };

        sleep(Duration::from_secs(config.scan_delay_in_min * 60));
    }
}

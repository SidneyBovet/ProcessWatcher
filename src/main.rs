use std::{thread, time, fs};
use std::sync::Arc;
use sysinfo::{ProcessExt, RefreshKind, SystemExt};
use serde::{Deserialize};
use reqwest::Error;

#[derive(Debug, Deserialize)]
struct Process {
    name: String,
    required_arguments: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Remote {
    ip: String,
    route_on: String,
    route_off: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    process: Process,
    remote: Remote,
}

// Reads config from JSON config
fn load_config() -> Config {
    let data = fs::read_to_string("./config.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");

    return serde_json::from_value(json).unwrap();
}

// Main loop that watches for a given process, calling back when either found or not
fn watch_loop<F>(config: &Config, found_callback: Arc<F>) where
    F: Fn(&Config, bool) {
    let mut system = sysinfo::System::new_all();

    loop {
        // update process information
        system.refresh_specifics(RefreshKind::new().with_processes());

        let mut found = false;
        for process in system.get_process_by_name(&config.process.name) {
            // Look for all processes with a given name and arguments
            let mut all_args_satisfy = true;
            for arg in &config.process.required_arguments {
                all_args_satisfy = all_args_satisfy && process.cmd().contains(&arg);
            }
            if all_args_satisfy {
                // The process matches all requirements
                found = true;
                break;
            }
        }

        found_callback(config, found);

        thread::sleep(time::Duration::from_millis(5000));
    }
}

// Sends a HTTP GET to the given address
fn send_get(addr: &str) -> Result<(), Error> {
    println!("Sending [GET {}]", addr);
    let body = reqwest::blocking::get(addr)?.text()?;
    println!("Body:\n{}", body);

    Ok(())
}

// Constructs the address given a remote and whether to turn it on or off
fn get_address(config: &Config, turn_on: bool) -> String {
    let mut address: String = "http://".to_owned();
    address.push_str(&config.remote.ip);
    if turn_on {
        address.push_str(&config.remote.route_on);
    } else {
        address.push_str(&config.remote.route_off);
    }
    return address;
}

fn main() {
    let config = load_config();

    let switch_led = Arc::new(move |config: &Config, on: bool| {
        println!("Switching LED");
        send_get(&get_address(config, on)).expect("Couldn't send HTTP GET");
    });

    watch_loop(&config, switch_led);
}

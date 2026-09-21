use clap::{Parser, Subcommand};
use serde::{Deserialize, Deserializer};
mod comm;
mod tools;
use comm::*;
use std::fmt;
use std::fs;
use tools::*;

use std::process::exit;
use std::{thread::sleep, time::Duration};

#[derive(Debug)]
pub struct HexU8(pub u8);

impl<'de> Deserialize<'de> for HexU8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value =
            u8::from_str_radix(s.trim_start_matches("0x"), 16).map_err(serde::de::Error::custom)?;
        Ok(HexU8(value))
    }
}

impl fmt::Display for HexU8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Config {
    address: u8,
    verbose: bool,
    i2c_speed_hz: u32,
    reset_on_open: bool,
    timeout: u64,
    ont: Pinconf,
}

#[derive(Debug, Deserialize)]
struct Pinconf {
    port: Port,
    pin: Pin,
    direction: Direction,
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    mitac: MitacConf,
}
#[derive(Debug, Deserialize)]
struct MitacConf {
    address: HexU8,
    verbose: bool,
    i2c_speed_hz: u32,
    reset_on_open: bool,
    timeout: u64,
    ignition: Pinconf,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// On
    On,
    /// Off
    Off,
    /// Status
    Status,
}

#[derive(clap::ValueEnum, Copy, Clone, PartialEq, Eq, Debug)]
enum Actions {
    /// On
    On,
    /// Off
    Off,
    ///Status
    Status,
}

fn read_config() -> MitacConf {
    let config_content = fs::read_to_string("config.yaml").unwrap_or_default();

    let parsed: Result<RootConfig, _> = serde_yaml::from_str(&config_content);
    match parsed {
        Ok(cfg) => cfg.mitac,
        Err(_) => MitacConf {
            address: HexU8(0x20),
            verbose: false,
            i2c_speed_hz: 400_000,
            reset_on_open: false,
            timeout: 10,
            ignition: Pinconf {
                port: Port::Portb,
                pin: Pin::Pin1,
                direction: Direction::Output,
            },
        },
    }
}

fn main() {
    let cli = Cli::parse();
    let config = read_config();
    if config.verbose {
        println!("[Mitac] Configuration: \n{:?}", config);
    }
    let mut dev = match start_device(config.i2c_speed_hz, config.reset_on_open, config.timeout) {
        Ok(x) => x,
        Err(err) => {
            println!("Error: {err}, check USB connection");
            exit(1);
        }
    };
    set_pin_dir(
        &mut dev,
        config.address.0,
        config.ignition.port,
        config.ignition.pin,
        config.ignition.direction,
    ); // Ignition

    match &cli.command {
        Some(Commands::On) => {
            write_pin(
                &mut dev,
                config.address.0,
                config.ignition.port,
                config.ignition.pin,
                1,
            );
        }
        Some(Commands::Off) => {
            write_pin(
                &mut dev,
                config.address.0,
                config.ignition.port,
                config.ignition.pin,
                0,
            );
        }
        Some(Commands::Status) => {
            let status = read_pin(
                &mut dev,
                config.address.0,
                config.ignition.port,
                config.ignition.pin,
            );
            match status {
                0 => println!("OFF"),
                _ => println!("ON"),
            }
        }
        None => {}
    }
}

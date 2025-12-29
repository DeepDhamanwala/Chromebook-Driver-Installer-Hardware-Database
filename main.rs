use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DriverDB {
    version: String,
    devices: Vec<Device>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Device {
    model_id: String,
    name: String,
    chipset: String,
    drivers: Vec<String>,
}

// --- CLI Structure ---

#[derive(Parser)]
#[command(name = "CrosDriverInstaller")]
#[command(version = "1.0")]
#[command(about = "Automated Driver Installer for Chromebooks running Linux", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all supported models in the database
    ListModels,
    /// Installs drivers for a specific model ID
    Install {
        #[arg(short, long)]
        model: String,
    },
    /// Updates the local driver database from the remote server
    UpdateDb,
}

// --- Core Logic ---

fn main() {
    let cli = Cli::parse();

    // In a real scenario, this URL would be your raw GitHub content or API endpoint
    // For this demo, we will load from a local file, but the logic simulates a fetch.
    let db_path = "hardware_db.json";

    match &cli.command {
        Commands::ListModels => {
            let db = load_database(db_path);
            println!("Loaded Database Version: {}", db.version);
            println!("{:<20} | {:<30} | {:<15}", "Model ID", "Device Name", "Chipset");
            println!("{:-<20}-|-{:-<30}-|-{:-<15}", "", "", "");
            
            for device in db.devices {
                println!("{:<20} | {:<30} | {:<15}", device.model_id, device.name, device.chipset);
            }
        }
        Commands::UpdateDb => {
            println!("Checking for updates...");
            // Simulate network latency
            thread::sleep(Duration::from_secs(1)); 
            
            // Here you would use reqwest::blocking::get(url)
            println!("Successfully fetched latest driver metadata!");
            println!("Database updated to latest version.");
        }
        Commands::Install { model } => {
            let db = load_database(db_path);
            
            if let Some(device) = db.devices.iter().find(|d| d.model_id == *model) {
                println!("Detected Device: {}", device.name);
                println!("Chipset: {}", device.chipset);
                println!("Preparing to install {} drivers...", device.drivers.len());
                
                for driver in &device.drivers {
                    install_driver(driver);
                }
                
                println!("\nSUCCESS: All drivers for {} installed successfully.", device.name);
                println!("Please reboot your device.");
            } else {
                eprintln!("Error: Model ID '{}' not found in database.", model);
                eprintln!("Run 'list-models' to see available devices.");
            }
        }
    }
}

fn load_database(path: &str) -> DriverDB {
    // Try to read the file
    let data = fs::read_to_string(path).expect("Unable to read hardware_db.json");
    let db: DriverDB = serde_json::from_str(&data).expect("JSON was not well-formatted");
    db
}

fn install_driver(driver_name: &str) {
    print!("Installing {}... ", driver_name);
    // Flush stdout to ensure "Installing..." prints before the pause
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    // Simulate installation work
    thread::sleep(Duration::from_millis(800));
    
    println!("Done [✓]");
}
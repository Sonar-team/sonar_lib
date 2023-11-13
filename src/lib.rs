use std::{error::Error, sync::{atomic::{AtomicBool, Ordering::SeqCst}, Arc}, thread::{self, sleep}, time::Duration};
use capture_packet::{all_interfaces, one_interface};
use colored::Colorize;
use csv::Writer;

pub mod capture_packet;

/// Prints an ASCII art banner for the application.
///
/// # Returns
/// A `String` containing the colored ASCII art banner.
pub fn print_banner() -> String {
    // ASCII art banner
    let banner = r"
    _________                           
   /   _____/ ____   ____ _____ _______ 
   \_____  \ /  _ \ /    \\__  \\_  __ \
   /        (  <_> )   |  \/ __ \|  | \/
  /_______  /\____/|___|  (____  /__|   
          \/            \/     \/          
   ";

    banner.green().to_string()
}


/// Scans the specified interface for a given duration and outputs to a CSV file.
///
/// # Arguments
/// * `output` - The file path to save the scan results as a CSV file.
/// * `interface` - The network interface to scan.
/// * `time` - Duration for which the scan will run, in seconds.
///
/// # Returns
/// A `Result<(), Box<dyn Error>>` indicating success or failure.
pub fn scan_for_time(output: &str, interface: &str, time: u64) -> Result<(), Box<dyn Error>> {
    println!(
        "Scanning {} interface(s) for {} seconds...",
        interface, time
    );
    let interface_clone = interface.to_owned();
    thread::spawn(move || {
        interfaces_handler(&interface_clone);
    });

    compte_a_rebours(time);
    create_csv(output)
}

fn compte_a_rebours(mut time: u64) {
    loop {
        println!(
            "{}",
            format!("Compte à rebours: {} secondes restantes", time).red()
        );
        if time == 0 {
            break;
        }
        time -= 1;
        sleep(Duration::from_secs(1));
    }
    println!("{}", "Compte à rebours: Temps écoulé!".red());
}

/// Creates a CSV file at the specified path.
///
/// # Arguments
/// * `output` - The file path where the CSV file will be created.
///
/// # Returns
/// A `Result<(), Box<dyn Error>>` indicating success or failure.
pub fn create_csv(output: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(output)?;
    writer.flush()?;
    Ok(())
}

/// Continuously scans the specified interface until an interrupt is received.
///
/// # Arguments
/// * `output` - The file path to save the scan results as a CSV file.
/// * `interface` - The network interface to scan.
///
/// # Returns
/// A `Result<(), Box<dyn Error>>` indicating success or failure.
pub fn scan_until_interrupt(interface: &str) {
    interfaces_handler(interface);
    
}

/// Handles interrupt signals and finalizes the CSV output.
///
/// # Arguments
/// * `r` - An `Arc<AtomicBool>` used to monitor interrupt signals.
/// * `output` - The file path to save the final scan results as a CSV file.
///
/// # Returns
/// A `Result<(), Box<dyn std::error::Error>>` indicating success or failure.
pub fn handle_interrupt(
    r: Arc<AtomicBool>,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Ctrl+C pressed. Exiting...");
    r.store(false, SeqCst);
    create_csv(output)
}

fn interfaces_handler(interface: &str) {
    match check_interface(interface) {
        true => all_interfaces(),
        false => one_interface(interface),
    }
}

fn check_interface(interface: &str) -> bool {
    matches!(interface, "all")
}

mod tests_unitaires;

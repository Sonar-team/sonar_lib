use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};

use capture_packet::{all_interfaces, one_interface};

use colored::Colorize;
use csv::Writer;

pub mod capture_packet;

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

pub fn create_csv(output: &str) -> Result<(), Box<dyn Error>> {
    // creat a csv file
    let mut writer = Writer::from_path(output)?;
    // Fermez le fichier CSV (c'est important pour garantir que les données sont écrites)
    writer.flush()?;
    Ok(())
}

pub fn scan_until_interrupt(output: &str, interface: &str) -> Result<(), Box<dyn Error>> {
    interfaces_handler(interface);

    create_csv(output)
}

// This function expects `create_csv` to be defined elsewhere and to return Result<(), io::Error>
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

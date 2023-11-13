use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use std::thread;

use layer_2_infos::PacketInfos;
mod layer_2_infos;

/// Captures packets on all network interfaces.
///
/// This function starts a separate thread for each network interface found on the system.
/// Each thread captures packets independently.
pub fn all_interfaces() {
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }
    // Wait for all threads to complete
    for handle in handles {
        match handle.join() {
            Ok(_) => (), // Thread completed without panicking
            Err(e) => eprintln!("A thread panicked: {:?}", e),
        }
    }
}

/// Captures packets on a specified network interface.
///
/// # Arguments
/// * `interface` - A string slice that holds the name of the interface.
///
/// # Panics
/// Panics if no interface with the given name is found.
pub fn one_interface(interface: &str) {
    println!("L'interface choisie est: {}", interface);
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface;
    let interfaces = datalink::interfaces();
    let captured_interface = match interfaces.into_iter().find(interface_names_match) {
        Some(interface) => interface,
        None => {
            panic!("No such interface '{}'", interface);
        }
    };
    capture_packets(captured_interface);
}

/// Internal function to capture packets on a given network interface.
///
/// # Arguments
/// * `interface` - A `datalink::NetworkInterface` object representing the network interface.
///
/// # Panics
/// Panics if there is an error in creating the datalink channel or in packet capture.
fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    let packet_info = PacketInfos::new(&interface.name, &ethernet_packet);
                    println!("{}", packet_info);
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

/// Retrieves a list of network interface names available on the system.
///
/// This function iterates over all network interfaces, logs their names,
/// and then collects these names into a vector. It's useful for obtaining
/// a quick overview of the network interfaces that can be used for further
/// network operations.
///
/// # Examples
/// ```
/// use sonar_lib::capture_packet::get_interfaces;
/// let interfaces = get_interfaces();
/// for interface in interfaces {
///     println!("Interface: {}", interface);
/// }
/// ```
///
/// # Returns
/// Returns a `Vec<String>` containing the names of all network interfaces
/// found on the system.
///
/// # Panics
/// This function does not explicitly handle any errors related to fetching
/// network interfaces and will panic if `datalink::interfaces()` fails.
pub fn get_interfaces() -> Vec<String> {
    
    let interfaces = datalink::interfaces();
    println!("Fetching network interfaces");

    let names: Vec<String> = interfaces.iter().map(|iface| {
        let name = iface.name.clone();
        println!("Found interface: {}", name);
        name
    }).collect();

    names
}

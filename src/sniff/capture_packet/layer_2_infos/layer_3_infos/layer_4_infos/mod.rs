use pnet::packet::icmp::IcmpPacket;
use pnet::packet::icmpv6::Icmpv6Packet;
//use pnet::packet::dhcp::DhcpPacket;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use serde::Serialize;

use log::info;
//use pnet::packet::Packet;

#[derive(Debug, Default, Serialize, Clone, Eq, Hash, PartialEq)]
pub struct Layer4Infos {
    pub port_source: Option<String>,
    pub port_destination: Option<String>,
}

trait PacketPorts {
    fn ports(&self) -> Layer4Infos;
}

impl PacketPorts for TcpPacket<'_> {
    fn ports(&self) -> Layer4Infos {
        Layer4Infos {
            port_source: Some(self.get_source().to_string()),
            port_destination: Some(self.get_destination().to_string()),
        }
    }
}

impl PacketPorts for UdpPacket<'_> {
    fn ports(&self) -> Layer4Infos {
        Layer4Infos {
            port_source: Some(self.get_source().to_string()),
            port_destination: Some(self.get_destination().to_string()),
        }
    }
}

pub fn get_layer_4_infos(proto: IpNextHeaderProtocol, data: &[u8]) -> Layer4Infos {
    match proto {
        IpNextHeaderProtocols::Tcp => {
            if let Some(tcp_packet) = TcpPacket::new(data) {
                tcp_packet.ports()
            } else {
                Default::default()
            }
        }
        IpNextHeaderProtocols::Udp => {
            if let Some(udp_packet) = UdpPacket::new(data) {
                udp_packet.ports()
            } else {
                Default::default()
            }
        }
        IpNextHeaderProtocols::Icmp => {
            if let Some(_icmp_packet) = IcmpPacket::new(data) {
                Default::default()
            } else {
                Default::default()
            }
        }
        IpNextHeaderProtocols::Icmpv6 => {
            if let Some(_icmpv6_packet) = Icmpv6Packet::new(data) {
                Default::default()
            } else {
                Default::default()
            }
        }
        IpNextHeaderProtocols::Igmp => Default::default(),
        IpNextHeaderProtocols::Ipv6Frag => Default::default(),
        IpNextHeaderProtocols::Hopopt => {
            // Handle HOPOPT protocol, if necessary
            Default::default()
        }
        _ => {
            // General case for all other EtherTypes
            info!("layer 4 - Unknown or unsupported packet type: {}", proto);
            Default::default()
        }
    }
}

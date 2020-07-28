use pnet_macros_support::packet::Packet;
use pnet_macros_support::types::*;
use pnet_packet::ip::IpNextHeaderProtocols::Tcp;
use pnet_packet::ipv4::{checksum, Ipv4Flags, MutableIpv4Packet};
use pnet_packet::tcp::{ipv4_checksum, MutableTcpPacket, TcpFlags, TcpPacket};
use pnet_transport::transport_channel;
use pnet_transport::TransportChannelType::Layer3;
use rand;
use std::io::{Error, ErrorKind};
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    if src.starts_with("0x") {
        u32::from_str_radix(&src[2..], 16)
    } else {
        u32::from_str(src)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sendpkt")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(long)]
    ip_ttl: Option<u8>,

    #[structopt(long)]
    ip_sip: Ipv4Addr,

    #[structopt(long)]
    tcp_sport: u16,

    #[structopt(long)]
    ip_dip: Ipv4Addr,

    #[structopt(long)]
    tcp_dport: u16,

    #[structopt(long)]
    tcp_window: Option<u16>,

    #[structopt(long, parse(try_from_str = parse_hex))]
    tcp_seq: Option<u32>,

    #[structopt(long)]
    tcp_flag_ack: bool,

    #[structopt(long)]
    tcp_flag_cwr: bool,

    #[structopt(long)]
    tcp_flag_ece: bool,

    #[structopt(long)]
    tcp_flag_fin: bool,

    #[structopt(long)]
    tcp_flag_ns: bool,

    #[structopt(long)]
    tcp_flag_psh: bool,

    #[structopt(long)]
    tcp_flag_rst: bool,

    #[structopt(long)]
    tcp_flag_syn: bool,

    #[structopt(long)]
    tcp_flag_urg: bool,
}

impl Opt {
    fn build_tcp_packet(&self) -> Result<TcpPacket, Error> {
        let mut ip_header = match MutableIpv4Packet::owned(vec![0u8; 20]) {
            Some(s) => s,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Can't create `MutableIpv4Packet`.",
                ))
            }
        };

        ip_header.set_version(4);
        ip_header.set_header_length(5);
        // Miss: type of service
        // Miss: total length
        ip_header.set_identification(rand::random::<u16>());
        ip_header.set_flags(Ipv4Flags::DontFragment);
        // Miss: fragment offset
        ip_header.set_ttl(match self.ip_ttl {
            Some(ttl) => ttl,
            None => 128,
        });
        ip_header.set_next_level_protocol(Tcp);
        ip_header.set_source(self.ip_sip);
        ip_header.set_destination(self.ip_dip);

        // construct TCP header
        let mut tcp_header = match MutableTcpPacket::owned(vec![0u8; 20]) {
            Some(s) => s,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Can't create `MutableTcpPacket`.",
                ))
            }
        };

        self.tcp_seq.map_or(rand::random::<u32>(), |seq| {
            tcp_header.set_sequence(seq);
            seq
        });

        tcp_header.set_source(self.tcp_sport);
        tcp_header.set_destination(self.tcp_dport);
        tcp_header.set_data_offset(5);

        self.tcp_window.map_or(29200, |w| {
            tcp_header.set_window(w);
            w
        });

        let mut flags: u9be = 0;

        if self.tcp_flag_ack {
            flags |= TcpFlags::ACK;
        }

        if self.tcp_flag_cwr {
            flags |= TcpFlags::CWR;
        }

        if self.tcp_flag_ece {
            flags |= TcpFlags::ECE;
        }

        if self.tcp_flag_fin {
            flags |= TcpFlags::FIN;
        }

        if self.tcp_flag_ns {
            flags |= TcpFlags::NS;
        }

        if self.tcp_flag_psh {
            flags |= TcpFlags::PSH;
        }

        if self.tcp_flag_rst {
            flags |= TcpFlags::RST;
        }

        if self.tcp_flag_syn {
            flags |= TcpFlags::SYN;
        }

        if self.tcp_flag_urg {
            flags |= TcpFlags::URG;
        }

        tcp_header.set_flags(flags);
        ip_header.set_checksum(checksum(&ip_header.to_immutable()));
        tcp_header.set_checksum(ipv4_checksum(
            &tcp_header.to_immutable(),
            &self.ip_sip,
            &self.ip_dip,
        ));

        if self.verbose {
            println!("Packet option: {:#?}", self);
            println!("IP Header: {:?}", ip_header);
            println!("TCP Header: {:?}", tcp_header);
        }

        let mut v1 = (*ip_header.packet()).to_vec();
        let mut v2 = (*tcp_header.packet()).to_vec();
        v1.append(&mut v2);

        return match TcpPacket::owned(v1) {
            Some(s) => Ok(s),
            None => Err(Error::new(
                ErrorKind::InvalidData,
                "Can't create `TcpPacket`.",
            )),
        };
    }
}

fn main() {
    let opt = Opt::from_args();

    let packet = opt.build_tcp_packet().unwrap();
    let msg = format!(
        "[{}{} -> {}{}] ready send packet {:?}",
        opt.ip_sip, opt.tcp_sport, opt.ip_dip, opt.tcp_dport, &packet
    );
    let (mut tx, _) = transport_channel(100, Layer3(Tcp)).unwrap();
    match tx.send_to(packet, IpAddr::V4(opt.ip_dip)) {
        Ok(_) => {
            println!("{}", msg);
        }
        Err(e) => panic!("failed to send packet: {:?}", e),
    }
}

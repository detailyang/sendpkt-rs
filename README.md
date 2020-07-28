<p align="center">
  <b>
    <span style="font-size:larger;">sendpkt-rs</span>
  </b>
  <br />
   <a href="https://travis-ci.com/detailyang/sendpkt-rs"><img src="https://travis-ci.com/detailyang/sendpkt-rs.svg?token=thDZbmEQtVwYMM6yT8Dv&branch=master"/></a>
   <a href="https://ci.appveyor.com/project/detailyang/sendpkt-rs"><img src="https://ci.appveyor.com/api/projects/status/drc2xk4kcoiydr0x?svg=true" /></a>
   <br />
   <b>sendpkt-rs send the TCP Packet from cli</b>
</p>

# purpose

kill the quiet TCP connection (CLOSE-WAIT status) by sending the RST segment.

# sendpkt-rs

sendpkt-rs is a command line tool to send TCP Packet with specified option as the following:

```bash
USAGE:
    sendpkt [FLAGS] [OPTIONS] --ip-dip <ip-dip> --ip-sip <ip-sip> --tcp-dport <tcp-dport> --tcp-sport <tcp-sport>

FLAGS:
    -h, --help            Prints help information
        --tcp-flag-ack
        --tcp-flag-cwr
        --tcp-flag-ece
        --tcp-flag-fin
        --tcp-flag-ns
        --tcp-flag-psh
        --tcp-flag-rst
        --tcp-flag-syn
        --tcp-flag-urg
    -V, --version         Prints version information
    -v, --verbose         Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
        --ip-dip <ip-dip>
        --ip-sip <ip-sip>
        --ip-ttl <ip-ttl>
        --tcp-dport <tcp-dport>
        --tcp-seq <tcp-seq>
        --tcp-sport <tcp-sport>
        --tcp-window <tcp-window>
```

# requirements
You have must WinPcap or npcap installed which is [libpnet](https://github.com/libpnet/libpnet) required.


### usage

## send rst packet
```bash
sendpkt --ip-dip 127.0.0.1 --ip-sip 127.0.0.1 --tcp-dport 8888 --tcp-sport 1234  --tcp-seq 0x12345 --tcp-flag-rst
```

## send syn packet
```bash
sendpkt --ip-dip 127.0.0.1 --ip-sip 127.0.0.1 --tcp-dport 8888 --tcp-sport 1234  --tcp-seq 0x12345 --tcp-flag-syn
```

# License
sendpkt-rs is under the [MIT license](/LICENSE). See the [LICENSE](/LICENSE) file for details.

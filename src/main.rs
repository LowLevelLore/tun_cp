use std::io::prelude::*;
use std::{io, thread};

// fn main() -> io::Result<()> {
//     println!("Started");
//     let res_nic = Iface::new("tun0", Mode::Tun);
//     match res_nic {
//         Ok(nic) => {
//             let name = nic.name();
//             let mut buffer = vec![0; 150]; // MTU + 4 for the header
//             println!("NIC: {name}");
//             loop {
//                 let nbytes = nic.recv(&mut buffer).unwrap();
//                 let flags = u16::from_be_bytes([buffer[0], buffer[1]]);
//                 let proto = u16::from_be_bytes([buffer[2], buffer[3]]);
//                 if proto != 0x0800 {
//                     // Not an IPv4 Packet
//                     continue;
//                 }
//                 eprintln!("FLAGS: {:X}, PROTO: {:X}", flags, proto);
//                 eprintln!(
//                     "Recieved ({}): {:?}",
//                     nbytes - 4 as usize,
//                     &buffer[4..nbytes]
//                 );
//             }
//         }
//         Err(msg) => {
//             println!("ERROR: {:?}", msg);
//             Ok(())
//         }
//     }
// }

fn main() -> io::Result<()> {
    let mut nic = tun_cp::Interface::new()?;
    let mut listener = nic.bind(8000)?;
    while let Ok(mut stream) = listener.accept() {
        eprintln!("Connection Established !!");
        thread::spawn(move || {
            stream.write(b"Hello from TUN-CP!\n").unwrap();
            stream.shutdown(std::net::Shutdown::Write).unwrap();
            loop {
                let mut buf = [0; 512];
                let n = stream.read(&mut buf[..]).unwrap();
                if n == 0 {
                    break;
                } else {
                    println!("READ: {}", std::str::from_utf8(&buf[..n]).unwrap());
                }
            }
        });
    }
    Ok(())
}

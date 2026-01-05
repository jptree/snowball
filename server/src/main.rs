use std::net::UdpSocket;

use tokio::{io, net::TcpListener, time};


fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080")?;
    let mut buf = [0; 1024];

    let mut last_ping = time::Instant::now();

    loop {
        let (len, addr) = sock.recv_from(&mut buf)?;
        println!("{:?} bytes received from {:?}", len, addr);

        let len = sock.send_to(&buf[..len], addr)?;
        println!("{:?} bytes sent", len);

        let now = time::Instant::now();

        if now - last_ping > time::Duration::from_secs(1) {
            last_ping = now;
            let ping = [4; 10];
            sock.send_to(&ping, addr)?;
        }
    }
}
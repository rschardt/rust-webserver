use std::net::UdpSocket;
use std::io::BufReader;
use std::io::read_to_string;
use std::fs::File;

fn main() -> std::io::Result<()> {
{
        // TODO irgendwie wird der string nicht gelesen
        let testHeader = File::open("src/http-headers/test-header.txt")?;
        let mut sendBuf = BufReader::new(testHeader);
        let stringToSend = read_to_string(sendBuf)?;
        //println!("String content: {stringToSend:?}");

        let socket = UdpSocket::bind("::1:8080").expect("couldn't bind to address");
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut recvBuf = [0; 10];

        let (amt, sockConn) = socket.recv_from(&mut recvBuf).expect("Didn't receive data");

        // TODO Find out why the buffer isn't send correctly back
        socket.send_to(stringToSend.as_bytes(), &sockConn)?;
    } // the socket is closed here
    Ok(())
}

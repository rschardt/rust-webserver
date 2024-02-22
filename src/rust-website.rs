use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{BufReader, Read, Write};
use std::fs::File;
use std::time::Instant;

const RECV_BYTES_LENGTH: usize = 1500;
const SEND_BYTES_LENGTH: usize = 3000;

fn main() -> std::io::Result<()>
{
    create_webserver()
}

fn create_webserver() -> std::io::Result<()> {
    let tcp_socket = TcpListener::bind("::1:8080").expect("couldn't bind to address");

    loop {
	handle_client(&tcp_socket);
    }
}

fn handle_client(tcp_socket: &TcpListener) {

    match tcp_socket.accept() {
	Ok((tcp_stream, socket_addr)) =>
	{
	    println!("new client: {socket_addr:?}");

	    // Data Transfer
	    let get_header = handle_request(&tcp_stream);
	    send_response(&tcp_stream, &get_header);

	    tcp_stream.shutdown(Shutdown::Both).expect("something went wrong during shutdown");
	    println!("client connection: {socket_addr:?} shutdown");
	},
	Err(e) => println!("couldn't get client: {e:?}"),
    };
}

fn handle_request(mut tcp_stream: &TcpStream) -> String {

    let mut full_request: Vec<u8> = Vec::new();
    let connection_timer = Instant::now();

    // Request reading
    while connection_timer.elapsed().as_secs() < 30 {

	let mut read_buffer = [0; RECV_BYTES_LENGTH];
	let read_bytes = tcp_stream.read(&mut read_buffer).unwrap();
	let buf_to_read = &read_buffer[0..read_bytes];
	println!("bytes read: {read_bytes:?}");
	println!("client message: {read_buffer:?}");

	full_request.extend_from_slice(buf_to_read);
	if read_bytes == 0 || read_bytes < RECV_BYTES_LENGTH { break; }
    }

    // Header parsing
    let request_string = String::from_utf8(full_request).unwrap();
    let splitted_headers: Vec<&str> = request_string.split("\r\n").collect();

    //println!("This is the request: {request_string:?}");
    //println!("Those are the splitted headers: {splitted_headers:?}");

    let get_header = match splitted_headers.iter().find(|x| x.starts_with("GET")) {
	None => "",
	Some(v) => v,
    };
    get_header.to_string()
}

fn send_response(mut tcp_stream: &TcpStream, get_header: &String) {

    // TODO return appropriate header
    //if !get_header.is_empty() -> send method not supported etc.

    // Method 1 from File
    let test_header = File::open("src/http-headers/test-header.txt").expect("couldn't open file");
    let mut buf_reader = BufReader::new(test_header);
    let mut read_buffer = [0; SEND_BYTES_LENGTH];

    let read_bytes = buf_reader.read(&mut read_buffer).expect("couldn't read file buffer");
    let buf_to_send = &read_buffer[0..read_bytes];

    let string_content = String::from_utf8_lossy(buf_to_send);
    println!("buffer created, ready to send: {string_content:?}");

    tcp_stream.write(&buf_to_send).expect("unable to write to tcp_stream");

    // Method 2 directly from memory
    // let written_Bytes = tcp_stream.write_all(b"HTTP/1.1 200 OK\nDate: Sat, 09 Oct 2010 14:28:02 GMT\nServer: Apache\nLast-Modified: Tue, 01 Dec 2009 20:18:22 GMT\nETag: \"51142bc1-7449-479b075b2891b\"\nAccept-Ranges: bytes\nContent-Type: text/html\n\n<!DOCTYPE html>\n<html>\n  <head>\n    <title>\n      Hello World\n    </title>\n  </head>\n  <body>\n    <pre>Hello World\n    </pre>\n  </body>\n</html>\n");
    // println!("bytes written: {written_Bytes:?}");

    tcp_stream.flush().expect("unable to flush tcp_stream");
}

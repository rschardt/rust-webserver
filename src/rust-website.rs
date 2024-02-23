use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

const RECV_BYTES_LENGTH: usize = 3000;

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

    // Request reading
    let mut read_buffer = [0; RECV_BYTES_LENGTH];
    let read_bytes = tcp_stream.read(&mut read_buffer).unwrap();
    let buf_to_read = &read_buffer[0..read_bytes];
    //println!("bytes read: {read_bytes:?}");
    //println!("client message: {read_buffer:?}");

    // Header parsing
    let request_string = String::from_utf8_lossy(buf_to_read);
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

    // TODO return appropriate headers
    //if !get_header.is_empty() -> send method not supported etc.
    // 405 Method Not Allowed
    let http_header = "HTTP/1.1 200 OK";
    let server_header = "Server: rust-webserver";
    let accept_ranges_header = "Accept-Ranges: bytes";
    let content_type_header = "Content-Type: text/html";
    let allow_header = "Allow: GET";
    // TODO this will require chrono crate, to handle DateTimes
    //let date_header = format!("Date: {now:?}");
    //let last_modified_header = "Last-Modified: Tue, 01 Dec 2009 20:18:22 GMT";
    let body = "<!DOCTYPE html>\n<html>\n  <head>\n    <title>\n      Hello World\n    </title>\n  </head>\n  <body>\n    <pre>Hello World\n    </pre>\n  </body>\n</html>\n";

    let server_response = format!("{http_header}\n{server_header}\n{accept_ranges_header}\n{content_type_header}\n{allow_header}\n\n{body}");
    //let server_response = format!("{http_header}\n{date_header}\n{server_header}\n{last_modified_header}\n{accept_ranges_header}\n{content_type_header}\n{allow_header}\n\n{body}");
    let written_bytes = tcp_stream.write_all(server_response.as_bytes());
    //println!("bytes written: {written_bytes:?}");

    tcp_stream.flush().expect("unable to flush tcp_stream");
}

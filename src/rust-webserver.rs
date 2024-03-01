use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::env;

const RECV_BYTES_LENGTH: usize = 3000;

fn main() -> std::io::Result<()>
{
    let web_root_str = handle_args();
    let mut web_root_path = Path::new(&web_root_str);
    create_webserver(&mut web_root_path)
}

fn handle_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && &args[1] == "--web-dir" {

	let new_path = Path::new(&args[2]);
	if match new_path.try_exists() {
	    Ok(true) => true,
	    Ok(false) => { println!("an incorrect argument was passed to --web-dir"); false },
	    Err(_) => { println!("the argument to --web-dir is not a valid directory"); false },
	} {
	    return String::from(&args[2]);
	}
    }
    panic!("the --web-dir argument was not passed correctly");
}

fn create_webserver(web_root: &Path) -> std::io::Result<()> {
    let tcp_socket = TcpListener::bind("::1:8080").expect("couldn't bind to address");

    loop {
	handle_client(&tcp_socket, web_root);
    }
}

fn handle_client(tcp_socket: &TcpListener, web_root: &Path) {

    match tcp_socket.accept() {
	Ok((tcp_stream, socket_addr)) =>
	{
	    println!("new client: {socket_addr:?}");

	    // Data Transfer
	    let get_header = handle_request(&tcp_stream);
	    send_response(&tcp_stream, &get_header, web_root);

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

    let get_header = match splitted_headers.iter().find(|header| header.starts_with("GET")) {
	None => "",
	Some(get_header) => get_header,
    };
    get_header.to_string()
}

fn send_response(mut tcp_stream: &TcpStream, get_header: &str, web_root: &Path) {

    // Status code and body determination
    let mut status_code = "200 OK";
    let mut file_extension = String::from("");
    let mut body = String::from("");

    if get_header.is_empty() {
	status_code = "405 Method Not Allowed";

    } else {

	let parts: Vec<&str> = get_header.split(' ').collect();
	let mut file_path = match parts.iter().find(|part| part.starts_with('/')) {
	    None => "",
	    Some(file_path) => file_path,
	};

	if !file_path.is_empty() {
	    if file_path == "/" { file_path = "/index.html" }

	    let lead_slash_trimmed = &file_path[1..file_path.len()];

	    let mut joined_path = PathBuf::new();
	    joined_path.push(web_root);
	    joined_path.push(lead_slash_trimmed);
	    //println!("Path: {joined_path:#?}");

	    match std::fs::canonicalize(joined_path) {
		Ok(absolute_path) => {
		    file_extension = determine_file_extension(&absolute_path);
		    let file_handle = File::open(absolute_path).unwrap();
		    let mut buf_reader = BufReader::new(file_handle);
		    buf_reader.read_to_string(&mut body).expect("couldn't read file buffer");
		    //println!("buffer created, ready to send: {body:?}");
		}
		Err(e) => {
		    status_code = "404 Not Found";
		    println!("File error: {}", e);
		},
	    }
	} else {
	    status_code = "400 Bad Request";
	}
    }

    let http_header = format!("HTTP/1.1 {status_code}");
    let server_header = "Server: rust-webserver";
    let accept_ranges_header = "Accept-Ranges: bytes";
    let content_type_header = format!("Content-Type: {file_extension}");
    let allow_header = "Allow: GET";

    let server_response = format!("{http_header}\n{server_header}\n{accept_ranges_header}\n{content_type_header}\n{allow_header}\n\n{body}");
    let _written_bytes = tcp_stream.write_all(server_response.as_bytes());
    //println!("bytes written: {written_bytes:?}");

    tcp_stream.flush().expect("unable to flush tcp_stream");
}

fn determine_file_extension(absolute_path: &PathBuf) -> String {
    match absolute_path.extension() {
	None => String::from("text/html"),
	Some(extension) => {
	    match extension.to_str().unwrap_or("") {
		"css" => String::from("text/css"),
		"html" => String::from("text/html"),
		"png" => String::from("image/png"),
		"jpg" | "jpeg" => String::from("image/jpeg"),
		"gif" => String::from("image/gif"),
		"pdf" => String::from("application/pdf"),
		_ => String::from("text/html"),
	    }
	},
    }
}

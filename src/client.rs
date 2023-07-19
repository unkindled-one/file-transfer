use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

pub fn download_file(download_folder: &PathBuf, port: u16) -> Result<(), Box<dyn Error>> {
    let mut stream = get_stream(port)?;
    let file_name = get_file_name(&stream)?;
    let mut final_path = PathBuf::new();
    final_path.push(download_folder);
    final_path.push(file_name);
    read_from_stream(&final_path, &mut stream)?;
    println!("Successfully downloaded file");
    Ok(())
}

/// Listens on port until there is a stream
fn get_stream(port: u16) -> Result<TcpStream, Box<dyn Error>> {
    println!("Waiting for computer to receive file from");
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .expect("Unable to connect");
    for stream in listener.incoming() {
        return match stream {
            Ok(s) => Ok(s),
            Err(_) => Err(Box::try_from("Unable to connect").unwrap()),
        }
    }
    Err(Box::try_from("Unable to connect").unwrap())
}

/// Gets the name of the file from the sender
fn get_file_name(mut stream: &TcpStream) -> Result<String, Box<dyn Error>> {
    // First byte length of name
    let buffer: &mut [u8] = &mut [0; 256];
    let bytes_read = stream.read(buffer)?;
    Ok(String::from_utf8(Vec::from(Vec::from(&buffer[..bytes_read])))?)
}

fn read_from_stream(final_path: &PathBuf, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Receiving data");
    let mut file = File::create(final_path)?;
    loop {
        let buffer: &mut [u8] = &mut [0; 256];
        let bytes_read = stream.read(buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
    }
    Ok(())
}
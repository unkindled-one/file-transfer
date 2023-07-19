use std::error::Error;
use std::fs::File;
use std::net::TcpStream;
use std::io::prelude::*;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub fn send_file(file: &PathBuf, port: u16) -> Result<(), Box<dyn Error>> {
    let mut stream = create_stream(port, 30)?;
    let file_name = file.file_name().ok_or("Error encoding")?.to_str().ok_or("Error encoding")?;
    stream.write(file_name.as_bytes()).expect("Error writing bytes");
    write_file_to_stream(file, stream)?;
    println!("Successfully sent file");
    Ok(())
}

/// Will try to connect to the listener for specific number of seconds, if the field is left empty
fn create_stream(port: u16, max_time_in_secs: u64) -> Result<TcpStream, Box<dyn Error>> {
    println!("Waiting for computer to send file to");
    let now = SystemTime::now();
    loop {
        let temp = TcpStream::connect(format!("127.0.0.1:{port}"));
        match temp {
            Ok(s) => {
                return Ok(s);
            },
            _ => {
                if now.elapsed().expect("System Clock Error").as_secs() >= max_time_in_secs {
                    break;
                }
                sleep(Duration::from_secs(1));
            }
        }
    }
    Err(Box::try_from("Maximum time elapsed, unable to connect").unwrap())
}

fn write_file_to_stream(file_path: &PathBuf, mut stream: TcpStream) -> Result<(), Box<dyn Error>>{
    println!("Sending data");
    let mut f = File::open(file_path)?;
    loop {
        let buffer: &mut [u8] = &mut [0; 256];
        let bytes_read = f.read(buffer)?;
        stream.write(&buffer[..bytes_read])?;
        if bytes_read == 0 {
            break;
        }
    }
    Ok(())
}
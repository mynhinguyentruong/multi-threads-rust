
// mod canada;
mod lib;

use::std::thread;
// use::std::time::Duration;

// use std::alloc::System;
// use std::time::SystemTime;

// use std::sync::{Arc, mpsc, Mutex};
// use std::rc::Rc;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use std::time::Duration;

// use chrono::Utc;
// use chrono::TimeZone;
// use chrono::Date;
// use chrono::{Duration as ChronoDuration, NaiveDate};
use lib::{ThreadPool};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(5);


    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // handle_connection(stream);

        // wants one of the idle thread to handle each incoming stream
        pool.execute(|| {
            handle_connection(stream)
        });
    }

}

fn handle_connection(mut stream: TcpStream) {

    println!("handle_connection ran");

    let mut buffer = [0; 1024];
    println!("After buffer ...");
    stream.read(&mut buffer).unwrap();

    println!("After stream.read...");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();

    println!("After content...");

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, content.len(), content
    );
    println!("After response...");


    stream.write_all(response.as_bytes()).unwrap();

    println!("After stream.write all");
    stream.flush().unwrap()
}







// fn get_date(y: i32, m: u32,d: u32) -> Date<Utc> {
//     let dt = Utc.ymd(y,m,d);
//     // println!("The date you enter: {:?}", dt.format("%Y-%M-%D").to_string());
//     return dt
// }



// fn get_days_left(x: Date)

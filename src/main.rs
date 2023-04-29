
mod canada;
mod lib;

use::std::thread;
use::std::time::Duration;

use std::alloc::System;
use std::time::SystemTime;

use std::sync::{Arc, mpsc, Mutex};
use std::rc::Rc;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpStream, TcpListener}};

use chrono::Utc;
use chrono::TimeZone;
use chrono::Date;
use chrono::{Duration as ChronoDuration, NaiveDate};
use crate::lib::{ThreadPool, Worker};

fn main() {

    let pool = ThreadPool::new(3);

    let worker = Worker::new(1);

    worker.thread.join().unwrap();

}







// fn get_date(y: i32, m: u32,d: u32) -> Date<Utc> {
//     let dt = Utc.ymd(y,m,d);
//     // println!("The date you enter: {:?}", dt.format("%Y-%M-%D").to_string());
//     return dt
// }



// fn get_days_left(x: Date)

#![allow(unused)]
use serde::{Serialize, Deserialize};
use serde_json;
use core::{error, num, time};
use std::{alloc::System, time::{Instant,SystemTime, UNIX_EPOCH}};
use chrono::{naive::{NaiveDate, NaiveTime}, TimeZone};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use rayon::prelude::*;
use std::thread::available_parallelism;
use anyhow::{anyhow, Error, Context};
use std::error::Error as StdError;
use thiserror::Error as ThisError;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    id: u32,
    is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewUserRequest {
    name: String,
    id: u32,
}

impl From<NewUserRequest> for User {
    fn from(request: NewUserRequest) -> Self {
        Self { 
            name: request.name,
            id: request.id,
            is_deleted: false
        }
    }
}

fn handle_request(json_request: &str) {
    match serde_json::from_str::<NewUserRequest>(json_request) {
        Ok(good_request) => {
            let new_user = User::from(good_request);
            println!("Made a new user! {new_user:#?}");
            println!(
                "Serialized back into JSON: {:#?}",
                serde_json::to_string(&new_user)
            );
        }
        Err(e) => {
            println!("Got an error from {json_request}: {e}");
        }
    }
}

fn parse_then_send(input: &[u8]) -> Result<(), Error> {
    let some_str = std::str::from_utf8(input)
        .with_context(|| "Couldn't parse into a str")?;
    let number = some_str.parse::<i32>()
        .with_context(|| format!("Got a weird str to parse: {some_str}"))?;
    send_number(number)?;
    anyhow::Ok(())
}

fn send_number(number: i32) -> Result<(), Error> {
    if number < 1_000_000 {
        println!("Number sent!");
        anyhow::Ok(())
    }
    else {
        println!("Too large!");
        Err(anyhow!("Number is too large"))
    }
}

trait SaysHello {
    fn hello(&self) {
        println!("Hello");
    }
}

impl<T> SaysHello for T {}

struct Nothing;

#[derive(ThisError, Debug)]
enum SystemError {
    #[error("Couldn't send: {0}")]
    SendError(i32),
    #[error("External crate error: {0}")]
    ExternalCrateError(String),
}

trait ToSystemError<T> {
    fn to_system_error(self) -> Result<T, SystemError>;
}

impl<T, E: StdError> ToSystemError<T> for Result<T, E> {
    fn to_system_error(self) -> Result<T, SystemError> {
        self.map_err(|e| SystemError::ExternalCrateError(e.to_string()))
    }
}

fn parse_then_send_2(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input).to_system_error()?;
    let number = some_str.parse::<i32>().to_system_error()?;
    send_number_2(number).to_system_error()?;
    Ok(())
}

fn send_number_2(number: i32) -> Result<(), SystemError> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    }
    else {
        println!("Too large!");
        Err(SystemError::SendError(number))
    }
}

fn main() {
    let good_json_request = r#"
    {
        "name": "BillyTheUser",
        "id": 6876
    }"#;

    let bad_json_request = r#"
    {
        "name": "BillyTheUser",
        "id": "6876"
    }"#;

    handle_request(good_json_request);
    handle_request(bad_json_request);

    let time = Instant::now();
    println!("{:?}", time);
    println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());

    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25));
    println!("{:?}", NaiveTime::from_hms_opt(12, 5, 30));
    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25).unwrap().and_hms_opt(12, 5, 30));

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    println!("Seconds from 1970 to today: {seconds}");

    let naive_dt = DateTime::from_timestamp(seconds as i64, 0).unwrap();
    println!("As DateTime: {naive_dt}");

    let mut my_vec = vec![0; 2_000_000];
    my_vec
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);

    println!("{:?}", &my_vec[5000..5005]);

    println!("Estimated parallelism on this computer: {:?}", available_parallelism());

    let mut without_rayon = vec![];
    let mut with_rayon = vec![];

    for _ in 0..10 {
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec
            .iter_mut().enumerate().for_each(|(index, number)| {
                *number += index + 1;
                *number -= index + 1;
            });
        let elapsed = now.elapsed();
        without_rayon.push(elapsed.as_micros());

        let mut my_vec = vec![0, 2_000_000];
        let now = std::time::Instant::now();
        my_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, number)| {
                *number += index + 1;
                *number -= index + 1;
            });
            let elapsed = now.elapsed();
            with_rayon.push(elapsed.as_micros());
    }

    println!(
        "Average time without rayon: {} microseconds",
        without_rayon.into_iter().sum::<u128>() / 10
    );

    println!(
        "Average time with rayon: {} microseconds",
        with_rayon.into_iter().sum::<u128>() / 10
    );

    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));

    8.hello();
    &'c'.hello();
    &mut String::from("Hello there").hello();
    8.7.hello();
    Nothing.hello();
    std::collections::HashMap::<i32, i32>::new().hello();

    println!("{}", parse_then_send_2(b"nine").unwrap_err());
    println!("{:?}", parse_then_send_2(b"nine"));
    println!("{:?}", parse_then_send_2(b"10"));
}

use dotenv::dotenv;
use error_chain::error_chain;
use reqwest::header::COOKIE;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

const FOLDER: &str = "inputs/";

fn read_input_file(day: usize) -> Result<String> {
    let data_result = File::open(format!("{}day{}.txt", FOLDER, day));

    let mut content = String::new();

    let mut data_file = match data_result {
        Ok(file) => file,
        Err(error) => return Err(error.into()),
    };

    data_file.read_to_string(&mut content)?;

    Ok(content)
}

fn fetch_data(day: usize) -> Result<String> {
    let session = match env::vars().find(|x| x.0 == "AOC_SESSION_COOKIE") {
        Some(var) => var.1,
        None => panic!("No session available"),
    };

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://adventofcode.com/2023/day/{}/input", day))
        .header(COOKIE, format!("session={}", session))
        .send()?;

    let contents = res.text()?;

    let mut data_file = File::create(format!("{}day{}.txt", FOLDER, day))?;

    data_file.write(contents.as_bytes())?;

    Ok(contents)
}

pub fn get_input(day: usize) -> Result<String> {
    dotenv().ok();

    let contents = match read_input_file(day) {
        Ok(content) => content,
        Err(_) => fetch_data(day)?,
    };

    Ok(contents)
}

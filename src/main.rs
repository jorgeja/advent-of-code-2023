#![allow(unused_imports)]
#![allow(dead_code)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use reqwest::{cookie::Jar, Url};
use std::{
    env,
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

pub fn get_input(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let directory = env::var_os("CARGO_MANIFEST_DIR").ok_or("CARGO_MANIFEST_DIR".to_string())?;
    let directory = directory
        .into_string()
        .map_err(|oss| -> String { format!("{:?}", oss).into() })?;
    let day_file = format!("{}/inputs/day_{day}_input.txt", directory);
    let p = Path::new(&day_file);
    if p.exists() {
        Ok(read_to_string(p)?)
    } else {
        let s = fetch_todays_input(year, day)?;
        let mut buffer = File::create(day_file)?;
        buffer.write_all(s.as_bytes())?;
        Ok(s)
    }
}

fn fetch_todays_input(year: u32, day: u32) -> Result<String, Box<dyn Error>> {
    let aoc_session = env::var_os("AOC_SESSION").ok_or("Missing AOC Session token".to_string())?;
    let cookie = format!("session={:?}; Domain=.adventofcode.com", aoc_session);
    let cookie_url = "https://adventofcode.com".parse::<Url>()?;
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &cookie_url);

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()?;

    let res = client.get(url).send()?.bytes()?;
    Ok(String::from_utf8_lossy(&res).to_string())
}

fn main() {}

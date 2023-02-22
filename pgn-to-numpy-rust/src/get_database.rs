use std::io::Write;
use std::io::Read;
use std::io::Seek;
use std::{error::Error, str::FromStr, io::SeekFrom};
use fs_err::File;

use clap::ArgMatches;
use inquire::Confirm;
use reqwest::{
    header::{HeaderValue, CONTENT_LENGTH, RANGE},
    StatusCode,
};
use zstd::Decoder;
use progress_bar::*;

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self, Box<dyn Error>> {
        if buffer_size == 0 {
            Err("invalid buffer_size, give a value greater than zero.")?;
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format!"),
            )
        }
    }
}
pub fn main(options: &ArgMatches) -> Result<(), Box<dyn Error>> {
    const CHUNK_SIZE: u32 = 1024 * 1024 * 100; // 100 MB
    let date = options.get_one::<String>("date").expect("no date");

    let url =
        format!("https://database.lichess.org/standard/lichess_db_standard_rated_{date}.pgn.zst");

    let client = reqwest::blocking::Client::new();
    let response = client.head(&url).send()?;
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")?;
    let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;

    let prompt = format!("The resulting file will be about {:.2} GB. Continue?", length as f64 / 1e9 * 7.1);
    let should_continue = Confirm::new(&prompt)
        .prompt()?;

    if !should_continue {
        println!("Aborting...");
        return Ok(());
    }

    let mut compressed = File::create(format!("database-{date}.pgn.zst"))?;

    init_progress_bar((length / CHUNK_SIZE as u64 + 1) as usize);
    set_progress_bar_action("Downloading", Color::Blue, Style::Bold);

    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
        let mut response = client.get(&url).header(RANGE, range.clone()).send()?;

        let status = response.status();
        if status == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .ok_or("response doesn't include the Retry-After header")?;
            let retry_after = u64::from_str(retry_after.to_str()?).map_err(|_| "invalid Retry-After header")?;
            println!("Too many requests, retrying in {} seconds...", retry_after);
            std::thread::sleep(std::time::Duration::from_secs(retry_after));
            response = client.get(&url).header(RANGE, range).send()?;
        }
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            panic!("Unexpected server response: {}", status)
        }
        std::io::copy(&mut response, &mut compressed)?;

        inc_progress_bar();
    }

    let content = response.text()?;
    std::io::copy(&mut content.as_bytes(), &mut compressed)?;

    finalize_progress_bar();

    drop(compressed);

    let mut uncompressed = File::create(format!("database-{date}.pgn"))?;
    let compressed = File::open(format!("database-{date}.pgn.zst"))?;

    let approx_length = length as f64 / (CHUNK_SIZE as f64 / 16.0 / 7.1) + 1.0;
    init_progress_bar(approx_length as usize);
    set_progress_bar_action("Decoding", Color::LightBlue, Style::Bold);

    let mut decoder = Decoder::new(compressed)?;
    let mut buffer = vec![0u8; (CHUNK_SIZE / 16) as usize];
    while decoder.read(&mut buffer)? > 0 {
        uncompressed.write_all(&buffer)?;
        inc_progress_bar();
    }

    finalize_progress_bar();

    drop(decoder);
    fs_err::remove_file(format!("database-{date}.pgn.zst"))?;

    eprintln!("\x1b[1;32mFile saved under: database-{date}.pgn!\x1b[0m");

    Ok(())
}

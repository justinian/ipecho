extern crate hyper;
extern crate chrono;

use chrono::Local;
use chrono::Duration;
use hyper::client::Client;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::unix::fs::MetadataExt;

static CACHE_FILE: &'static str = "/tmp/extip.cache";
static IPECHO_URL: &'static str = "http://ipecho.net/plain";

fn get_cached_ip() -> Option<String> {
	let max_cache = Duration::minutes(15);

	match File::open(CACHE_FILE) {
		Ok(mut f) => {
			match f.metadata() {
				Ok(m) => {
					let cutoff = (Local::now() - max_cache).timestamp();
					if m.ctime() < cutoff {
						return None;
					}
				},

				Err(_) => return None,
			}

			let mut ip = String::new();
			match f.read_to_string(&mut ip) {
				Ok(_) => Some(ip),
				Err(e) => {
					println!("Error reading from cache file: {}", e);
					None
				},
			}
		},
		Err(_) => None,
	}
}

fn write_file(ip: &String) {
	match File::create(CACHE_FILE) {
		Ok(mut f) => {
			match f.write_all(ip.as_bytes()) {
				Ok(_) => {},
				Err(e) => println!("Error writing to cache file: {}", e),
			}
		}
		Err(e) => println!("Error opening cache file: {}", e),
	}
}

fn get_ip_external() -> Result<String, String> {
	let client = Client::new();
	let mut res = match client.get(IPECHO_URL).send() {
		Ok(res) => res,
		Err(e) => return Err(e.to_string()),
	};

	assert_eq!(res.status, hyper::Ok);

	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();

	write_file(&body);
	return Ok(body);
}

fn main() {
	let ip = match get_cached_ip() {
		Some(ip) => ip,
		None => match get_ip_external() {
			Ok(ip) => ip,
			Err(e) => {
				println!("Error: {}", e);
				return;
			},
		},
	};

	println!("{}", ip);
}

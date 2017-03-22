extern crate tiny_http;

use std::sync::Arc;
use std::thread;

fn main() {
	let server = Arc::new(tiny_http::Server::http("0.0.0.0:8000").unwrap());
	println!("listening on port 8000");

	loop {
		let req = match server.recv() {
			Ok(req) => req,
			Err(_) => break
		};

		println!("{:?}", req);
		println!("headers: {:?}", req.headers());
	}

}

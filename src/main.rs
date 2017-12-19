extern crate clap;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use clap::{Arg, App};
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::ContentLength;
use std::io::{self, Write};
use tokio_core::reactor::Core;

fn main() {
    let matches = App::new("httpbin test tool")
                    .version("0.1.0")
                    .author("Florian Keller <github@floriankeller.de>")
                    .about("test httpbin endpoints")
                    .arg(Arg::with_name("path")
                        .default_value("/ip")
                        .help("Sets a custom path")
                        .long("path")
                        .short("p")
                        .takes_value(true)
                    )
                    .arg(Arg::with_name("method")
                        .default_value("GET")
                        .help("Sets the method to use")
                        .long("method")
                        .short("m")
                        .takes_value(true)
                    )
                    .arg(Arg::with_name("data")
                        .default_value("")
                        .help("Sets the data to transfer")
                        .long("data")
                        .short("d")
                        .takes_value(true)
                    )
                    .get_matches();

    let path = matches
                    .value_of("path")
                    .unwrap()
                    .trim_left_matches('/');
    println!("Using path: \"/{}\"", path);

    let method = matches
                    .value_of("method")
                    .unwrap();
    println!("Using method: \"{}\"", method.to_uppercase());

    let data = matches
                .value_of("data")
                .unwrap()
                .to_owned();
    println!("Using data: \"{}\"", data);

    let uri = format!("http://eu.httpbin.org/{}", path)
                .parse()
                .expect("Parse failed");
    println!("Using URI: \"{}\"", uri);

    connect(uri, method, &data);
}

fn connect(uri: hyper::Uri, method: &str, data: &str) {
    let mut core = Core::new().expect("Core initialization failed");
    let client = Client::new(&core.handle());

    let work = match method.to_uppercase().as_str() {
        "DELETE" => client.request(Request::new(Method::Delete, uri)),
        "GET" => client.request(Request::new(Method::Get, uri)),
        "PATCH" => {
            let mut rq = Request::new(Method::Patch, uri);
            rq.set_body(data.to_owned());
            rq.headers_mut().set(ContentLength(data.len() as u64));
            client.request(rq)
        },
        "POST" => {
            let mut rq = Request::new(Method::Post, uri);
            rq.set_body(data.to_owned());
            rq.headers_mut().set(ContentLength(data.len() as u64));
            client.request(rq)
        },
        "PUT" => {
            let mut rq = Request::new(Method::Put, uri);
            rq.set_body(data.to_owned());
            rq.headers_mut().set(ContentLength(data.len() as u64));
            client.request(rq)
        },
        _ => panic!("Invalid method specified."),
    }
    .and_then(|res| {  // If the future was successful, returns second future
        println!("Response status: {}", res.status());
        println!("Response:");

        res
            .body() // Returns a stream of chunks (byte values)
            .for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(From::from)
            })
    });

    core
        .run(work)
        .expect("Core start failed");
}

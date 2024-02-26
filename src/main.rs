mod output;

use std::env;
use reqwest::blocking;
use crate::output::{output_to_dest, OutputDest};

enum RequestMethods {
    Get,
    Post,
    Delete,
    Head,
}

fn get_request_method_name(from: &RequestMethods) -> String {
    match from {
        RequestMethods::Get => "GET",
        RequestMethods::Delete => "DELETE",
        RequestMethods::Head => "HEAD",
        RequestMethods::Post => "POST"
    }.to_string()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut output_dest = OutputDest::Stdout;
    let mut output_file: Option<String> = None;
    let mut method = RequestMethods::Get;

    if args.contains(&"-P".to_string()) {
        method = RequestMethods::Post;
    }
    else if args.contains(&"-H".to_string()) {
        method = RequestMethods::Head;
    }
    else if args.contains(&"-D".to_string()) {
        method = RequestMethods::Delete;
    }

    println!("[{}] {}", get_request_method_name(&method), args[1].to_string());

    if let Some(f_flag_pos) = args.iter().position(|r| r == "-f") {
        output_dest = OutputDest::File;
        output_file = Some(args[f_flag_pos + 1].to_string());
        println!("Query content will be saved to {}.", output_file.clone().unwrap());
    }

    let client = blocking::Client::new();
    let res: blocking::Response = match method {
        RequestMethods::Get => client.get(args[1].clone()),
        RequestMethods::Post => client.post(args[1].clone()),
        RequestMethods::Head => client.head(args[1].clone()),
        RequestMethods::Delete => client.delete(args[1].clone())
    }.send().expect("Failed to retrieve data from the provided URL.");
    if res.status().is_success() {
        println!("Reading data received from the source.");
        for (header_name, header_value) in res.headers() {
            output_to_dest(
                output_dest,
                format!("{}: {:?}\n", header_name.to_string(), header_value),
                output_file.clone()
            );
        }
        if let Ok(content) = res.text() {
            output_to_dest(output_dest, format!("\n{}", content), output_file.clone());
        }
    } else {
        println!("Failed to retrieve data from the provided URL: {}.", res.status());
    }
}

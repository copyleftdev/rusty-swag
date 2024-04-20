use clap::{App, Arg};
use colored::*;
use reqwest::{self, Client};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use log::{info, error};
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::OpenOptions;

async fn check_host_for_swagger_doc_path(target: String, semaphore: Arc<Semaphore>, file_path: String) {
    let permit = semaphore.acquire().await.expect("Failed to acquire semaphore");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))  // Setting a timeout of 10 seconds
        .build()
        .expect("Failed to create HTTP client");

    let response = client.get(&target).send().await;
    drop(permit); // Explicitly drop the permit
    match response {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) if text.contains("Swagger UI") => {
                    info!("{}", format!("Document Found at: {}", target).green());
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&file_path)
                        .await
                        .expect("Failed to open file");
                    tokio::io::AsyncWriteExt::write_all(&mut file, format!("{}\n", target).as_bytes()).await.expect("Failed to write to file");
                },
                Ok(_) => {
                    info!("{}", format!("Document Not Found at: {}", target).red());
                },
                Err(e) => {
                    error!("Error reading response text for URL {}: {}", target, e);
                }
            }
        }
        Err(e) => {
            error!("Request failed for URL: {}: {}", target, e);
        }
    }
}

async fn ensure_file_exists(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .await
            .expect("Failed to create file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let matches = App::new("Swagger Detection Tool")
        .version("1.0")
        .author("Your Name")
        .about("Checks for Swagger documentation at given host paths")
        .arg(Arg::with_name("hosts")
             .short('h')
             .long("hosts")
             .takes_value(true)
             .required(true)
             .help("Path to the file containing newline-separated host URLs"))
        .arg(Arg::with_name("workers")
             .short('w')
             .long("workers")
             .takes_value(true)
             .help("Number of concurrent workers, default is 10"))
        .arg(Arg::with_name("routefile")
             .short('r')
             .long("routefile")
             .takes_value(true)
             .required(true)
             .help("Path to the file containing Swagger routes"))
        .get_matches();

    let host_file = matches.value_of("hosts").unwrap();
    let worker_count = matches.value_of("workers").unwrap_or("10").parse::<usize>().unwrap_or(10);
    let route_file = matches.value_of("routefile").unwrap();
    let file_path = "found.txt";
    ensure_file_exists(file_path).await;  // Ensure file exists at start-up

    let hosts: Vec<String> = read_lines(host_file).unwrap()
                                    .filter_map(Result::ok)
                                    .collect();
    let doc_routes: Vec<String> = read_lines(route_file).unwrap()
                                    .filter_map(Result::ok)
                                    .collect();
    
    let semaphore = Arc::new(Semaphore::new(worker_count));
    let mut handles = Vec::new();

    for host in hosts {
        for route in &doc_routes {
            let target = format!("https://{}{}", host, route);
            let sem_clone = semaphore.clone();
            handles.push(tokio::spawn(check_host_for_swagger_doc_path(target, sem_clone, file_path.to_string())));
        }
    }

    for handle in handles {
        match handle.await {
            Ok(_) => (),
            Err(e) if e.is_panic() => {
                error!("Task panicked: {:?}", e);
            },
            Err(e) => {
                error!("Task failed with error: {:?}", e);
            }
        }
    }
}

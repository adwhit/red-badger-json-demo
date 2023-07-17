use clap::Parser;
use serde::Deserialize;

use std::{cmp::Ordering, path::PathBuf};

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Table {
    User,
    Role,
    Comment,
    Topic,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(tag = "type", rename_all = "snake_case")]
enum EventType {
    Http {
        path: String,
        method: HttpMethod,
        latency_secs: f64,
    },
    Database {
        table: Table,
        entites_affected: u32,
        latency_secs: f64,
    },
    Log {
        message: String,
    },
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Event {
    timestamp: DateTime,
    event: EventType,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file = std::io::BufReader::new(std::fs::File::open(cli.input_file)?);
    let events: Vec<Event> = serde_json::from_reader(file)?;

    println!("read {} events", events.len());

    let mut latencies = Vec::new();
    for ev in events {
        match ev.event {
            EventType::Http { latency_secs, .. } => latencies.push(latency_secs),
            EventType::Database { .. } => {}
            EventType::Log { .. } => {}
        }
    }
    latencies.sort_by(|l, r| {
        if l < r {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let llen = latencies.len();
    let mean = latencies.iter().sum::<f64>() / llen as f64;
    let median = latencies[llen / 2];
    let nnpct = latencies[(llen / 100) * 99];

    println!("{mean} {median} {nnpct}");
    Ok(())
}

use std::path::PathBuf;

use clap::Parser;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::{thread_rng as rng, Rng};
use serde::Serialize;

#[derive(Parser)]
struct Cli {
    n_lines: usize,
    out_file: PathBuf,
}

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Serialize, FromPrimitive)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Serialize, FromPrimitive)]
#[serde(rename_all = "snake_case")]
enum Table {
    User,
    Role,
    Comment,
    Topic,
}

#[derive(Serialize)]
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

fn rand_str(len: usize) -> String {
    std::iter::repeat_with(|| (rng().gen_range(0..26) + b'a') as char)
        .take(len)
        .collect()
}

#[derive(Serialize)]
struct Event {
    timestamp: DateTime,
    event: EventType,
}

fn random_event_ty() -> EventType {
    let mut rng = rng();
    let roll: f64 = rng.gen();
    if roll < 0.2 {
        EventType::Http {
            path: format!("/{}/{}", rand_str(5), rand_str(10)),
            method: HttpMethod::from_u32(rng.gen_range(0..4)).unwrap(),
            latency_secs: rng.gen(),
        }
    } else if roll < 0.5 {
        EventType::Database {
            table: Table::from_u32(rng.gen_range(0..4)).unwrap(),
            entites_affected: rng.gen_range(0..1000),
            latency_secs: rng.gen(),
        }
    } else {
        EventType::Log {
            message: rand_str(rng.gen_range(30..50)),
        }
    }
}

fn random_event() -> Event {
    let mut rng = rng();
    let event = random_event_ty();
    let timestamp: DateTime = chrono::NaiveDate::from_ymd_opt(2023, 7, 17)
        .and_then(|t| {
            t.and_hms_opt(
                rng.gen_range(0..24),
                rng.gen_range(0..60),
                rng.gen_range(0..60),
            )
        })
        .map(|t| t.and_utc())
        .unwrap();
    Event { event, timestamp }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    println!("create {} rows of data", cli.n_lines);
    let mut events = std::iter::repeat_with(random_event)
        .take(cli.n_lines)
        .collect::<Vec<_>>();
    events.sort_by_key(|k| k.timestamp);
    println!("write to {}", cli.out_file.display());
    let file = std::io::BufWriter::new(std::fs::File::create(&cli.out_file)?);
    serde_json::to_writer_pretty(file, &events)?;
    println!("done");
    Ok(())
}

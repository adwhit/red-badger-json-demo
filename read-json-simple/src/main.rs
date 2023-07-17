use serde_json::Value;
use std::cmp::Ordering;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = std::env::args().into_iter().skip(1).next().unwrap();
    println!("loading file {f}");
    let json = std::fs::read_to_string(f)?;
    let data: Value = serde_json::from_str(&json)?;
    println!("loaded");

    let mut latencies = Vec::new();
    for log in data.as_array().unwrap() {
        let event = &log["event"];
        if event["type"] == "http" {
            latencies.push(event["latency_secs"].as_f64().unwrap());
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

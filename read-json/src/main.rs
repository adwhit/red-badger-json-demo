use serde::Deserialize;

type DateTime = chrono::DateTime<chrono::Utc>;

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

#[derive(Deserialize)]
struct Event {
    timestamp: DateTime,
    event: EventType,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::io::BufReader::new(std::fs::File::open("../log.json")?);
    let events: Vec<Event> = serde_json::from_reader(file)?;
    println!("read {} events", events.len());
    Ok(())
}

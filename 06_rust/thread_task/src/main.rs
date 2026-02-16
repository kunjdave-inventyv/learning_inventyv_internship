use std::{
    sync::{
        Arc,
        RwLock,
        atomic::{AtomicI32, Ordering},
    },
    thread,
    time::Duration,
};

use rand::{distributions::Alphanumeric, Rng};
use chrono::{Local, DateTime, Duration as ChronoDuration};

#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    recordAddedTime: String,
    threadId: String,
}

fn random_thread_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

fn is_older_than_20s(record_time: &str) -> bool {
    if let Ok(t) = DateTime::parse_from_rfc3339(record_time) {
        let local_time = t.with_timezone(&Local);
        return Local::now() - local_time > ChronoDuration::seconds(20);
    }
    false
}

fn main() {
    let records: Arc<RwLock<Vec<MultiThread>>> = Arc::new(RwLock::new(Vec::new()));
    let global_id = Arc::new(AtomicI32::new(1));

    /* Thread 1 — Record Creator */
    {
        let records = Arc::clone(&records);
        let global_id = Arc::clone(&global_id);

        thread::spawn(move || loop {
            let id = global_id.fetch_add(1, Ordering::SeqCst);

            let record = MultiThread {
                id,
                recordAddedTime: Local::now().to_rfc3339(),
                threadId: random_thread_id(),
            };

            {
                let mut data = records.write().unwrap();
                data.push(record);
            }

            println!(" Added record {}", id);
            thread::sleep(Duration::from_secs(10));
        });
    }

    /* Thread 2 — State Printer */
    {
        let records = Arc::clone(&records);

        thread::spawn(move || loop {
            let snapshot = {
                let data = records.read().unwrap();
                data.clone()
            };

            println!("\n Current Records ({}):", snapshot.len());
            for r in snapshot {
                println!("{:?}", r);
            }

            thread::sleep(Duration::from_secs(5));
        });
    }

    /* Thread 3 — Even Record Cleaner */
    {
        let records = Arc::clone(&records);

        thread::spawn(move || loop {
            {
                let mut data = records.write().unwrap();
                data.retain(|r| !(r.id % 2 == 0 && is_older_than_20s(&r.recordAddedTime)));
            }
            thread::sleep(Duration::from_secs(5));
        });
    }

    /* Thread 4 — Odd Record Cleaner */
    {
        let records = Arc::clone(&records);

        thread::spawn(move || loop {
            {
                let mut data = records.write().unwrap();
                data.retain(|r| !(r.id % 2 != 0 && is_older_than_20s(&r.recordAddedTime)));
            }
            thread::sleep(Duration::from_secs(5));
        });
    }

    /* Thread 5 — Even Counter */
    {
        let records = Arc::clone(&records);

        thread::spawn(move || loop {
            let count = {
                let data = records.read().unwrap();
                data.iter().filter(|r| r.id % 2 == 0).count()
            };

            println!(" Even count: {}", count);
            thread::sleep(Duration::from_secs(7));
        });
    }

    /* Thread 6 — Odd Counter */
    {
        let records = Arc::clone(&records);

        thread::spawn(move || loop {
            let count = {
                let data = records.read().unwrap();
                data.iter().filter(|r| r.id % 2 != 0).count()
            };

            println!(" Odd count: {}", count);
            thread::sleep(Duration::from_secs(7));
        });
    }

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

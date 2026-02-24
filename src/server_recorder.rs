use crate::{structs, utils};
use futures::future::join_all;
use sqlx::query;
use std::time::Duration;
use tokio::time::interval;

fn latency_average(latencies: &Vec<f64>) ->f64 {
    let mut sums = 0.0;
    for i in latencies {
        sums += i;
    }
    sums / latencies.len() as f64
}
pub async fn server_recorder(config: structs::RecorderConfig) {
    let server_addr = config.mcserver.address;
    let table = config.mcserver.db_table;
    let times = config.mcserver.times;
    let client = config.client;
    let pool = config.pool;

    let mut ticker = interval(Duration::from_millis(config.interval as u64));
    loop {
        ticker.tick().await;

        let mut futures_vec = Vec::new();
        for _ in 0..times {
            let client_clone = client.clone();
            let server_addr_clone = server_addr.clone();
            let fut = async move {
                let status = client_clone.ping_java(server_addr_clone.as_str()).await;
                let player;
                match status {
                    Ok(status) => {
                        if let Some((online, _)) = status.players() {
                            player = online;
                        } else {
                            eprintln!(
                                "Failed to fetch player count, using default: 0, server addr: {}",
                                server_addr_clone
                            );
                            player = -2
                        }
                        (player, utils::advanced_round(status.latency, 3))
                    }
                    Err(_) => {
                        eprintln!("Failed to ping server, server addr: {}", server_addr_clone);
                        (-1, -1.0)
                    }
                }
            };
            futures_vec.push(fut);
        }

        let results = join_all(futures_vec).await;

        let mut players = Vec::new();
        let mut latencies = Vec::new();

        for (player, latency) in results {
            players.push(player);
            latencies.push(latency);
        }

        let player = *players.iter().max().unwrap();
        let latency = latency_average(&latencies);

        let s = query(
            format!(
                r#"
INSERT INTO {table} (latency, players) VALUES (?, ?)
        "#
            )
            .as_str(),
        )
        .bind(latency)
        .bind(player)
        .execute(&pool)
        .await;
        match s {
            Ok(a) => {
                if a.rows_affected() != 1 {
                    println!(
                        "something wrong when inserting data, {} lines affected",
                        a.rows_affected()
                    );
                }
            }
            Err(e) => {
                println!("Failed to insert data: {}", e);
            }
        }
    }
}

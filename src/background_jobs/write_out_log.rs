use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use chrono::Utc;
use inline_colorization::*;
use tokio::time::sleep;

use crate::{gachaplus_database::short_log_table::ShortLog, http_handler::AppState};

pub async fn write_out_log_service(app_state: Arc<AppState>) {
    loop {
        let mut current_logs: Vec<ShortLog> = Vec::new();
        {
            let mut logs = app_state.log_queue.lock().await;
            logs.clone_into(&mut current_logs);
            logs.clear();
        }
        let mut delays_in_ms: Vec<f64> = Vec::new();
        for log in current_logs.iter() {
            let now = Instant::now();
            let result = app_state.database.short_log_table.insert_log(log).await;
            let delay_in_ms = now.elapsed().as_micros() as f64 / 1000f64;
            delays_in_ms.push(delay_in_ms);
            if let Err(log_error) = result {
                println!(
                    "{}{}\tLogging: Error at updating logs: {:?}{}",
                    color_yellow,
                    Utc::now().format("[%H:%M:%S]"),
                    log_error,
                    color_white,
                );
            }
        }

        if current_logs.len() > 0 {
            println!(
                "{}{}\tLogging: {} log row added!\tAVG delay: {:.3} ms\tAll delay: {:.3} ms{}",
                color_bright_black,
                Utc::now().format("[%H:%M:%S]"),
                current_logs.len(),
                median(&mut delays_in_ms).unwrap_or_default(),
                delays_in_ms.iter().sum::<f64>(),
                color_white,
            );
        }

        sleep(Duration::from_secs(15)).await;
    }
}

fn median(numbers: &mut Vec<f64>) -> Option<f64> {
    let len = numbers.len();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    match len {
        0 => None,
        _ => {
            let mid = len / 2;
            if len % 2 == 0 {
                Some((numbers[mid - 1] + numbers[mid]) / 2.0)
            } else {
                Some(numbers[mid])
            }
        }
    }
}

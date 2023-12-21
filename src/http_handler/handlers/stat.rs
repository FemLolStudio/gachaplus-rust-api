use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    extract::{Query, State},
    response,
};
use build_html::*;
use chrono::Utc;
use memory_stats::memory_stats;
use serde::Deserialize;
use sysinfo::{Component, Components, Disks, NetworkData, Networks, Pid, Process, System};
use thousands::Separable;
use tokio::sync::Mutex;

use crate::{enviorment, http_handler::AppState};

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;

#[derive(Deserialize)]
pub struct QueryParams {
    pub password: Option<String>,
    pub refresh: Option<bool>,
}

#[axum::debug_handler]
pub async fn get_info(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> response::Html<String> {
    let password = enviorment::get_enviorment("PASSWORD");
    if params.password.is_none() || params.password.unwrap() != password {
        return response::Html(String::from("<h1 align='center'>Bad password</h1>"));
    }

    let sys = System::new_all();
    let mut app_table: Vec<[String; 2]> = Vec::new();
    let mut ratelimit_table: Vec<[String; 2]> = Vec::new();
    let mut memory_table: Vec<[String; 2]> = Vec::new();
    let mut sysmemory_table: Vec<[String; 2]> = Vec::new();
    let mut systeminfo_table: Vec<[String; 2]> = Vec::new();
    let mut tempreture_table: Vec<[String; 2]> = Vec::new();
    let mut network_table: Vec<[String; 2]> = Vec::new();
    let mut disk_table: Vec<[String; 4]> = Vec::new();
    let mut processes_table: Vec<[String; 3]> = Vec::new();

    //---------------------------------------------------------

    {
        app_table.push([
            "Free OCs".to_owned(),
            format!(
                "{}",
                app_state
                    .oc_chache
                    .read()
                    .await
                    .len()
                    .separate_with_spaces()
            ),
        ]);
        app_table.push([
            "Logs".to_owned(),
            format!(
                "{}",
                app_state
                    .log_queue
                    .lock()
                    .await
                    .len()
                    .separate_with_spaces()
            ),
        ]);

        let hmap_vec: Vec<&(Mutex<HashMap<String, Instant>>, Duration)> =
            app_state.rate_limit.iter().map(|a| a.1).collect();
        let mut db = 0;
        for (hmap, _) in hmap_vec {
            db += hmap.lock().await.len();
        }
        app_table.push([
            "Stored IPs".to_owned(),
            format!("{}", db.separate_with_spaces()),
        ]);
    }
    //---------------------------------------------------------

    for (limit_path, (_, limit_duration)) in app_state.rate_limit.iter() {
        ratelimit_table.push([
            limit_path.to_owned().to_owned(),
            format!("{:.1} s", limit_duration.as_secs_f64()),
        ]);
    }
    ratelimit_table.push([escape_html("<other>"), "∞ s".to_owned()]);

    //---------------------------------------------------------

    memory_table.push(["Used physical memory".to_owned(), escape_html("<unknown>")]);
    memory_table.push(["Used virtual memory".to_owned(), escape_html("<unknown>")]);

    if let Some(usage) = memory_stats() {
        memory_table[0][1] = format!(
            "{} MB",
            (usage.physical_mem / MB as usize).separate_with_spaces()
        );
        memory_table[1][1] = format!(
            "{} MB",
            (usage.virtual_mem / MB as usize).separate_with_spaces()
        );
    }

    //---------------------------------------------------------

    sysmemory_table.push([
        "Total memory".to_owned(),
        format!("{} MB", (sys.total_memory() / MB).separate_with_spaces()),
    ]);
    sysmemory_table.push([
        "Used memory".to_owned(),
        format!("{} MB", (sys.used_memory() / MB).separate_with_spaces()),
    ]);
    sysmemory_table.push([
        "Total swap".to_owned(),
        format!("{} MB", (sys.total_swap() / MB).separate_with_spaces()),
    ]);
    sysmemory_table.push([
        "Used swap".to_owned(),
        format!("{} MB", (sys.used_swap() / MB).separate_with_spaces()),
    ]);

    //---------------------------------------------------------

    systeminfo_table.push([
        "System name".to_owned(),
        System::name().unwrap_or(escape_html("<unknown>")),
    ]);
    systeminfo_table.push([
        "System kernel version".to_owned(),
        System::kernel_version().unwrap_or(escape_html("<unknown>")),
    ]);
    systeminfo_table.push([
        "System host name".to_owned(),
        System::host_name().unwrap_or(escape_html("<unknown>")),
    ]);
    systeminfo_table.push(["Number of CPUs".to_owned(), format!("{}", sys.cpus().len())]);

    //---------------------------------------------------------

    let components = Components::new_with_refreshed_list();
    let mut components: Vec<&Component> = components.iter().collect();
    components.sort_by(|a, b| a.label().cmp(b.label()));
    for component in components {
        //println!("{:?}", component);
        tempreture_table.push([
            component.label().to_owned(),
            format!("{:.1} °C", component.temperature()),
        ])
    }

    //---------------------------------------------------------

    // Network interfaces name, data received and data transmitted:
    let networks = Networks::new_with_refreshed_list();
    let mut networks: Vec<(&String, &NetworkData)> = networks.into_iter().collect();
    networks.sort_by(|a, b| b.1.total_transmitted().cmp(&a.1.total_transmitted()));

    for (interface_name, data) in networks {
        network_table.push([
            interface_name.to_owned(),
            format!(
                "{}/{} KB",
                (data.received() / KB).separate_with_spaces(),
                (data.packets_transmitted() / KB).separate_with_spaces()
            ),
        ]);
    }

    //---------------------------------------------------------

    // We display all disks' information:
    for disk in Disks::new_with_refreshed_list().iter() {
        //println!("{:?}", disk);
        disk_table.push([
            disk.name().to_string_lossy().to_string(),
            disk.mount_point().to_string_lossy().to_string(),
            String::from_utf8(disk.file_system().as_encoded_bytes().to_vec())
                .unwrap_or(escape_html("<unknown>")),
            format!(
                "{}/{} GB <i>({} GB available)</i>",
                ((disk.total_space() - disk.available_space()) / GB).separate_with_spaces(),
                (disk.total_space() / GB).separate_with_spaces(),
                (disk.available_space() / GB).separate_with_spaces()
            ),
        ]);
    }

    //---------------------------------------------------------

    let mut processes: Vec<(&Pid, &Process)> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));

    for (pid, process) in processes {
        processes_table.push([
            format!("[{}]", pid),
            process.name().to_owned(),
            format!("{} MB", (process.memory() / MB).separate_with_spaces()),
        ]);
    }

    //---------------------------------------------------------

    for row in app_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in ratelimit_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in systeminfo_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in memory_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in sysmemory_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in tempreture_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in network_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in disk_table.iter_mut() {
        row[0] = format!("<b>{}</b>", row[0]);
    }
    for row in processes_table.iter_mut() {
        row[1] = format!("<b>{}</b>", row[1]);
    }

    //---------------------------------------------------------
    let mut html_page = HtmlPage::new()
        .with_title("Gacha Plus")
        .with_header(1, "Infos")
        .with_header(3, Utc::now().format("%Y.%m.%d. %H:%M:%S (UTC)"))
        .with_container(
            Container::new(ContainerType::Main)
                .with_header(2, "App info")
                .with_table(Table::from(app_table))
                .with_header(2, "Rate limits")
                .with_table(Table::from(ratelimit_table))
                .with_header(2, "App memory info")
                .with_table(Table::from(memory_table))
                .with_header(2, "System memory info")
                .with_table(Table::from(sysmemory_table))
                .with_header(2, "System info")
                .with_table(Table::from(systeminfo_table))
                .with_header(2, "Tempreture info")
                .with_table(Table::from(tempreture_table))
                .with_header(2, "Network info")
                .with_table(Table::from(network_table))
                .with_header(2, "Disk info")
                .with_table(Table::from(disk_table).with_header_row([
                    "Name",
                    "Path",
                    "File system",
                    "Disk space",
                ]))
                .with_header(2, "Processes")
                .with_table(Table::from(processes_table).with_header_row([
                    "PID",
                    "Process name",
                    "Memory used (RAM)",
                ])),
        );

    html_page.add_head_link("files/style.css", "stylesheet");
    html_page.add_head_link_attr("files/icon.png", "icon", [("type", "image/png")]);

    if params.refresh.unwrap_or(false) {
        html_page.add_script_literal(
            r#"
        setTimeout(function() {
            // Az oldal újratöltése
            location.reload();
        }, 5000); // 3000 milliszekundum = 3 másodperc
        "#,
        );
    }
    response::Html(html_page.to_html_string())
}

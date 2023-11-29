#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use chrono::Utc;
use inline_colorization::*;
use std::net::SocketAddr;

mod background_jobs;
mod character_code;
mod enviorment;
mod gachaplus_database;
mod http_handler;
mod tests;

use http_handler::AppState;

#[tokio::main]
async fn main() {
    run_server().await;
}
pub async fn run_server() {
    //checking enviorments
    _ = dotenv::dotenv();
    let database_url = enviorment::get_enviorment("DATABASE_URL");
    _ = enviorment::get_enviorment("PASSWORD");

    // build our application with a route
    let app_state = AppState::new(database_url).await;
    let app = http_handler::create_router(app_state.clone()).await;

    //starting bg services
    background_jobs::start(app_state);

    //opening port
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("{color_cyan}{}{color_green}\tWebszerver: 💚 Listening on '{color_cyan}{addr}{color_green}' 💚{color_white}",
        Utc::now().format("[%H:%M:%S]"),
    );

    //running server
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

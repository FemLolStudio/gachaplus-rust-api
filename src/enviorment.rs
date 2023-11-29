use chrono::Utc;
use inline_colorization::*;

pub fn get_enviorment(key: &str) -> String {
    let res = std::env::var(key);
    match res {
        Ok(envi) => {
            println!("{color_cyan}{}{color_green}\tEnviorment: ✅ Getting '{color_cyan}{key}{color_green}' enviorment is successful! ✅{color_white}", Utc::now().format("[%H:%M:%S]"));
            envi
        }
        Err(err) => {
            println!(
                "{color_red}{}\tEnviorment: 🔥 Failed to get '{key}' enviorment: {:?} 🔥{color_white}",
                Utc::now().format("[%H:%M:%S]"),
                err
            );
            std::process::exit(1);
        }
    }
}

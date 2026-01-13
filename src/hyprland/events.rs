use std::{
    env,
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
};
use crate::logger::Logger;

pub fn listen_active_window<F>(mut handler: F)
where
    F: FnMut(String, String),
{
    let runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR not set");
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE not set");

    Logger::log(&format!("Runtime: {}, Signature: {}", runtime, sig));

    let path = format!("{runtime}/hypr/{sig}/.socket2.sock");
    let stream = UnixStream::connect(path).expect("Failed to connect to Hyprland socket");

    let reader = BufReader::new(stream);

    for line in reader.lines().flatten() {
        if let Some(data) = line.strip_prefix("activewindow>>") {
            let mut parts = data.splitn(2, ',');

            let class = parts.next().unwrap_or("").to_string();
            let title = parts.next().unwrap_or("").to_string();

            Logger::log(&format!("Current class: {}, current title: {}", class, title));

            handler(class, title);
        }
    }
}

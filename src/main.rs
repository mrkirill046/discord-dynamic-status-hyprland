mod config;
mod discord;
mod hyprland;
mod rules;
mod logger;
mod constants;

use discord::rpc::DiscordRpc;
use hyprland::events::listen_active_window;
use logger::Logger;

fn main() {
    Logger::log("Starting application...");

    let config = config::Config::load();

    Logger::log("Config loaded successfully!");

    let mut rpc = DiscordRpc::new(&config.app_id);

    rpc.connect();

    Logger::log("Connected to Discord successfully!");

    listen_active_window(|class, title| {
        let rule = rules::resolve_rule(&config, &class);

        rpc.update(rule, &title);
    });
}

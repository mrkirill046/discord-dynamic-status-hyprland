use crate::config::RpcRule;
use crate::logger::Logger;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub struct DiscordRpc {
    client: DiscordIpcClient,
}

impl DiscordRpc {
    pub fn new(client_id: &str) -> Self {
        Self {
            client: DiscordIpcClient::new(client_id),
        }
    }

    pub fn connect(&mut self) {
        self.client.connect().unwrap();
    }

    pub fn update(&mut self, rule: &RpcRule, title: &str) {
        let mut act = activity::Activity::new();

        if let Some(state) = &rule.state {
            act = act.state(state);
        }

        if rule.details_from_title.unwrap_or(false) {
            act = act.details(title);
        } else if let Some(details) = &rule.details {
            act = act.details(details);
        }

        if rule.large_image.is_some() || rule.small_image.is_some() {
            let mut assets = activity::Assets::new();

            if let Some(v) = &rule.large_image {
                assets = assets.large_image(v);
            }
            if let Some(v) = &rule.large_text {
                assets = assets.large_text(v);
            }
            if let Some(v) = &rule.small_image {
                assets = assets.small_image(v);
            }
            if let Some(v) = &rule.small_text {
                assets = assets.small_text(v);
            }

            act = act.assets(assets);
        }

        Logger::log(&format!(
            "[RPC] state={:?}, details={:?}, large_image={:?}, large_text={:?}, small_image={:?}, small_text={:?}",
            rule.state,
            if rule.details_from_title.unwrap_or(false) {
                Some(title)
            } else {
                rule.details.as_deref()
            },
            rule.large_image,
            rule.large_text,
            rule.small_image,
            rule.small_text
        ));

        let _ = self.client.set_activity(act);
    }
}

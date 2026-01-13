use crate::config::{Config, RpcRule};

pub fn resolve_rule<'a>(config: &'a Config, class: &str) -> &'a RpcRule {
    config.classes.get(class).unwrap_or(&config.default)
}

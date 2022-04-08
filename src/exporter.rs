use crate::config;
use crate::constants;

use lazy_static::lazy_static;
use log::{error, info};
use prometheus::{GaugeVec, IntCounterVec, IntGaugeVec, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
}

pub fn register() {}

fn update_metrics(cfg: &config::Configuration) {}

pub fn serve_metrics(cfg: &config::Configuration) -> String {
    update_metrics(cfg);

    let encoder = prometheus::TextEncoder::new();
    let mut buffer = String::new();

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    }

    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    };
    buffer
}

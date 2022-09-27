use std::os::linux::raw::stat;
use once_cell::sync::Lazy;
use prometheus::{exponential_buckets, HistogramOpts, HistogramTimer, HistogramVec, IntCounter, IntGauge, linear_buckets, Opts};
use prometheus::local::LocalHistogram;
use prometheus::Registry;

pub const SERVER_NAMESPACE: &str = "server";

pub const TSKV_SUBSYSTEM: &str = "tskv";

pub const QUERY_SUBSYSTEM: &str = "query";

pub static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

pub static KV_READ_SUCCESS: Lazy<IntCounter> = Lazy::new( || {
    IntCounter::with_opts(
        Opts::new(
            "kv_read_success_total",
            "Number of read requests",
        ).namespace(SERVER_NAMESPACE)
            .subsystem(TSKV_SUBSYSTEM),
    ).expect("ts kv metric cannot be created")
});

pub static KV_READ_FAILED: Lazy<IntCounter> = Lazy::new( || {
    IntCounter::with_opts(
        Opts::new(
            "kv_read_failure_total",
            "Number of tskv read failures",
        ).namespace(SERVER_NAMESPACE)
            .subsystem(TSKV_SUBSYSTEM),
    ).expect("tskv metric cannot be created")
});

pub static KV_WRITE_SUCCESS: Lazy<IntCounter> = Lazy::new( || {
    IntCounter::with_opts(
        Opts::new(
            "kv_write_success_total",
            "Number of tskv read success",
        ).namespace(SERVER_NAMESPACE)
            .subsystem(TSKV_SUBSYSTEM),
    ).expect("tskv metric cannot be created")
});

pub static KV_WRITE_FAILED: Lazy<IntCounter> = Lazy::new(|| {
    IntGauge::with_opts(
        Opts::new(
            "kv_write_failure_total",
            "Number of tskv write failed",
        ).namespace(SERVER_NAMESPACE)
            .subsystem(TSKV_SUBSYSTEM),
    ).expect("tskv metric cannot be created")
});

pub static KV_READ_LATENCY: Lazy<HistogramVec> = Lazy::new( || {
    HistogramVec::new(
        HistogramOpts::new(
            "kv_read_milliseconds",
            "Total latency distribution of kv read"
        ).namespace(SERVER_NAMESPACE)
            .subsystem(TSKV_SUBSYSTEM)
            .buckets(linear_buckets(0.0, 10.0, 200).unwrap()),
        &["status"],
    ).expect("tskv metric cannot be created")
});

pub static KV_WRITE_LATENCY: Lazy<HistogramVec> = Lazy::new( || {
    HistogramVec::new(
        HistogramOpts::new(
            "kv_write_milliseconds",
            "Total latency distribution of kv write"
        ).namespace(SERVER_NAMESPACE)
            .subsystem(TSKV_SUBSYSTEM)
            .buckets(linear_buckets(0.0, 10.0, 200).unwrap()),
        &["status"],
    ).expect("tskv metric cannot be created")
});

fn init_tskv_recorder(){
    REGISTRY.register(Box::new(KV_READ_LATENCY.clone()))
        .expect("tskv metrics collector cannot be registered");

    REGISTRY.register(Box::new(KV_WRITE_LATENCY.clone()))
        .expect("tskv metrics collector cannot be registered");

    REGISTRY.register(Box::new(KV_READ_SUCCESS.clone()))
        .expect("tskv metrics collector cannot be registered");

    REGISTRY.register(Box::new(KV_READ_FAILED.clone()))
        .expect("tskv metrics collector cannot be registered");

    REGISTRY.register(Box::new(KV_WRITE_SUCCESS.clone()))
        .expect("tskv metrics collector cannot be registered");

    REGISTRY.register(Box::new(KV_WRITE_FAILED.clone()))
        .expect("tskv metrics collector cannot be registered");
}

pub fn sample_tskv_metrics_read_latency(status: String, delta: f64) {
    KV_READ_LATENCY.with_label_values(&[&status]).observe(delta)
}

pub fn sample_tskv_metrics_write_latency(status: String, delta: f64) {
    KV_READ_LATENCY.with_label_values(&[&status]).observe(delta)
}

pub fn incr_tskv_read_success_operation() {
    KV_READ_SUCCESS.inc()
}

pub fn incr_tskv_read_failed_operation(){
    KV_READ_FAILED.inc()
}

pub fn incr_tskv_write_failed_operation(){
    KV_WRITE_FAILED.inc()
}

pub fn incr_tskv_write_success_operation(){
    KV_WRITE_SUCCESS.inc();
}
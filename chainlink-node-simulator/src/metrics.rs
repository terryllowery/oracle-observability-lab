use prometheus::{Counter, Encoder, Gauge, Histogram, HistogramOpts, Opts, Registry, TextEncoder};
use std::sync::Arc;

#[derive(Clone)]
pub struct NodeMetrics {
    registry: Registry,

    // Request counters
    pub request_total: Counter,
    pub requests_success: Counter,
    pub requests_failed: Counter,

    // Latency histogram
    pub request_duration: Histogram,

    // Node state gauges
    pub reputation_score: Gauge,
    pub eth_balance: Gauge,
    pub link_balance: Gauge,
}

impl NodeMetrics {
    pub fn new(node_id: &str) -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        // Total requests counter
        let requests_total = Counter::with_opts(
            Opts::new("oracle_requests_total", "Total number of oracle requests")
                .const_label("node_id", node_id),
        )?;
        registry.register(Box::new(requests_total.clone()))?;

        // Success counter
        let requests_success = Counter::with_opts(
            Opts::new(
                "oracle_requests_success_total",
                "Successful oracle requests",
            )
            .const_label("node_id", node_id),
        )?;
        registry.register(Box::new(requests_success.clone()))?;

        // Failed counter
        let requests_failed = Counter::with_opts(
            Opts::new("oracle_requests_failed_total", "Failed oracle requests")
                .const_label("node_id", node_id),
        )?;
        registry.register(Box::new(requests_failed.clone()))?;

        // Reques duration histogram
        // Buckets: 1s, 2s, 5s, 10s, 20s, 30s, 60s
        let request_duration = Histogram::with_opts(
            HistogramOpts::new(
                "oracle_request_duration_seconds",
                "Oracle request duration in seconds",
            )
            .const_label("node_id", node_id)
            .buckets(vec![1.0, 2.0, 5.0, 10.0, 20.0, 30.0, 60.0]),
        )?;
        registry.register(Box::new(request_duration.clone()))?;

        // Reputation score gauge (0-100)
        let reputation_score = Gauge::with_opts(
            Opts::new("oracle_reputation_score", "Node reputation score (0-100)")
                .const_label("node_id", node_id),
        )?;
        registry.register(Box::new(reputation_score.clone()))?;

        // ETH balance
        let eth_balance = Gauge::with_opts(
            Opts::new("oracle_eth_balance", "Node ETH balance").const_label("node_id", node_id),
        )?;
        registry.register(Box::new(eth_balance.clone()))?;

        // LINK balance
        let link_balance = Gauge::with_opts(
            Opts::new("oracle_link_balance", "Node LINK token balance")
                .const_label("node_id", node_id),
        )?;
        registry.register(Box::new(link_balance.clone()))?;
        link_balance.set(500.0); // Start with 500 LINK

        Ok(Self {
            registry,
            request_total: requests_total,
            requests_success,
            requests_failed,
            request_duration,
            reputation_score,
            eth_balance,
            link_balance,
        })
    }

    pub fn export(&self) -> Result<String, prometheus::Error> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        Ok(String::from_utf8(buffer).unwrap())
    }
}

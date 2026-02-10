use crate::metrics::NodeMetrics;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::interval;
use rand::{Rng, RngExt};

// Quality tier determines node behavior
pub enum QualityTier {
    High,     // 99.9% uptime, fast response
    Medium,   // 98% uptime, medium response
    Low,      // 95% uptime, slow response
}

pub struct OracleNode {
    node_id: String,
    quality: QualityTier,
    metrics: Arc<NodeMetrics>,
}

impl OracleNode {
    pub fn new(node_id: String, quality: QualityTier, metrics: Arc<NodeMetrics>) 
    -> Self {
        Self {
            node_id,
            quality,
            metrics,
        }
    }

    // Simulate processing an oracle request in background
    pub async fn start_request_loop(self: Arc<Self>) {
        tokio::spawn(async move {
        // 5 second request interval
        let mut interval = interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            self.handle_request().await;
        }
    });
}
    async fn handle_request(&self) {
       let start = Instant::now();

       self.metrics.request_total.inc();

       let should_fail = self.should_fail();
       let delay = self.simulate_delay();

       // simulate external API call delay
       tokio::time::sleep(Duration::from_millis(delay)).await;

       if should_fail {
        // Request failed
        self.metrics.requests_failed.inc();

        // Degrade reputation score on failure
        let current_score = self.metrics.reputation_score.get();
        self.metrics.reputation_score.set((current_score - 0.1).max(0.0));

        tracing::warn!("Node {} failed to process request", self.node_id);
       } else {
            // Request succeeded
            self.metrics.requests_success.inc();

            // Improve reputation score on success
          let reward = 0.1; // 0.1 LINK per request
          let current_link = self.metrics.link_balance.get();
          self.metrics.link_balance.set(current_link + reward); 
       }
       // Record request duration
       let duration = start.elapsed();
       self.metrics.request_duration.observe(duration.as_secs_f64());

       // sim fas costs (decrease ETH balance)
       let gas_cost = 0.0001; // 0.0001 ETH per request
       let current_eth = self.metrics.eth_balance.get();
         self.metrics.eth_balance.set((current_eth - gas_cost).max(0.0));


    }

    // Determine if the request should fail based on quality tier
    fn should_fail(&self) -> bool {
        let mut rng = rand::rng();
        match self.quality {
            QualityTier::High => rng.random_bool(0.01), // 0.1% failure
            QualityTier::Medium => rng.random_bool(0.02), // 2% failure
            QualityTier::Low => rng.random_bool(0.05),    // 5% failure
        }
    }

    // Simulate response delay based on quality tier
    fn simulate_delay(&self) -> u64 {
        let mut rng = rand::rng();
        match self.quality {
            QualityTier::High => rng.random_range(500..2000),
            QualityTier::Medium => rng.random_range(2000..8000),
            QualityTier::Low => rng.random_range(5000..15000),   // 100-
        }
    }
}

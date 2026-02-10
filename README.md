# Oracle Network Observability Lab

A hands-on simulation environment demonstrating observability best practices for decentralized oracle networks. Built with Rust, Prometheus, and Grafana.

## What This Is

This project simulates a decentralized oracle network with multiple node operators, each with different reliability characteristics. It implements production-grade monitoring, SLO/SLI tracking, and alerting patterns commonly used in distributed systems.

**What are blockchain oracles?** They're services that fetch real-world data (prices, weather, etc.) and deliver it to smart contracts on blockchains. Since smart contracts can't directly access external APIs, they rely on decentralized oracle networks for this data.

## Quick Start

### Prerequisites

- Docker & Docker Compose
- 4GB RAM minimum

### Running the Lab

```bash
git clone https://github.com/yourusername/oracle-observability-lab
cd oracle-observability-lab

docker-compose up -d
docker-compose ps
```

### Access Points

- Grafana: http://localhost:3000 (admin/admin)
- Prometheus: http://localhost:9090
- Node 1 metrics: http://localhost:9091/metrics
- Node 2 metrics: http://localhost:9092/metrics  
- Node 3 metrics: http://localhost:9093/metrics

Open Grafana, go to Dashboards → Browse, and check out "Oracle Node Health" or "SLO Compliance".

## Testing Scenarios

### Simulate a Node Failure

```bash
docker-compose stop node-3

# Watch in Grafana:
# - Node goes offline
# - Alerts fire in Prometheus
# - Error budget starts burning
```

### Restart and Recover

```bash
docker-compose start node-3

# Observe recovery:
# - Metrics return to normal
# - Reputation score rebuilds over time
```

### Compare Node Behaviors

The three nodes simulate different quality levels:
- **Node 1 (High):** 99.9% uptime, fast responses (~1s)
- **Node 2 (Medium):** 98% uptime, medium responses (~5s)
- **Node 3 (Low):** 95% uptime, slow responses (~10s), occasional bad data

## Metrics & Monitoring

### What Gets Tracked

Each node exposes:
- Request counters (total, success, failed)
- Latency histograms (p50, p95, p99 response times)
- Reputation scores (performance-based, 0-100)
- Resource balances (simulated ETH for gas, LINK for rewards)
- Transaction rates (blockchain submission success/failure)

### SLO Tracking

The system tracks 7 critical service level indicators:

- **Availability:** 99.9% uptime - Node must respond to requests
- **Response Time (p95):** <30s - DeFi needs real-time data
- **Data Accuracy:** 99.99% - Wrong data = contract failures
- **Transaction Success:** 99.5% - Failed txs waste gas
- **Consensus Rate:** 99.9% - Nodes must agree on data
- **Data Freshness:** <60s - Stale data creates arbitrage
- **Gas Efficiency:** <200k gas - Keeps operation costs low

### Alerting

Multi-burn-rate alerts fire at different severities:
- **Critical:** Fast error budget burn (page immediately)
- **Warning:** Medium burn rate (investigate next day)
- **Info:** Slow burn (ticket for later)

## Architecture

```
Oracle Nodes (3 simulators)
  ↓ Expose /metrics endpoints
Prometheus
  ↓ Scrapes every 15s, calculates SLIs, evaluates alerts
Grafana
  ↓ Dashboards for node health, SLO compliance, alerts
```

## Configuration

### Adjust Node Behavior

Edit `docker-compose.yml`:

```yaml
environment:
  - QUALITY_TIER=high  # Options: high, medium, low
```

### Modify SLO Targets

Edit `prometheus/rules/slo_alert_rules.yml` to adjust thresholds.

### Customize Dashboards

Dashboards are in `grafana/dashboards/` as JSON files. Import, modify in Grafana UI, then export back to JSON.

## What This Demonstrates

- Prometheus instrumentation patterns
- PromQL queries for recording rules and alerts
- SLO/SLI design for distributed systems
- Error budget tracking
- Multi-burn-rate alerting
- Effective dashboard design

## Project Structure

```
oracle-observability-lab/
├── README.md
├── docker-compose.yml
├── oracle-node-simulator/          # Rust application
│   ├── src/
│   └── Dockerfile
├── prometheus/
│   ├── prometheus.yml
│   └── rules/                      # Recording & alert rules
└── grafana/
    ├── provisioning/
    └── dashboards/
```

## Development

### Build Locally

```bash
cd oracle-node-simulator
cargo build --release
cargo test
```

### Run a Single Node

```bash
export NODE_ID=node-1
export QUALITY_TIER=high
cargo run
```

### Reload Prometheus Config

```bash
# After editing rules:
curl -X POST http://localhost:9090/-/reload
```

## Use Cases

This lab is useful for:
- Learning SRE practices hands-on
- Practicing Prometheus/Grafana configuration
- Understanding distributed system patterns
- Interview preparation
- Workshop examples

## References

- [Google SRE Book - SLOs](https://sre.google/sre-book/service-level-objectives/)
- [Multi-Window Multi-Burn-Rate Alerts](https://sre.google/workbook/alerting-on-slos/)
- [Prometheus Best Practices](https://prometheus.io/docs/practices/)
- [Blockchain Oracles Explained](https://ethereum.org/en/developers/docs/oracles/)

## License

MIT License

---

Built by Terry Lowery | [LinkedIn](https://www.linkedin.com/in/terrylowery/)

**Note:** This is a simulation for learning purposes. It does not connect to real blockchains or oracle networks.

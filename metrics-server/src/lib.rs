use std::net::SocketAddr;

use prometheus_client::encoding::text::{Encode, EncodeMetric, Encoder};
use prometheus_client::registry::Registry;

use parking_lot::RwLock;
use tokio_metrics::TaskMonitor;

use crate::chain::BlockMetrics;
use crate::consensus::ConsensusMetrics;
use crate::mempool::MempoolMetrics;
use crate::network::NetworkMetrics;
use crate::server::metrics_server;
#[cfg(tokio_unstable)]
use crate::tokio_runtime::TokioRuntimeMetrics;
use crate::tokio_task::TokioTaskMetrics;
use nimiq_blockchain_proxy::BlockchainProxy;
use nimiq_consensus::ConsensusProxy;
use nimiq_mempool::mempool::Mempool;
use nimiq_network_interface::network::Network;
use prometheus_client::metrics::MetricType;
use std::sync::Arc;
#[cfg(tokio_unstable)]
use tokio_metrics::RuntimeMonitor;

mod chain;
mod consensus;
mod mempool;
mod network;
mod server;
#[cfg(tokio_unstable)]
mod tokio_runtime;
mod tokio_task;

#[derive(Clone)]
pub struct NimiqTaskMonitor {
    pub name: String,
    pub monitor: TaskMonitor,
}

struct NumericClosureMetric<T: Encode + Sized> {
    metric_type: MetricType,
    lambda: Box<dyn Fn() -> T + Sync + Send>,
}

impl<T: Encode + Sized> NumericClosureMetric<T> {
    fn new(
        metric_type: MetricType,
        lambda: Box<dyn Fn() -> T + Sync + Send>,
    ) -> NumericClosureMetric<T> {
        NumericClosureMetric {
            metric_type,
            lambda,
        }
    }

    pub fn new_gauge(lambda: Box<dyn Fn() -> T + Sync + Send>) -> NumericClosureMetric<T> {
        NumericClosureMetric::new(MetricType::Gauge, lambda)
    }
}

impl<T: Encode + Sized> EncodeMetric for NumericClosureMetric<T> {
    fn encode(&self, mut encoder: Encoder) -> Result<(), std::io::Error> {
        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value((self.lambda)())?
            .no_exemplar()?;

        Ok(())
    }

    fn metric_type(&self) -> prometheus_client::metrics::MetricType {
        self.metric_type
    }
}

pub fn start_metrics_server<TNetwork: Network>(
    addr: SocketAddr,
    blockchain_proxy: BlockchainProxy,
    mempool: Option<Arc<Mempool>>,
    consensus_proxy: ConsensusProxy<TNetwork>,
    network: Arc<nimiq_network_libp2p::Network>,
    task_monitors: &[NimiqTaskMonitor],
) {
    let mut registry = Registry::default();
    let nimiq_registry = registry.sub_registry_with_prefix("nimiq");

    BlockMetrics::register(nimiq_registry, blockchain_proxy);
    ConsensusMetrics::register(nimiq_registry, consensus_proxy);
    NetworkMetrics::register(nimiq_registry, network);

    if let Some(mempool) = mempool {
        MempoolMetrics::register(nimiq_registry, mempool);
    }

    // Setup the task metrics
    let task_metrics = Arc::new(RwLock::new(TokioTaskMetrics::new()));
    task_metrics.write().register(
        nimiq_registry,
        &task_monitors
            .iter()
            .map(|e| e.name.clone())
            .collect::<Vec<String>>()[..],
    );

    #[cfg(tokio_unstable)]
    {
        // Setup the tokio runtime metrics
        let handle = tokio::runtime::Handle::current();
        let tokio_rt_monitor = RuntimeMonitor::new(&handle);
        let mut tokio_rt_metrics = TokioRuntimeMetrics::new();
        tokio_rt_metrics.register(nimiq_registry);
        let tokio_rt_metrics = Arc::new(RwLock::new(tokio_rt_metrics));

        // Spawn Tokio runtime metrics updater
        tokio::spawn(TokioRuntimeMetrics::update_metric_values(
            tokio_rt_metrics,
            tokio_rt_monitor,
        ));
    }

    // Spawn the metrics server
    tokio::spawn(metrics_server(addr, registry));

    // Spawn Tokio task monitor updaters
    for i in 0..task_monitors.len() {
        let task_monitors = task_monitors.to_vec();
        tokio::spawn({
            let task_metrics = Arc::clone(&task_metrics);
            let task_monitors = task_monitors;
            async move {
                TokioTaskMetrics::update_metric_values(
                    task_metrics,
                    &task_monitors[i].name,
                    task_monitors[i].monitor.clone(),
                )
                .await;
            }
        });
    }
}

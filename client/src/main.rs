use structopt::StructOpt;
use std::time::Duration;
use log::info;
use nimiq::prover::prover_main;
use nimiq::client::Client;
use nimiq::config::{command_line::CommandLine, config::ClientConfig, config_file::ConfigFile};
use nimiq::error::Error;
use nimiq::extras::{
    logging::{initialize_logging, log_error_cause_chain},
    metrics_server::NimiqTaskMonitor,
    panic::initialize_panic_reporting,
    signal_handling::initialize_signal_handler,
};
use tokio_metrics::TaskMonitor;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    statistics: Option<u64>,
}

async fn main_inner(test_interval: Option<u64>) -> Result<(), Error> {
    // Parse command line.
    let command_line = CommandLine::parse();
    log::trace!("Command line: {:#?}", command_line);

    // Parse config file - this will obey the `--config` command line option.
    let config_file = ConfigFile::find(Some(&command_line))?;
    log::trace!("Config file: {:#?}", config_file);

    // Initialize logging with config values.
    initialize_logging(
        Some(&command_line),
        if command_line.prove {
            Some(&config_file.prover_log)
        } else {
            Some(&config_file.log)
        },
    )?;

    // Initialize panic hook.
    initialize_panic_reporting();

    // Initialize signal handler.
    initialize_signal_handler();

    // Early return in case of a proving process.
    if command_line.prove {
        info!("Starting proof generation. Waiting for input.");
        return Ok(prover_main().await?);
    }

    // Create config builder and apply command line and config file.
    let mut builder = ClientConfig::builder();
    builder.config_file(&config_file)?;
    builder.command_line(&command_line)?;

    // Finalize config.
    let config = builder.build()?;
    log::debug!("Final configuration: {:#?}", config);

    // Initialize the client.
    let client = Client::from_config(config.clone()).await?;

    // Clone config for RPC and metrics server.
    let rpc_config = config.rpc_server.clone();
    let metrics_config = config.metrics_server.clone();

    // Initialize mempool and consensus proxy.
    let mempool = client.mempool().clone();
    let consensus = client.consensus_proxy().clone();

    // Initialize task monitor correctly.
    let task_monitor = NimiqTaskMonitor{
        name: "TaskMonitor1".to_string(),
        monitor: TaskMonitor::new()
    };

    // Start metrics server.
    if let Some(metrics_config) = metrics_config {
        nimiq::extras::metrics_server::start_metrics_server(
            metrics_config.addr,
            client.blockchain(),
            mempool.clone(),
            consensus.clone(),
            client.network(),
            &[task_monitor], // Pass as a slice.
        );
    }

    // Create the "monitor" future which never completes to keep the client alive.
    let mut statistics_interval = test_interval.unwrap_or(config_file.log.statistics);
    let mut show_statistics = true;
    if statistics_interval < 1 {
        statistics_interval = 10; // Set a default minimum interval.
        show_statistics = false;
    }

    // Run periodically.
    let mut interval = tokio::time::interval(Duration::from_secs(statistics_interval));
    loop {
        interval.tick().await;

        if show_statistics {
            match client.network().network_info().await {
                Ok(network_info) => {
                    let head = client.blockchain_head();

                    info!(
                        "Consensus: {} - Head: {} - Peers: {} - BlockNumber: {}",
                        if consensus.is_established() {
                            "established"
                        } else {
                            "lost"
                        },
                        head,
                        network_info.num_peers(),
                        block_number = head.block_number() // ,
                        // num_peers = network_info.num_peers()
                    );
                }
                Err(err) => {
                    log::error!("Error retrieving NetworkInfo: {:?}", err);
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    // Set test_interval to zero to simulate the DoS condition.
    let test_interval = Some(0);

    if let Err(e) = main_inner(test_interval).await {
        log_error_cause_chain(&e);
        std::process::exit(1);
    }
    std::process::exit(0);
}

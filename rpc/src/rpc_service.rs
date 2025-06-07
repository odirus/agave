//! The `rpc_service` module implements the Solana JSON RPC service.
//!
//! This module provides three ways to start the RPC service:
//!
//! 1. **HTTP only**: Use `JsonRpcService::new()` with a TCP socket address
//! 2. **Unix Socket only**: Use `JsonRpcService::new_unix_socket()` with a Unix socket path (Unix/macOS only)
//! 3. **Both HTTP and Unix Socket**: Use `JsonRpcService::new_with_both()` to run both simultaneously (Unix/macOS only)
//!
//! ## Examples
//!
//! ### HTTP only
//! ```rust,no_run
//! # use std::sync::Arc;
//! # use solana_rpc::rpc_service::JsonRpcService;
//! # use solana_rpc::rpc::JsonRpcConfig;
//! # use std::net::SocketAddr;
//! let rpc_addr: SocketAddr = "127.0.0.1:8899".parse().unwrap();
//! let service = JsonRpcService::new(
//!     rpc_addr,
//!     JsonRpcConfig::default(),
//!     // ... other parameters
//! #   None, Arc::new(std::sync::RwLock::new(solana_runtime::bank_forks::BankForks::new_empty())),
//! #   Arc::new(std::sync::RwLock::new(solana_runtime::commitment::BlockCommitmentCache::default())),
//! #   Arc::new(solana_ledger::blockstore::Blockstore::open(&std::path::PathBuf::from("test")).unwrap()),
//! #   Arc::new(solana_gossip::cluster_info::ClusterInfo::new_with_invalid_keypair(
//! #       solana_gossip::contact_info::ContactInfo::default()
//! #   )),
//! #   None, solana_sdk::hash::Hash::default(), &std::path::PathBuf::from("test"),
//! #   Arc::new(std::sync::RwLock::new(solana_sdk::exit::Exit::default())),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(false)),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(false)),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(true)),
//! #   Arc::new(std::sync::RwLock::new(solana_rpc::optimistically_confirmed_bank_tracker::OptimisticallyConfirmedBank::locked_from_bank_forks_root(&Arc::new(std::sync::RwLock::new(solana_runtime::bank_forks::BankForks::new_empty()))))),
//! #   solana_send_transaction_service::send_transaction_service::Config::default(),
//! #   Arc::new(solana_rpc::max_slots::MaxSlots::default()),
//! #   Arc::new(solana_ledger::leader_schedule_cache::LeaderScheduleCache::default()),
//! #   Arc::new(solana_client::connection_cache::ConnectionCache::new("test")),
//! #   Arc::new(std::sync::atomic::AtomicU64::new(0)),
//! #   Arc::new(std::sync::atomic::AtomicU64::new(0)),
//! #   Arc::new(solana_runtime::prioritization_fee_cache::PrioritizationFeeCache::default()),
//! );
//! ```
//!
//! ### Unix Socket only (Unix/macOS)
//! ```rust,no_run
//! # #[cfg(unix)]
//! # {
//! # use std::sync::Arc;
//! # use solana_rpc::rpc_service::JsonRpcService;
//! # use solana_rpc::rpc::JsonRpcConfig;
//! # use std::path::PathBuf;
//! let socket_path = PathBuf::from("/tmp/solana-rpc.sock");
//! let service = JsonRpcService::new_unix_socket(
//!     socket_path,
//!     JsonRpcConfig::default(),
//!     // ... other parameters
//! #   None, Arc::new(std::sync::RwLock::new(solana_runtime::bank_forks::BankForks::new_empty())),
//! #   Arc::new(std::sync::RwLock::new(solana_runtime::commitment::BlockCommitmentCache::default())),
//! #   Arc::new(solana_ledger::blockstore::Blockstore::open(&std::path::PathBuf::from("test")).unwrap()),
//! #   Arc::new(solana_gossip::cluster_info::ClusterInfo::new_with_invalid_keypair(
//! #       solana_gossip::contact_info::ContactInfo::default()
//! #   )),
//! #   None, solana_sdk::hash::Hash::default(), &std::path::PathBuf::from("test"),
//! #   Arc::new(std::sync::RwLock::new(solana_sdk::exit::Exit::default())),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(false)),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(false)),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(true)),
//! #   Arc::new(std::sync::RwLock::new(solana_rpc::optimistically_confirmed_bank_tracker::OptimisticallyConfirmedBank::locked_from_bank_forks_root(&Arc::new(std::sync::RwLock::new(solana_runtime::bank_forks::BankForks::new_empty()))))),
//! #   solana_send_transaction_service::send_transaction_service::Config::default(),
//! #   Arc::new(solana_rpc::max_slots::MaxSlots::default()),
//! #   Arc::new(solana_ledger::leader_schedule_cache::LeaderScheduleCache::default()),
//! #   Arc::new(solana_client::connection_cache::ConnectionCache::new("test")),
//! #   Arc::new(std::sync::atomic::AtomicU64::new(0)),
//! #   Arc::new(std::sync::atomic::AtomicU64::new(0)),
//! #   Arc::new(solana_runtime::prioritization_fee_cache::PrioritizationFeeCache::default()),
//! );
//! # }
//! ```
//!
//! ### Both HTTP and Unix Socket (Unix/macOS)
//! ```rust,no_run
//! # #[cfg(unix)]
//! # {
//! # use std::sync::Arc;
//! # use solana_rpc::rpc_service::JsonRpcService;
//! # use solana_rpc::rpc::JsonRpcConfig;
//! # use std::net::SocketAddr;
//! # use std::path::PathBuf;
//! let rpc_addr: SocketAddr = "127.0.0.1:8899".parse().unwrap();
//! let socket_path = PathBuf::from("/tmp/solana-rpc.sock");
//! let service = JsonRpcService::new_with_both(
//!     rpc_addr,
//!     socket_path,
//!     JsonRpcConfig::default(),
//!     // ... other parameters
//! #   None, Arc::new(std::sync::RwLock::new(solana_runtime::bank_forks::BankForks::new_empty())),
//! #   Arc::new(std::sync::RwLock::new(solana_runtime::commitment::BlockCommitmentCache::default())),
//! #   Arc::new(solana_ledger::blockstore::Blockstore::open(&std::path::PathBuf::from("test")).unwrap()),
//! #   Arc::new(solana_gossip::cluster_info::ClusterInfo::new_with_invalid_keypair(
//! #       solana_gossip::contact_info::ContactInfo::default()
//! #   )),
//! #   None, solana_sdk::hash::Hash::default(), &std::path::PathBuf::from("test"),
//! #   Arc::new(std::sync::RwLock::new(solana_sdk::exit::Exit::default())),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(false)),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(false)),
//! #   Arc::new(std::sync::atomic::AtomicBool::new(true)),
//! #   Arc::new(std::sync::RwLock::new(solana_rpc::optimistically_confirmed_bank_tracker::OptimisticallyConfirmedBank::locked_from_bank_forks_root(&Arc::new(std::sync::RwLock::new(solana_runtime::bank_forks::BankForks::new_empty()))))),
//! #   solana_send_transaction_service::send_transaction_service::Config::default(),
//! #   Arc::new(solana_rpc::max_slots::MaxSlots::default()),
//! #   Arc::new(solana_ledger::leader_schedule_cache::LeaderScheduleCache::default()),
//! #   Arc::new(solana_client::connection_cache::ConnectionCache::new("test")),
//! #   Arc::new(std::sync::atomic::AtomicU64::new(0)),
//! #   Arc::new(std::sync::atomic::AtomicU64::new(0)),
//! #   Arc::new(solana_runtime::prioritization_fee_cache::PrioritizationFeeCache::default()),
//! );
//! # }
//! ```

use {
    crate::{
        cluster_tpu_info::ClusterTpuInfo,
        max_slots::MaxSlots,
        optimistically_confirmed_bank_tracker::OptimisticallyConfirmedBank,
        rpc::{rpc_accounts::*, rpc_accounts_scan::*, rpc_bank::*, rpc_full::*, rpc_minimal::*, *},
        rpc_cache::LargestAccountsCache,
        rpc_health::*,
    },
    crossbeam_channel::unbounded,
    jsonrpc_core::{futures::prelude::*, MetaIoHandler},
    jsonrpc_http_server::{
        hyper, AccessControlAllowOrigin, CloseHandle, DomainsValidation, RequestMiddleware,
        RequestMiddlewareAction, ServerBuilder,
    },
    regex::Regex,
    solana_client::connection_cache::ConnectionCache,
    solana_gossip::cluster_info::ClusterInfo,
    solana_ledger::{
        bigtable_upload::ConfirmedBlockUploadConfig,
        bigtable_upload_service::BigTableUploadService, blockstore::Blockstore,
        leader_schedule_cache::LeaderScheduleCache,
    },
    solana_metrics::inc_new_counter_info,
    solana_perf::thread::renice_this_thread,
    solana_poh::poh_recorder::PohRecorder,
    solana_runtime::{
        bank::Bank, bank_forks::BankForks, commitment::BlockCommitmentCache,
        non_circulating_supply::calculate_non_circulating_supply,
        prioritization_fee_cache::PrioritizationFeeCache,
        snapshot_archive_info::SnapshotArchiveInfoGetter,
        snapshot_bank_utils::DISABLED_SNAPSHOT_ARCHIVE_INTERVAL, snapshot_config::SnapshotConfig,
        snapshot_utils,
    },
    solana_sdk::{
        exit::Exit, genesis_config::DEFAULT_GENESIS_DOWNLOAD_PATH, hash::Hash,
        native_token::lamports_to_sol,
    },
    solana_send_transaction_service::{
        send_transaction_service::{self, SendTransactionService},
        transaction_client::ConnectionCacheClient,
    },
    solana_storage_bigtable::CredentialType,
    std::{
        net::SocketAddr,
        path::{Path, PathBuf},
        pin::Pin,
        sync::{
            atomic::{AtomicBool, AtomicU64, Ordering},
            Arc, RwLock,
        },
        task::{Context, Poll},
        thread::{self, Builder, JoinHandle},
        time::{Duration, Instant},
    },
            tokio_util::{
        bytes::Bytes,
        codec::{BytesCodec, FramedRead},
    },
};

#[cfg(unix)]
use {
    std::os::unix::fs::PermissionsExt,
};

const FULL_SNAPSHOT_REQUEST_PATH: &str = "/snapshot.tar.bz2";
const INCREMENTAL_SNAPSHOT_REQUEST_PATH: &str = "/incremental-snapshot.tar.bz2";
const LARGEST_ACCOUNTS_CACHE_DURATION: u64 = 60 * 60 * 2;
/// Default minimum snapshot download speed is 10 MB/s
/// Full snapshots are ~90 GB, incremental are ~1 GB today but both will increase over time
/// Full: 120 GB / 10 MB/s = 12,000 seconds -> ~30k slots
const FALLBACK_FULL_SNAPSHOT_TIMEOUT_SECS: Duration = Duration::from_secs(12_000);
/// Incremental: 2.5 GB / 10 MB/s = 250 seconds -> ~625 slots
const FALLBACK_INCREMENTAL_SNAPSHOT_TIMEOUT_SECS: Duration = Duration::from_secs(250);

enum SnapshotKind {
    Full,
    Incremental,
}

struct TimeoutStream<S> {
    inner: S,
    deadline: Instant,
}

impl<S> TimeoutStream<S> {
    fn new(inner: S, timeout: Duration) -> Self {
        Self {
            inner,
            deadline: Instant::now() + timeout,
        }
    }
}

impl<S> Stream for TimeoutStream<S>
where
    S: Stream<Item = std::io::Result<Bytes>> + Unpin,
{
    type Item = std::io::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if Instant::now() >= self.deadline {
            return Poll::Ready(Some(Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "snapshot transfer deadline exceeded",
            ))));
        }
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

pub struct JsonRpcService {
    thread_hdl: JoinHandle<()>,

    #[cfg(test)]
    pub request_processor: JsonRpcRequestProcessor, // Used only by test_rpc_new()...

    close_handle: Option<CloseHandle>,
    exit: Arc<AtomicBool>,
    #[cfg(unix)]
    unix_cleanup: Option<Box<dyn FnOnce() + Send>>,
}

struct RpcRequestMiddleware {
    ledger_path: PathBuf,
    full_snapshot_archive_path_regex: Regex,
    incremental_snapshot_archive_path_regex: Regex,
    snapshot_config: Option<SnapshotConfig>,
    bank_forks: Arc<RwLock<BankForks>>,
    health: Arc<RpcHealth>,
}

impl Clone for RpcRequestMiddleware {
    fn clone(&self) -> Self {
        Self {
            ledger_path: self.ledger_path.clone(),
            full_snapshot_archive_path_regex: Regex::new(
                snapshot_utils::FULL_SNAPSHOT_ARCHIVE_FILENAME_REGEX,
            ).unwrap(),
            incremental_snapshot_archive_path_regex: Regex::new(
                snapshot_utils::INCREMENTAL_SNAPSHOT_ARCHIVE_FILENAME_REGEX,
            ).unwrap(),
            snapshot_config: self.snapshot_config.clone(),
            bank_forks: self.bank_forks.clone(),
            health: self.health.clone(),
        }
    }
}

impl RpcRequestMiddleware {
    pub fn new(
        ledger_path: PathBuf,
        snapshot_config: Option<SnapshotConfig>,
        bank_forks: Arc<RwLock<BankForks>>,
        health: Arc<RpcHealth>,
    ) -> Self {
        Self {
            ledger_path,
            full_snapshot_archive_path_regex: Regex::new(
                snapshot_utils::FULL_SNAPSHOT_ARCHIVE_FILENAME_REGEX,
            )
            .unwrap(),
            incremental_snapshot_archive_path_regex: Regex::new(
                snapshot_utils::INCREMENTAL_SNAPSHOT_ARCHIVE_FILENAME_REGEX,
            )
            .unwrap(),
            snapshot_config,
            bank_forks,
            health,
        }
    }

    fn redirect(location: &str) -> hyper::Response<hyper::Body> {
        hyper::Response::builder()
            .status(hyper::StatusCode::SEE_OTHER)
            .header(hyper::header::LOCATION, location)
            .body(hyper::Body::from(String::from(location)))
            .unwrap()
    }

    fn not_found() -> hyper::Response<hyper::Body> {
        hyper::Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())
            .unwrap()
    }

    fn internal_server_error() -> hyper::Response<hyper::Body> {
        hyper::Response::builder()
            .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
            .body(hyper::Body::empty())
            .unwrap()
    }

    fn strip_leading_slash(path: &str) -> Option<&str> {
        path.strip_prefix('/')
    }

    fn is_file_get_path(&self, path: &str) -> bool {
        if path == DEFAULT_GENESIS_DOWNLOAD_PATH {
            return true;
        }

        if self.snapshot_config.is_none() {
            return false;
        }

        let Some(path) = Self::strip_leading_slash(path) else {
            return false;
        };

        self.full_snapshot_archive_path_regex.is_match(path)
            || self.incremental_snapshot_archive_path_regex.is_match(path)
    }

    #[cfg(unix)]
    async fn open_no_follow(path: impl AsRef<Path>) -> std::io::Result<tokio::fs::File> {
        tokio::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)
            .await
    }

    #[cfg(not(unix))]
    async fn open_no_follow(path: impl AsRef<Path>) -> std::io::Result<tokio::fs::File> {
        // TODO: Is there any way to achieve the same on Windows?
        tokio::fs::File::open(path).await
    }

    fn find_snapshot_file<P>(&self, stem: P) -> (PathBuf, SnapshotKind)
    where
        P: AsRef<Path>,
    {
        let is_full = self
            .full_snapshot_archive_path_regex
            .is_match(Path::new("").join(&stem).to_str().unwrap());
        let root = if is_full {
            &self
                .snapshot_config
                .as_ref()
                .unwrap()
                .full_snapshot_archives_dir
        } else {
            &self
                .snapshot_config
                .as_ref()
                .unwrap()
                .incremental_snapshot_archives_dir
        };
        let local_path = root.join(&stem);
        let path = if local_path.exists() {
            local_path
        } else {
            // remote snapshot archive path
            snapshot_utils::build_snapshot_archives_remote_dir(root).join(stem)
        };
        (
            path,
            if is_full {
                SnapshotKind::Full
            } else {
                SnapshotKind::Incremental
            },
        )
    }

    fn process_file_get(&self, path: &str) -> RequestMiddlewareAction {
        let (filename, snapshot_type) = {
            let stem = Self::strip_leading_slash(path).expect("path already verified");
            match path {
                DEFAULT_GENESIS_DOWNLOAD_PATH => {
                    inc_new_counter_info!("rpc-get_genesis", 1);
                    (self.ledger_path.join(stem), None)
                }
                _ => {
                    inc_new_counter_info!("rpc-get_snapshot", 1);
                    let (path, snapshot_type) = self.find_snapshot_file(stem);
                    (path, Some(snapshot_type))
                }
            }
        };
        let file_length = std::fs::metadata(&filename)
            .map(|m| m.len())
            .unwrap_or(0)
            .to_string();
        info!("get {} -> {:?} ({} bytes)", path, filename, file_length);

        if cfg!(not(test)) {
            assert!(
                self.snapshot_config.is_some(),
                "snapshot_config should never be None outside of tests"
            );
        }
        let snapshot_timeout = self.snapshot_config.as_ref().and_then(|config| {
            snapshot_type.map(|st| {
                let slots = match st {
                    SnapshotKind::Full => config.full_snapshot_archive_interval_slots,
                    SnapshotKind::Incremental => config.incremental_snapshot_archive_interval_slots,
                };
                let computed = if slots == DISABLED_SNAPSHOT_ARCHIVE_INTERVAL {
                    Duration::ZERO
                } else {
                    Duration::from_millis(
                        slots.saturating_mul(solana_sdk::clock::DEFAULT_MS_PER_SLOT),
                    )
                };
                let fallback = match st {
                    SnapshotKind::Full => FALLBACK_FULL_SNAPSHOT_TIMEOUT_SECS,
                    SnapshotKind::Incremental => FALLBACK_INCREMENTAL_SNAPSHOT_TIMEOUT_SECS,
                };
                std::cmp::max(computed, fallback)
            })
        });

        RequestMiddlewareAction::Respond {
            should_validate_hosts: true,
            response: Box::pin(async move {
                match Self::open_no_follow(filename).await {
                    Err(err) => Ok(if err.kind() == std::io::ErrorKind::NotFound {
                        Self::not_found()
                    } else {
                        Self::internal_server_error()
                    }),
                    Ok(file) => {
                        let stream =
                            FramedRead::new(file, BytesCodec::new()).map_ok(|b| b.freeze());
                        let body = if let Some(timeout) = snapshot_timeout {
                            hyper::Body::wrap_stream(TimeoutStream::new(stream, timeout))
                        } else {
                            hyper::Body::wrap_stream(stream)
                        };
                        Ok(hyper::Response::builder()
                            .header(hyper::header::CONTENT_LENGTH, file_length)
                            .body(body)
                            .unwrap())
                    }
                }
            }),
        }
    }

    fn health_check(&self) -> &'static str {
        let response = match self.health.check() {
            RpcHealthStatus::Ok => "ok",
            RpcHealthStatus::Behind { .. } => "behind",
            RpcHealthStatus::Unknown => "unknown",
        };
        info!("health check: {}", response);
        response
    }
}

impl RequestMiddleware for RpcRequestMiddleware {
    fn on_request(&self, request: hyper::Request<hyper::Body>) -> RequestMiddlewareAction {
        trace!("request uri: {}", request.uri());

        if let Some(ref snapshot_config) = self.snapshot_config {
            if request.uri().path() == FULL_SNAPSHOT_REQUEST_PATH
                || request.uri().path() == INCREMENTAL_SNAPSHOT_REQUEST_PATH
            {
                // Convenience redirect to the latest snapshot
                let full_snapshot_archive_info =
                    snapshot_utils::get_highest_full_snapshot_archive_info(
                        &snapshot_config.full_snapshot_archives_dir,
                    );
                let snapshot_archive_info =
                    if let Some(full_snapshot_archive_info) = full_snapshot_archive_info {
                        if request.uri().path() == FULL_SNAPSHOT_REQUEST_PATH {
                            Some(full_snapshot_archive_info.snapshot_archive_info().clone())
                        } else {
                            snapshot_utils::get_highest_incremental_snapshot_archive_info(
                                &snapshot_config.incremental_snapshot_archives_dir,
                                full_snapshot_archive_info.slot(),
                            )
                            .map(|incremental_snapshot_archive_info| {
                                incremental_snapshot_archive_info
                                    .snapshot_archive_info()
                                    .clone()
                            })
                        }
                    } else {
                        None
                    };
                return if let Some(snapshot_archive_info) = snapshot_archive_info {
                    RpcRequestMiddleware::redirect(&format!(
                        "/{}",
                        snapshot_archive_info
                            .path
                            .file_name()
                            .unwrap_or_else(|| std::ffi::OsStr::new(""))
                            .to_str()
                            .unwrap_or("")
                    ))
                } else {
                    RpcRequestMiddleware::not_found()
                }
                .into();
            }
        }

        if let Some(path) = match_supply_path(request.uri().path()) {
            process_rest(&self.bank_forks, path)
        } else if self.is_file_get_path(request.uri().path()) {
            self.process_file_get(request.uri().path())
        } else if request.uri().path() == "/health" {
            hyper::Response::builder()
                .status(hyper::StatusCode::OK)
                .body(hyper::Body::from(self.health_check()))
                .unwrap()
                .into()
        } else {
            request.into()
        }
    }
}

fn match_supply_path(path: &str) -> Option<&str> {
    match path {
        "/v0/circulating-supply" | "/v0/total-supply" => Some(path),
        _ => None,
    }
}

#[derive(Debug)]
pub enum SupplyCalcError {
    Scan(String),
}

async fn calculate_circulating_supply_async(bank: &Arc<Bank>) -> Result<u64, SupplyCalcError> {
    let total_supply = bank.capitalization();
    let bank = Arc::clone(bank);
    let non_circulating_supply =
        tokio::task::spawn_blocking(move || calculate_non_circulating_supply(&bank))
            .await
            .expect("Failed to spawn blocking task")
            .map_err(|e| SupplyCalcError::Scan(e.to_string()))?;

    Ok(total_supply.saturating_sub(non_circulating_supply.lamports))
}

async fn handle_rest(bank_forks: &Arc<RwLock<BankForks>>, path: &str) -> Option<String> {
    match path {
        "/v0/circulating-supply" => {
            let bank = bank_forks.read().unwrap().root_bank();
            let supply_result = calculate_circulating_supply_async(&bank).await;
            match supply_result {
                Ok(supply) => Some(format!("{}", lamports_to_sol(supply))),
                Err(_) => None,
            }
        }
        "/v0/total-supply" => {
            let bank = bank_forks.read().unwrap().root_bank();
            let total_supply = bank.capitalization();
            Some(format!("{}", lamports_to_sol(total_supply)))
        }
        _ => None,
    }
}

fn process_rest(bank_forks: &Arc<RwLock<BankForks>>, path: &str) -> RequestMiddlewareAction {
    let bank_forks = bank_forks.clone();
    let path = path.to_string();

    RequestMiddlewareAction::Respond {
        should_validate_hosts: true,
        response: Box::pin(async move {
            let result = handle_rest(&bank_forks, path.as_str()).await;
            match result {
                Some(s) => Ok(hyper::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .body(hyper::Body::from(s))
                    .unwrap()),
                None => Ok(RpcRequestMiddleware::not_found()),
            }
        }),
    }
}

impl JsonRpcService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rpc_addr: SocketAddr,
        config: JsonRpcConfig,
        snapshot_config: Option<SnapshotConfig>,
        bank_forks: Arc<RwLock<BankForks>>,
        block_commitment_cache: Arc<RwLock<BlockCommitmentCache>>,
        blockstore: Arc<Blockstore>,
        cluster_info: Arc<ClusterInfo>,
        poh_recorder: Option<Arc<RwLock<PohRecorder>>>,
        genesis_hash: Hash,
        ledger_path: &Path,
        validator_exit: Arc<RwLock<Exit>>,
        exit: Arc<AtomicBool>,
        override_health_check: Arc<AtomicBool>,
        startup_verification_complete: Arc<AtomicBool>,
        optimistically_confirmed_bank: Arc<RwLock<OptimisticallyConfirmedBank>>,
        send_transaction_service_config: send_transaction_service::Config,
        max_slots: Arc<MaxSlots>,
        leader_schedule_cache: Arc<LeaderScheduleCache>,
        connection_cache: Arc<ConnectionCache>,
        max_complete_transaction_status_slot: Arc<AtomicU64>,
        max_complete_rewards_slot: Arc<AtomicU64>,
        prioritization_fee_cache: Arc<PrioritizationFeeCache>,
    ) -> Result<Self, String> {
        Self::new_impl(
            Some(rpc_addr),
            None,
            config,
            snapshot_config,
            bank_forks,
            block_commitment_cache,
            blockstore,
            cluster_info,
            poh_recorder,
            genesis_hash,
            ledger_path,
            validator_exit,
            exit,
            override_health_check,
            startup_verification_complete,
            optimistically_confirmed_bank,
            send_transaction_service_config,
            max_slots,
            leader_schedule_cache,
            connection_cache,
            max_complete_transaction_status_slot,
            max_complete_rewards_slot,
            prioritization_fee_cache,
        )
    }

    #[cfg(unix)]
    #[allow(clippy::too_many_arguments)]
    pub fn new_unix_socket(
        unix_socket_path: PathBuf,
        config: JsonRpcConfig,
        snapshot_config: Option<SnapshotConfig>,
        bank_forks: Arc<RwLock<BankForks>>,
        block_commitment_cache: Arc<RwLock<BlockCommitmentCache>>,
        blockstore: Arc<Blockstore>,
        cluster_info: Arc<ClusterInfo>,
        poh_recorder: Option<Arc<RwLock<PohRecorder>>>,
        genesis_hash: Hash,
        ledger_path: &Path,
        validator_exit: Arc<RwLock<Exit>>,
        exit: Arc<AtomicBool>,
        override_health_check: Arc<AtomicBool>,
        startup_verification_complete: Arc<AtomicBool>,
        optimistically_confirmed_bank: Arc<RwLock<OptimisticallyConfirmedBank>>,
        send_transaction_service_config: send_transaction_service::Config,
        max_slots: Arc<MaxSlots>,
        leader_schedule_cache: Arc<LeaderScheduleCache>,
        connection_cache: Arc<ConnectionCache>,
        max_complete_transaction_status_slot: Arc<AtomicU64>,
        max_complete_rewards_slot: Arc<AtomicU64>,
        prioritization_fee_cache: Arc<PrioritizationFeeCache>,
    ) -> Result<Self, String> {
        Self::new_impl(
            None,
            Some(unix_socket_path),
            config,
            snapshot_config,
            bank_forks,
            block_commitment_cache,
            blockstore,
            cluster_info,
            poh_recorder,
            genesis_hash,
            ledger_path,
            validator_exit,
            exit,
            override_health_check,
            startup_verification_complete,
            optimistically_confirmed_bank,
            send_transaction_service_config,
            max_slots,
            leader_schedule_cache,
            connection_cache,
            max_complete_transaction_status_slot,
            max_complete_rewards_slot,
            prioritization_fee_cache,
        )
    }

    #[cfg(unix)]
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_both(
        rpc_addr: SocketAddr,
        unix_socket_path: PathBuf,
        config: JsonRpcConfig,
        snapshot_config: Option<SnapshotConfig>,
        bank_forks: Arc<RwLock<BankForks>>,
        block_commitment_cache: Arc<RwLock<BlockCommitmentCache>>,
        blockstore: Arc<Blockstore>,
        cluster_info: Arc<ClusterInfo>,
        poh_recorder: Option<Arc<RwLock<PohRecorder>>>,
        genesis_hash: Hash,
        ledger_path: &Path,
        validator_exit: Arc<RwLock<Exit>>,
        exit: Arc<AtomicBool>,
        override_health_check: Arc<AtomicBool>,
        startup_verification_complete: Arc<AtomicBool>,
        optimistically_confirmed_bank: Arc<RwLock<OptimisticallyConfirmedBank>>,
        send_transaction_service_config: send_transaction_service::Config,
        max_slots: Arc<MaxSlots>,
        leader_schedule_cache: Arc<LeaderScheduleCache>,
        connection_cache: Arc<ConnectionCache>,
        max_complete_transaction_status_slot: Arc<AtomicU64>,
        max_complete_rewards_slot: Arc<AtomicU64>,
        prioritization_fee_cache: Arc<PrioritizationFeeCache>,
    ) -> Result<Self, String> {
        Self::new_impl(
            Some(rpc_addr),
            Some(unix_socket_path),
            config,
            snapshot_config,
            bank_forks,
            block_commitment_cache,
            blockstore,
            cluster_info,
            poh_recorder,
            genesis_hash,
            ledger_path,
            validator_exit,
            exit,
            override_health_check,
            startup_verification_complete,
            optimistically_confirmed_bank,
            send_transaction_service_config,
            max_slots,
            leader_schedule_cache,
            connection_cache,
            max_complete_transaction_status_slot,
            max_complete_rewards_slot,
            prioritization_fee_cache,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new_impl(
        rpc_addr: Option<SocketAddr>,
        #[cfg(unix)] unix_socket_path: Option<PathBuf>,
        #[cfg(not(unix))] _unix_socket_path: Option<PathBuf>,
        config: JsonRpcConfig,
        snapshot_config: Option<SnapshotConfig>,
        bank_forks: Arc<RwLock<BankForks>>,
        block_commitment_cache: Arc<RwLock<BlockCommitmentCache>>,
        blockstore: Arc<Blockstore>,
        cluster_info: Arc<ClusterInfo>,
        poh_recorder: Option<Arc<RwLock<PohRecorder>>>,
        genesis_hash: Hash,
        ledger_path: &Path,
        validator_exit: Arc<RwLock<Exit>>,
        exit: Arc<AtomicBool>,
        override_health_check: Arc<AtomicBool>,
        startup_verification_complete: Arc<AtomicBool>,
        optimistically_confirmed_bank: Arc<RwLock<OptimisticallyConfirmedBank>>,
        send_transaction_service_config: send_transaction_service::Config,
        max_slots: Arc<MaxSlots>,
        leader_schedule_cache: Arc<LeaderScheduleCache>,
        connection_cache: Arc<ConnectionCache>,
        max_complete_transaction_status_slot: Arc<AtomicU64>,
        max_complete_rewards_slot: Arc<AtomicU64>,
        prioritization_fee_cache: Arc<PrioritizationFeeCache>,
    ) -> Result<Self, String> {
        // Validate configuration - we expect both HTTP and Unix socket to be provided
        #[cfg(unix)]
        if rpc_addr.is_none() || unix_socket_path.is_none() {
            return Err("Both HTTP address and Unix socket path must be provided for optimal RPC service configuration".to_string());
        }
        
        #[cfg(not(unix))]
        if rpc_addr.is_none() {
            return Err("HTTP address must be provided for RPC service".to_string());
        }
        
        #[cfg(unix)]
        if let Some(ref socket_path) = unix_socket_path {
            info!("rpc bound to unix socket: {:?}", socket_path);
        }
        if let Some(rpc_addr) = rpc_addr {
            info!("rpc bound to {:?}", rpc_addr);
        }
        
        info!("rpc configuration: {:?}", config);
        let rpc_threads = 1.max(config.rpc_threads);
        let rpc_blocking_threads = 1.max(config.rpc_blocking_threads);
        let rpc_niceness_adj = config.rpc_niceness_adj;

        let health = Arc::new(RpcHealth::new(
            Arc::clone(&optimistically_confirmed_bank),
            Arc::clone(&blockstore),
            config.health_check_slot_distance,
            override_health_check,
            startup_verification_complete,
        ));

        let largest_accounts_cache = Arc::new(RwLock::new(LargestAccountsCache::new(
            LARGEST_ACCOUNTS_CACHE_DURATION,
        )));

        let tpu_address = cluster_info
            .my_contact_info()
            .tpu(connection_cache.protocol())
            .ok_or_else(|| {
                format!(
                    "Invalid {:?} socket address for TPU",
                    connection_cache.protocol()
                )
            })?;

        let runtime = service_runtime(rpc_threads, rpc_blocking_threads, rpc_niceness_adj);

        let exit_bigtable_ledger_upload_service = Arc::new(AtomicBool::new(false));

        let (bigtable_ledger_storage, _bigtable_ledger_upload_service) =
            if let Some(RpcBigtableConfig {
                enable_bigtable_ledger_upload,
                ref bigtable_instance_name,
                ref bigtable_app_profile_id,
                timeout,
                max_message_size,
            }) = config.rpc_bigtable_config
            {
                let bigtable_config = solana_storage_bigtable::LedgerStorageConfig {
                    read_only: !enable_bigtable_ledger_upload,
                    timeout,
                    credential_type: CredentialType::Filepath(None),
                    instance_name: bigtable_instance_name.clone(),
                    app_profile_id: bigtable_app_profile_id.clone(),
                    max_message_size,
                };
                runtime
                    .block_on(solana_storage_bigtable::LedgerStorage::new_with_config(
                        bigtable_config,
                    ))
                    .map(|bigtable_ledger_storage| {
                        info!("BigTable ledger storage initialized");

                        let bigtable_ledger_upload_service = if enable_bigtable_ledger_upload {
                            Some(Arc::new(BigTableUploadService::new_with_config(
                                runtime.clone(),
                                bigtable_ledger_storage.clone(),
                                blockstore.clone(),
                                block_commitment_cache.clone(),
                                max_complete_transaction_status_slot.clone(),
                                max_complete_rewards_slot.clone(),
                                ConfirmedBlockUploadConfig::default(),
                                exit_bigtable_ledger_upload_service.clone(),
                            )))
                        } else {
                            None
                        };

                        (
                            Some(bigtable_ledger_storage),
                            bigtable_ledger_upload_service,
                        )
                    })
                    .unwrap_or_else(|err| {
                        error!("Failed to initialize BigTable ledger storage: {:?}", err);
                        (None, None)
                    })
            } else {
                (None, None)
            };

        let full_api = config.full_api;
        let max_request_body_size = config
            .max_request_body_size
            .unwrap_or(MAX_REQUEST_BODY_SIZE);
        let (request_processor, receiver) = JsonRpcRequestProcessor::new(
            config,
            snapshot_config.clone(),
            bank_forks.clone(),
            block_commitment_cache,
            blockstore,
            validator_exit.clone(),
            health.clone(),
            cluster_info.clone(),
            genesis_hash,
            bigtable_ledger_storage,
            optimistically_confirmed_bank,
            largest_accounts_cache,
            max_slots,
            leader_schedule_cache,
            max_complete_transaction_status_slot,
            max_complete_rewards_slot,
            prioritization_fee_cache,
            Arc::clone(&runtime),
        );

        let leader_info =
            poh_recorder.map(|recorder| ClusterTpuInfo::new(cluster_info.clone(), recorder));
        let client = ConnectionCacheClient::new(
            connection_cache,
            tpu_address,
            send_transaction_service_config.tpu_peers.clone(),
            leader_info,
            send_transaction_service_config.leader_forward_count,
        );
        let _send_transaction_service = Arc::new(SendTransactionService::new_with_client(
            &bank_forks,
            receiver,
            client,
            send_transaction_service_config,
            exit.clone(),
        ));

        #[cfg(test)]
        let test_request_processor = request_processor.clone();

        let ledger_path = ledger_path.to_path_buf();

        #[cfg(unix)]
        let socket_path_for_cleanup = unix_socket_path.clone();
        
        // Clone exit for use in the service thread
        let exit_for_service = exit.clone();
        
        let (close_handle_sender, close_handle_receiver) = unbounded();
        let thread_hdl = Builder::new()
            .name("solJsonRpcSvc".to_string())
            .spawn(move || {
                renice_this_thread(rpc_niceness_adj).unwrap();

                let mut io = MetaIoHandler::default();

                io.extend_with(rpc_minimal::MinimalImpl.to_delegate());
                if full_api {
                    io.extend_with(rpc_bank::BankDataImpl.to_delegate());
                    io.extend_with(rpc_accounts::AccountsDataImpl.to_delegate());
                    io.extend_with(rpc_accounts_scan::AccountsScanImpl.to_delegate());
                    io.extend_with(rpc_full::FullImpl.to_delegate());
                }

                let request_middleware = RpcRequestMiddleware::new(
                    ledger_path,
                    snapshot_config,
                    bank_forks.clone(),
                    health.clone(),
                );

                                // Start Native Unix socket server if configured
                #[cfg(unix)]
                if let Some(socket_path) = unix_socket_path.clone() {
                    let unix_io = io.clone();
                    let unix_request_processor = request_processor.clone();
                    let unix_exit = exit_for_service.clone();
                    let unix_runtime = runtime.clone();
                    
                    // Start Native Unix Socket server with dedicated thread pool
                    Builder::new()
                        .name("solUnixRpcSvc".to_string())
                        .spawn(move || {
                            renice_this_thread(rpc_niceness_adj).unwrap();
                            
                            let result = unix_runtime.block_on(async move {
                                Self::start_native_unix_socket_server(
                                    socket_path,
                                    unix_io,
                                    unix_request_processor,
                                    unix_exit,
                                    rpc_threads, // Pass thread count for connection pool
                                ).await
                            });
                            
                            if let Err(e) = result {
                                warn!("Native Unix socket RPC service error: {:?}", e);
                            }
                        })
                        .unwrap();
                }

                // Start TCP server if configured
                if let Some(addr) = rpc_addr {
                    let server = ServerBuilder::with_meta_extractor(
                        io.clone(),
                        move |req: &hyper::Request<hyper::Body>| {
                            let xbigtable = req.headers().get("x-bigtable");
                            if xbigtable.is_some_and(|v| v == "disabled") {
                                request_processor.clone_without_bigtable()
                            } else {
                                request_processor.clone()
                            }
                        },
                    )
                    .event_loop_executor(runtime.handle().clone())
                    .threads(1)
                    .cors(DomainsValidation::AllowOnly(vec![
                        AccessControlAllowOrigin::Any,
                    ]))
                    .cors_max_age(86400)
                    .request_middleware(request_middleware.clone())
                    .max_request_body_size(max_request_body_size)
                    .start_http(&addr);

                    if let Err(e) = server {
                        warn!(
                            "JSON RPC service unavailable error: {:?}. \n\
                               Also, check that port {} is not already in use by another application",
                            e,
                            addr.port()
                        );
                        close_handle_sender.send(Err(e.to_string())).unwrap();
                        return;
                    }

                    let server = server.unwrap();
                    close_handle_sender.send(Ok(server.close_handle())).unwrap();
                    server.wait();
                } else {
                    // If no HTTP address is configured, this is an invalid configuration
                    error!("Invalid RPC configuration: HTTP address is required when using RPC service");
                    close_handle_sender.send(Err("HTTP address is required for RPC service".to_string())).unwrap();
                    return;
                }
                
                exit_bigtable_ledger_upload_service.store(true, Ordering::Relaxed);
            })
            .unwrap();

        #[cfg(unix)]
        let unix_cleanup = if let Some(path) = socket_path_for_cleanup {
            Some(Box::new(move || {
                // Clean up Native Unix Socket file
                if path.exists() {
                    let _ = std::fs::remove_file(&path);
                }
            }) as Box<dyn FnOnce() + Send>)
        } else {
            None
        };

        let close_handle = close_handle_receiver.recv().unwrap()?;
        let close_handle_ = close_handle.clone();
        validator_exit
            .write()
            .unwrap()
            .register_exit(Box::new(move || {
                close_handle_.close();
            }));
        Ok(Self {
            thread_hdl,
            #[cfg(test)]
            request_processor: test_request_processor,
            close_handle: Some(close_handle),
            exit: exit.clone(),
            #[cfg(unix)]
            unix_cleanup,
        })
    }

    pub fn exit(&mut self) {
        // Set the exit flag to signal all threads to stop
        self.exit.store(true, Ordering::Relaxed);
        
        if let Some(c) = self.close_handle.take() {
            c.close()
        }
        
        #[cfg(unix)]
        if let Some(cleanup) = self.unix_cleanup.take() {
            cleanup();
        }
    }

    pub fn join(mut self) -> thread::Result<()> {
        self.exit();
        self.thread_hdl.join()
    }



    #[cfg(unix)]
    async fn start_native_unix_socket_server(
        socket_path: PathBuf,
        io: MetaIoHandler<JsonRpcRequestProcessor>,
        request_processor: JsonRpcRequestProcessor,
        exit: Arc<AtomicBool>,
        thread_count: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use tokio::net::UnixListener;
        use tokio::time::{timeout, Duration};
        
        // Remove existing socket file if it exists
        if socket_path.exists() {
            std::fs::remove_file(&socket_path)?;
        }
        
        // Create the Unix listener
        let listener = UnixListener::bind(&socket_path)?;
        
        // Set socket permissions (readable/writable by owner and group)
        std::fs::set_permissions(&socket_path, std::fs::Permissions::from_mode(0o660))?;
        
        // Create a semaphore to limit concurrent connections
        let max_connections = std::cmp::max(thread_count * 8, 64); // Ensure at least 64 concurrent connections
        let connection_semaphore = Arc::new(tokio::sync::Semaphore::new(max_connections));
        
        info!("[UDS-RPC] Native Unix socket RPC server listening on: {:?} with {} threads, max {} concurrent connections", socket_path, thread_count, max_connections);
        
        // Handle incoming connections with exit condition
        loop {
            // Check exit condition
            if exit.load(Ordering::Relaxed) {
                info!("[UDS-RPC] Native Unix socket RPC server shutting down");
                break;
            }
            
            // Accept connections with timeout to allow periodic exit checks
            match timeout(Duration::from_millis(100), listener.accept()).await {
                Ok(Ok((mut stream, _))) => {
                    let io = io.clone();
                    let request_processor = request_processor.clone();
                    let semaphore = connection_semaphore.clone();
                    
                    // Spawn task to handle connection with semaphore control
                    tokio::spawn(async move {
                        let _permit = match semaphore.acquire().await {
                            Ok(permit) => permit,
                            Err(_) => {
                                warn!("[UDS-RPC] Semaphore closed, dropping connection");
                                return;
                            }
                        };
                        use tokio::io::{AsyncWriteExt, BufReader, BufWriter, AsyncBufReadExt};
                        
                        // Use buffered I/O for better performance
                        let (reader, writer) = stream.split();
                        let mut buf_reader = BufReader::new(reader);
                        let mut buf_writer = BufWriter::new(writer);
                        let mut line = String::new();
                        
                        loop {
                            line.clear();
                            match buf_reader.read_line(&mut line).await {
                                Ok(0) => break, // EOF
                                Ok(_) => {
                                    let request = line.trim();
                                    if request.is_empty() {
                                        continue;
                                    }
                                    
                                    info!("[UDS-RPC] Processing Unix socket request: {}", request);
                                    
                                    // Process JSON-RPC request directly with better error handling
                                    match io.handle_request_sync(request, request_processor.clone()) {
                                        Some(response) => {
                                            if let Err(e) = buf_writer.write_all(response.as_bytes()).await {
                                                warn!("[UDS-RPC] Failed to write response: {}", e);
                                                break;
                                            }
                                        }
                                        None => {
                                            // For notifications, still send empty response to maintain protocol
                                            if let Err(e) = buf_writer.write_all(b"{}").await {
                                                warn!("[UDS-RPC] Failed to write notification response: {}", e);
                                                break;
                                            }
                                        }
                                    }
                                    
                                    // Always send newline and flush
                                    if let Err(e) = buf_writer.write_all(b"\n").await {
                                        warn!("[UDS-RPC] Failed to write newline: {}", e);
                                        break;
                                    }
                                    if let Err(e) = buf_writer.flush().await {
                                        warn!("[UDS-RPC] Failed to flush response: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    warn!("[UDS-RPC] Error reading from Unix socket: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                }
                Ok(Err(e)) => {
                    warn!("[UDS-RPC] Failed to accept native Unix socket connection: {}", e);
                }
                Err(_) => {
                    // Timeout occurred, continue loop to check exit condition
                    continue;
                }
            }
        }
        
        Ok(())
    }

}

pub fn service_runtime(
    rpc_threads: usize,
    rpc_blocking_threads: usize,
    rpc_niceness_adj: i8,
) -> Arc<tokio::runtime::Runtime> {
    // The jsonrpc_http_server crate supports two execution models:
    //
    // - By default, it spawns a number of threads - configured with .threads(N) - and runs a
    //   single-threaded futures executor in each thread.
    // - Alternatively when configured with .event_loop_executor(executor) and .threads(1),
    //   it executes all the tasks on the given executor, not spawning any extra internal threads.
    //
    // We use the latter configuration, using a multi threaded tokio runtime as the executor. We
    // do this so we can configure the number of worker threads, the number of blocking threads
    // and then use tokio::task::spawn_blocking() to avoid blocking the worker threads on CPU
    // bound operations like getMultipleAccounts. This results in reduced latency, since fast
    // rpc calls (the majority) are not blocked by slow CPU bound ones.
    //
    // NB: `rpc_blocking_threads` shouldn't be set too high (defaults to num_cpus / 2). Too many
    // (busy) blocking threads could compete with CPU time with other validator threads and
    // negatively impact performance.
    let runtime = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(rpc_threads)
            .max_blocking_threads(rpc_blocking_threads)
            .on_thread_start(move || renice_this_thread(rpc_niceness_adj).unwrap())
            .thread_name("solRpcEl")
            .enable_all()
            .build()
            .expect("Runtime"),
    );
    runtime
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::rpc::{create_validator_exit, tests::new_test_cluster_info},
        solana_ledger::{
            genesis_utils::{create_genesis_config, GenesisConfigInfo},
            get_tmp_ledger_path_auto_delete,
        },
        solana_rpc_client_api::config::RpcContextConfig,
        solana_runtime::bank::Bank,
        solana_sdk::{
            genesis_config::{ClusterType, DEFAULT_GENESIS_ARCHIVE},
            signature::Signer,
        },
        std::{
            io::Write,
            net::{IpAddr, Ipv4Addr},
        },
        tokio::runtime::Runtime,
    };

    #[test]
    fn test_rpc_new() {
        let GenesisConfigInfo {
            genesis_config,
            mint_keypair,
            ..
        } = create_genesis_config(10_000);
        let exit = Arc::new(AtomicBool::new(false));
        let validator_exit = create_validator_exit(exit.clone());
        let bank = Bank::new_for_tests(&genesis_config);
        let cluster_info = Arc::new(new_test_cluster_info());
        let ip_addr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
        let rpc_addr = SocketAddr::new(
            ip_addr,
            solana_net_utils::find_available_port_in_range(ip_addr, (10000, 65535)).unwrap(),
        );
        let bank_forks = BankForks::new_rw_arc(bank);
        let ledger_path = get_tmp_ledger_path_auto_delete!();
        let blockstore = Arc::new(Blockstore::open(ledger_path.path()).unwrap());
        let block_commitment_cache = Arc::new(RwLock::new(BlockCommitmentCache::default()));
        let optimistically_confirmed_bank =
            OptimisticallyConfirmedBank::locked_from_bank_forks_root(&bank_forks);
        let connection_cache = Arc::new(ConnectionCache::new("connection_cache_test"));
        let mut rpc_service = JsonRpcService::new(
            rpc_addr,
            JsonRpcConfig::default(),
            None,
            bank_forks,
            block_commitment_cache,
            blockstore,
            cluster_info,
            None,
            Hash::default(),
            &PathBuf::from("farf"),
            validator_exit,
            exit,
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(true)),
            optimistically_confirmed_bank,
            send_transaction_service::Config {
                retry_rate_ms: 1000,
                leader_forward_count: 1,
                ..send_transaction_service::Config::default()
            },
            Arc::new(MaxSlots::default()),
            Arc::new(LeaderScheduleCache::default()),
            connection_cache,
            Arc::new(AtomicU64::default()),
            Arc::new(AtomicU64::default()),
            Arc::new(PrioritizationFeeCache::default()),
        )
        .expect("assume successful JsonRpcService start");
        let thread = rpc_service.thread_hdl.thread();
        assert_eq!(thread.name().unwrap(), "solJsonRpcSvc");

        assert_eq!(
            10_000,
            rpc_service
                .request_processor
                .get_balance(&mint_keypair.pubkey(), RpcContextConfig::default())
                .unwrap()
                .value
        );
        rpc_service.exit();
        rpc_service.join().unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn test_rpc_unix_socket() {
        use tempfile::NamedTempFile;
        
        let GenesisConfigInfo {
            genesis_config,
            mint_keypair,
            ..
        } = create_genesis_config(10_000);
        let exit = Arc::new(AtomicBool::new(false));
        let validator_exit = create_validator_exit(exit.clone());
        let bank = Bank::new_for_tests(&genesis_config);
        let cluster_info = Arc::new(new_test_cluster_info());
        
        // Create a temporary socket path
        let temp_file = NamedTempFile::new().unwrap();
        let socket_path = temp_file.path().with_extension("sock");
        
        let bank_forks = BankForks::new_rw_arc(bank);
        let ledger_path = get_tmp_ledger_path_auto_delete!();
        let blockstore = Arc::new(Blockstore::open(ledger_path.path()).unwrap());
        let block_commitment_cache = Arc::new(RwLock::new(BlockCommitmentCache::default()));
        let optimistically_confirmed_bank =
            OptimisticallyConfirmedBank::locked_from_bank_forks_root(&bank_forks);
        let connection_cache = Arc::new(ConnectionCache::new("connection_cache_test"));
        
        let mut rpc_service = JsonRpcService::new_unix_socket(
            socket_path.clone(),
            JsonRpcConfig::default(),
            None,
            bank_forks,
            block_commitment_cache,
            blockstore,
            cluster_info,
            None,
            Hash::default(),
            &PathBuf::from("farf"),
            validator_exit,
            exit,
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(true)),
            optimistically_confirmed_bank,
            send_transaction_service::Config {
                retry_rate_ms: 1000,
                leader_forward_count: 1,
                ..send_transaction_service::Config::default()
            },
            Arc::new(MaxSlots::default()),
            Arc::new(LeaderScheduleCache::default()),
            connection_cache,
            Arc::new(AtomicU64::default()),
            Arc::new(AtomicU64::default()),
            Arc::new(PrioritizationFeeCache::default()),
        )
        .expect("assume successful JsonRpcService Unix socket start");
        
        let thread = rpc_service.thread_hdl.thread();
        assert_eq!(thread.name().unwrap(), "solJsonRpcSvc");

        // Give the server a moment to start
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Verify socket file was created
        assert!(socket_path.exists(), "Unix socket file should be created");
        
        rpc_service.exit();
        rpc_service.join().unwrap();
        
        // Verify socket file was cleaned up
        assert!(!socket_path.exists(), "Unix socket file should be cleaned up");
    }

    #[cfg(unix)]
    #[test]
    fn test_rpc_with_both_http_and_unix_socket() {
        use tempfile::NamedTempFile;
        
        let GenesisConfigInfo {
            genesis_config,
            mint_keypair,
            ..
        } = create_genesis_config(10_000);
        let exit = Arc::new(AtomicBool::new(false));
        let validator_exit = create_validator_exit(exit.clone());
        let bank = Bank::new_for_tests(&genesis_config);
        let cluster_info = Arc::new(new_test_cluster_info());
        
        // Create a temporary socket path
        let temp_file = NamedTempFile::new().unwrap();
        let socket_path = temp_file.path().with_extension("sock");
        
        // Get an available port for HTTP
        let ip_addr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
        let rpc_addr = SocketAddr::new(
            ip_addr,
            solana_net_utils::find_available_port_in_range(ip_addr, (10000, 65535)).unwrap(),
        );
        
        let bank_forks = BankForks::new_rw_arc(bank);
        let ledger_path = get_tmp_ledger_path_auto_delete!();
        let blockstore = Arc::new(Blockstore::open(ledger_path.path()).unwrap());
        let block_commitment_cache = Arc::new(RwLock::new(BlockCommitmentCache::default()));
        let optimistically_confirmed_bank =
            OptimisticallyConfirmedBank::locked_from_bank_forks_root(&bank_forks);
        let connection_cache = Arc::new(ConnectionCache::new("connection_cache_test"));
        
        let mut rpc_service = JsonRpcService::new_with_both(
            rpc_addr,
            socket_path.clone(),
            JsonRpcConfig::default(),
            None,
            bank_forks,
            block_commitment_cache,
            blockstore,
            cluster_info,
            None,
            Hash::default(),
            &PathBuf::from("farf"),
            validator_exit,
            exit,
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(true)),
            optimistically_confirmed_bank,
            send_transaction_service::Config {
                retry_rate_ms: 1000,
                leader_forward_count: 1,
                ..send_transaction_service::Config::default()
            },
            Arc::new(MaxSlots::default()),
            Arc::new(LeaderScheduleCache::default()),
            connection_cache,
            Arc::new(AtomicU64::default()),
            Arc::new(AtomicU64::default()),
            Arc::new(PrioritizationFeeCache::default()),
        )
        .expect("assume successful JsonRpcService with both HTTP and Unix socket start");
        
        let thread = rpc_service.thread_hdl.thread();
        assert_eq!(thread.name().unwrap(), "solJsonRpcSvc");

        // Give the servers a moment to start
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        // Verify socket file was created
        assert!(socket_path.exists(), "Unix socket file should be created");
        
        // Test that the request processor works
        assert_eq!(
            10_000,
            rpc_service
                .request_processor
                .get_balance(&mint_keypair.pubkey(), RpcContextConfig::default())
                .unwrap()
                .value
        );
        
        rpc_service.exit();
        rpc_service.join().unwrap();
        
        // Verify socket file was cleaned up
        assert!(!socket_path.exists(), "Unix socket file should be cleaned up");
    }

    fn create_bank_forks() -> Arc<RwLock<BankForks>> {
        let GenesisConfigInfo {
            mut genesis_config, ..
        } = create_genesis_config(10_000);
        genesis_config.cluster_type = ClusterType::MainnetBeta;
        let bank = Bank::new_for_tests(&genesis_config);
        BankForks::new_rw_arc(bank)
    }

    #[test]
    fn test_process_rest_api() {
        let bank_forks = create_bank_forks();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async {
            assert_eq!(
                None,
                handle_rest(&bank_forks, "not-a-supported-rest-api").await
            );

            let circulating_supply = handle_rest(&bank_forks, "/v0/circulating-supply").await;
            assert!(circulating_supply.is_some());

            let total_supply = handle_rest(&bank_forks, "/v0/total-supply").await;
            assert!(total_supply.is_some());

            assert_eq!(
                handle_rest(&bank_forks, "/v0/circulating-supply").await,
                handle_rest(&bank_forks, "/v0/total-supply").await
            );
        });
    }

    #[test]
    fn test_strip_prefix() {
        assert_eq!(RpcRequestMiddleware::strip_leading_slash("/"), Some(""));
        assert_eq!(RpcRequestMiddleware::strip_leading_slash("//"), Some("/"));
        assert_eq!(
            RpcRequestMiddleware::strip_leading_slash("/abc"),
            Some("abc")
        );
        assert_eq!(
            RpcRequestMiddleware::strip_leading_slash("//abc"),
            Some("/abc")
        );
        assert_eq!(
            RpcRequestMiddleware::strip_leading_slash("/./abc"),
            Some("./abc")
        );
        assert_eq!(
            RpcRequestMiddleware::strip_leading_slash("/../abc"),
            Some("../abc")
        );

        assert_eq!(RpcRequestMiddleware::strip_leading_slash(""), None);
        assert_eq!(RpcRequestMiddleware::strip_leading_slash("./"), None);
        assert_eq!(RpcRequestMiddleware::strip_leading_slash("../"), None);
        assert_eq!(RpcRequestMiddleware::strip_leading_slash("."), None);
        assert_eq!(RpcRequestMiddleware::strip_leading_slash(".."), None);
        assert_eq!(RpcRequestMiddleware::strip_leading_slash("abc"), None);
    }

    #[test]
    fn test_is_file_get_path() {
        let ledger_path = get_tmp_ledger_path_auto_delete!();
        let blockstore = Arc::new(Blockstore::open(ledger_path.path()).unwrap());
        let bank_forks = create_bank_forks();
        let optimistically_confirmed_bank =
            OptimisticallyConfirmedBank::locked_from_bank_forks_root(&bank_forks);
        let health = RpcHealth::stub(optimistically_confirmed_bank, blockstore);

        let bank_forks = create_bank_forks();
        let rrm = RpcRequestMiddleware::new(
            ledger_path.path().to_path_buf(),
            None,
            bank_forks.clone(),
            health.clone(),
        );
        let rrm_with_snapshot_config = RpcRequestMiddleware::new(
            ledger_path.path().to_path_buf(),
            Some(SnapshotConfig::default()),
            bank_forks,
            health,
        );

        assert!(rrm.is_file_get_path(DEFAULT_GENESIS_DOWNLOAD_PATH));
        assert!(!rrm.is_file_get_path(DEFAULT_GENESIS_ARCHIVE));
        assert!(!rrm.is_file_get_path("//genesis.tar.bz2"));
        assert!(!rrm.is_file_get_path("/../genesis.tar.bz2"));

        // These two are redirects
        assert!(!rrm.is_file_get_path("/snapshot.tar.bz2"));
        assert!(!rrm.is_file_get_path("/incremental-snapshot.tar.bz2"));

        assert!(!rrm.is_file_get_path(
            "/snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));
        assert!(!rrm.is_file_get_path(
            "/incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));

        assert!(rrm_with_snapshot_config.is_file_get_path(
            "/snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));
        assert!(rrm_with_snapshot_config.is_file_get_path(
            "/snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.zst"
        ));
        assert!(rrm_with_snapshot_config
            .is_file_get_path("/snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.gz"));
        assert!(rrm_with_snapshot_config
            .is_file_get_path("/snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"));

        assert!(rrm_with_snapshot_config.is_file_get_path(
            "/incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));
        assert!(rrm_with_snapshot_config.is_file_get_path(
            "/incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.zst"
        ));
        assert!(rrm_with_snapshot_config.is_file_get_path(
            "/incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.gz"
        ));
        assert!(rrm_with_snapshot_config.is_file_get_path(
            "/incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"
        ));

        assert!(!rrm_with_snapshot_config.is_file_get_path(
            "/snapshot-notaslotnumber-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));
        assert!(!rrm_with_snapshot_config.is_file_get_path(
            "/incremental-snapshot-notaslotnumber-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));
        assert!(!rrm_with_snapshot_config.is_file_get_path(
            "/incremental-snapshot-100-notaslotnumber-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar.bz2"
        ));

        assert!(!rrm_with_snapshot_config.is_file_get_path("../../../test/snapshot-123-xxx.tar"));
        assert!(!rrm_with_snapshot_config
            .is_file_get_path("../../../test/incremental-snapshot-123-456-xxx.tar"));

        assert!(!rrm.is_file_get_path("/"));
        assert!(!rrm.is_file_get_path("//"));
        assert!(!rrm.is_file_get_path("/."));
        assert!(!rrm.is_file_get_path("/./"));
        assert!(!rrm.is_file_get_path("/.."));
        assert!(!rrm.is_file_get_path("/../"));
        assert!(!rrm.is_file_get_path("."));
        assert!(!rrm.is_file_get_path("./"));
        assert!(!rrm.is_file_get_path(".//"));
        assert!(!rrm.is_file_get_path(".."));
        assert!(!rrm.is_file_get_path("../"));
        assert!(!rrm.is_file_get_path("..//"));
        assert!(!rrm.is_file_get_path("🎣"));

        assert!(!rrm_with_snapshot_config
            .is_file_get_path("//snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"));
        assert!(!rrm_with_snapshot_config
            .is_file_get_path("/./snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"));
        assert!(!rrm_with_snapshot_config
            .is_file_get_path("/../snapshot-100-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"));
        assert!(!rrm_with_snapshot_config.is_file_get_path(
            "//incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"
        ));
        assert!(!rrm_with_snapshot_config.is_file_get_path(
            "/./incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"
        ));
        assert!(!rrm_with_snapshot_config.is_file_get_path(
            "/../incremental-snapshot-100-200-AvFf9oS8A8U78HdjT9YG2sTTThLHJZmhaMn2g8vkWYnr.tar"
        ));
    }

    #[test]
    fn test_process_file_get() {
        let runtime = Runtime::new().unwrap();

        let ledger_path = get_tmp_ledger_path_auto_delete!();
        let blockstore = Arc::new(Blockstore::open(ledger_path.path()).unwrap());
        let genesis_path = ledger_path.path().join(DEFAULT_GENESIS_ARCHIVE);
        let bank_forks = create_bank_forks();
        let optimistically_confirmed_bank =
            OptimisticallyConfirmedBank::locked_from_bank_forks_root(&bank_forks);
        let rrm = RpcRequestMiddleware::new(
            ledger_path.path().to_path_buf(),
            None,
            bank_forks,
            RpcHealth::stub(optimistically_confirmed_bank, blockstore),
        );

        // File does not exist => request should fail.
        let action = rrm.process_file_get(DEFAULT_GENESIS_DOWNLOAD_PATH);
        if let RequestMiddlewareAction::Respond { response, .. } = action {
            let response = runtime.block_on(response);
            let response = response.unwrap();
            assert_ne!(response.status(), 200);
        } else {
            panic!("Unexpected RequestMiddlewareAction variant");
        }

        {
            let mut file = std::fs::File::create(&genesis_path).unwrap();
            file.write_all(b"should be ok").unwrap();
        }

        // Normal file exist => request should succeed.
        let action = rrm.process_file_get(DEFAULT_GENESIS_DOWNLOAD_PATH);
        if let RequestMiddlewareAction::Respond { response, .. } = action {
            let response = runtime.block_on(response);
            let response = response.unwrap();
            assert_eq!(response.status(), 200);
        } else {
            panic!("Unexpected RequestMiddlewareAction variant");
        }

        std::fs::remove_file(&genesis_path).unwrap();
        {
            let mut file = std::fs::File::create(ledger_path.path().join("wrong")).unwrap();
            file.write_all(b"wrong file").unwrap();
        }
        symlink::symlink_file("wrong", &genesis_path).unwrap();

        // File is a symbolic link => request should fail.
        let action = rrm.process_file_get(DEFAULT_GENESIS_DOWNLOAD_PATH);
        if let RequestMiddlewareAction::Respond { response, .. } = action {
            let response = runtime.block_on(response);
            let response = response.unwrap();
            assert_ne!(response.status(), 200);
        } else {
            panic!("Unexpected RequestMiddlewareAction variant");
        }
    }
}

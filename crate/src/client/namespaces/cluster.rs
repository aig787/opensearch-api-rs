//! Cluster namespace for OpenSearch

use crate::error::Error;
use derive_builder::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_with::skip_serializing_none;

/// Client namespace for cluster-related operations
#[derive(Debug, Clone)]
pub struct ClusterNamespace {
    client: crate::client::Client,
}

/// Cluster health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterHealthStatus {
    /// Green: all primary and replica shards are active
    Green,
    /// Yellow: all primary shards are active, but not all replica shards are active
    Yellow,
    /// Red: not all primary shards are active
    Red,
}

/// Response from the cluster health API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealthResponse {
    /// Cluster name
    pub cluster_name: String,
    /// Cluster status
    pub status: ClusterHealthStatus,
    /// Whether the cluster timed out while gathering data
    pub timed_out: bool,
    /// Number of nodes in the cluster
    pub number_of_nodes: u32,
    /// Number of data nodes in the cluster
    pub number_of_data_nodes: u32,
    /// Number of active primary shards
    pub active_primary_shards: u32,
    /// Number of active shards
    pub active_shards: u32,
    /// Number of relocating shards
    pub relocating_shards: u32,
    /// Number of initializing shards
    pub initializing_shards: u32,
    /// Number of unassigned shards
    pub unassigned_shards: u32,
    /// Percentage of shards that are active
    pub active_shards_percent_as_number: f64,
    /// Number of delayed unassigned shards
    pub delayed_unassigned_shards: u32,
    /// Number of pending tasks
    pub number_of_pending_tasks: u32,
    /// Number of in-flight fetch operations
    pub number_of_in_flight_fetch: u32,
    /// Task max wait time in queue (ms)
    pub task_max_waiting_in_queue_millis: u64,
    /// Whether the cluster is fully formed
    pub cluster_formed: Option<bool>,
    /// Indices health information, if requested with ?level=indices
    #[serde(default)]
    pub indices: HashMap<String, IndexHealthInfo>,
}

/// Health information for an index
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexHealthInfo {
    /// Index status
    pub status: ClusterHealthStatus,
    /// Number of active primary shards
    pub active_primary_shards: u32,
    /// Number of active shards
    pub active_shards: u32,
    /// Number of relocating shards
    pub relocating_shards: u32,
    /// Number of initializing shards
    pub initializing_shards: u32,
    /// Number of unassigned shards
    pub unassigned_shards: u32,
    /// Shard health details, if requested with ?level=shards
    #[serde(default)]
    pub shards: HashMap<String, Vec<ShardHealthInfo>>,
}

/// Health information for a shard
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardHealthInfo {
    /// Shard status
    pub status: ClusterHealthStatus,
    /// Whether this is a primary shard
    pub primary: bool,
    /// ID of the node where this shard is allocated
    pub node: Option<String>,
    /// Whether the shard is relocating
    pub relocating_node: Option<String>,
    /// Shard allocation explanation
    pub allocation_explanation: Option<String>,
}

/// Response from the cluster stats API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStatsResponse {
    /// Timestamp of the response
    pub timestamp: u64,
    /// Cluster name
    pub cluster_name: String,
    /// Cluster UUID
    pub cluster_uuid: String,
    /// Status of the cluster
    pub status: ClusterHealthStatus,
    /// Indices statistics
    pub indices: ClusterIndicesStats,
    /// Nodes statistics
    pub nodes: ClusterNodesStats,
}

/// Indices statistics for the cluster
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterIndicesStats {
    /// Count of indices in the cluster
    pub count: u32,
    /// Shard statistics
    pub shards: ShardStats,
    /// Document statistics
    pub docs: DocStats,
    /// Storage statistics
    pub store: StoreStats,
    /// Field data statistics
    pub fielddata: FieldDataStats,
    /// Query cache statistics
    pub query_cache: QueryCacheStats,
    /// Completion suggester statistics
    pub completion: CompletionStats,
    /// Segments statistics
    pub segments: SegmentsStats,
}

/// Shard statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardStats {
    /// Total number of shards
    pub total: u32,
    /// Number of primary shards
    pub primaries: u32,
    /// Number of replica shards
    pub replication: f32,
    /// Shard counts per index
    pub index: IndexShardStats,
}

/// Index-level shard statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexShardStats {
    /// Shard statistics by count
    pub shards: HashMap<String, f32>,
    /// Primary shard statistics
    pub primaries: HashMap<String, f32>,
    /// Replication factor statistics
    pub replication: HashMap<String, f32>,
}

/// Document statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocStats {
    /// Count of documents in the cluster
    pub count: u64,
    /// Count of deleted documents
    pub deleted: u64,
}

/// Storage statistics
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreStats {
    /// Size in bytes
    pub size_in_bytes: Option<u64>,
    /// Throttle time in milliseconds
    pub throttle_time_in_millis: Option<u64>,
}

/// Field data statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDataStats {
    /// Memory usage in bytes
    pub memory_size_in_bytes: Option<u64>,
    /// Cache evictions
    pub evictions: Option<u64>,
}

/// Query cache statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCacheStats {
    /// Memory usage in bytes
    pub memory_size_in_bytes: Option<u64>,
    /// Total number of cache entries
    pub total_count: Option<u64>,
    /// Cache hit count
    pub hit_count: Option<u64>,
    /// Cache miss count
    pub miss_count: Option<u64>,
    /// Cache evictions
    pub evictions: Option<u64>,
}

/// Completion suggester statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionStats {
    /// Size in bytes
    pub size_in_bytes: Option<u64>,
}

/// Segments statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentsStats {
    /// Count of segments
    pub count: Option<u32>,
    /// Memory usage in bytes
    pub memory_in_bytes: Option<u64>,
    /// Terms memory usage
    pub terms_memory_in_bytes: Option<u64>,
    /// Stored fields memory usage
    pub stored_fields_memory_in_bytes: Option<u64>,
    /// Term vectors memory usage
    pub term_vectors_memory_in_bytes: Option<u64>,
    /// Norms memory usage
    pub norms_memory_in_bytes: Option<u64>,
    /// Points memory usage
    pub points_memory_in_bytes: Option<u64>,
    /// Doc values memory usage
    pub doc_values_memory_in_bytes: Option<u64>,
    /// Index writer memory usage
    pub index_writer_memory_in_bytes: Option<u64>,
    /// Version map memory usage
    pub version_map_memory_in_bytes: Option<u64>,
    /// Fixed bit set memory usage
    pub fixed_bit_set_memory_in_bytes: Option<u64>,
    /// Max unsafe auto ID timestamp
    pub max_unsafe_auto_id_timestamp: Option<i64>,
    /// File sizes
    #[serde(default)]
    pub file_sizes: HashMap<String, u64>,
}

/// Nodes statistics for the cluster
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodesStats {
    /// Count of nodes
    pub count: ClusterNodeCounts,
    /// Versions of nodes
    pub versions: Vec<String>,
    /// OS statistics
    pub os: OperatingSystemStats,
    /// Process statistics
    pub process: ProcessStats,
    /// JVM statistics
    pub jvm: JvmStats,
    /// File system statistics
    pub fs: FileSystemStats,
    /// Network types
    pub network_types: NetworkTypeStats,
    /// List of plugins
    #[serde(default)]
    pub plugins: Vec<NodePlugin>,
    /// Discovery types
    #[serde(default)]
    pub discovery_types: HashMap<String, usize>,
    /// Packaging types
    #[serde(default)]
    pub packaging_types: Vec<PackagingType>,
    /// Ingest info
    pub ingest: Option<ClusterIngestInfo>,
    /// Catch all for other fields we might not explicitly model
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

/// Packaging type information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagingType {
    /// Type of packaging
    #[serde(rename = "type")]
    pub packaging_type: String,
    /// Count of nodes with this packaging
    pub count: usize,
}

/// Ingest information for the cluster
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterIngestInfo {
    /// Number of pipelines
    pub number_of_pipelines: usize,
    /// Processor statistics
    pub processor_stats: HashMap<String, serde_json::Value>,
}

/// Counts of different node types
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeCounts {
    /// Total number of nodes
    pub total: u32,
    /// Number of data nodes
    pub data: Option<u32>,
    /// Number of coordinating nodes
    pub coordinating_only: Option<u32>,
    /// Number of master nodes
    pub master: Option<u32>,
    /// Number of ingest nodes
    pub ingest: Option<u32>,
}

/// Operating system statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingSystemStats {
    /// Available processors
    pub available_processors: Option<u32>,
    /// Allocated processors
    pub allocated_processors: Option<u32>,
    /// Memory stats
    pub mem: Option<MemoryStats>,
    /// Distribution of operating systems
    pub names: Option<Vec<OsDistribution>>,
    /// Pretty names of operating systems
    pub pretty_names: Option<Vec<OsDistribution>>,
    /// CPU stats
    pub cpu: Option<CpuStats>,
}

/// Memory statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memory in bytes
    pub total_in_bytes: Option<u64>,
    /// Free memory in bytes
    pub free_in_bytes: Option<u64>,
    /// Used memory in bytes
    pub used_in_bytes: Option<u64>,
    /// Free percent
    pub free_percent: Option<u32>,
    /// Used percent
    pub used_percent: Option<u32>,
}

/// Operating system distribution
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsDistribution {
    /// Name of the OS
    #[serde(alias = "pretty_name")]
    pub name: String,
    /// Count of nodes with this OS
    pub count: u32,
}

/// CPU statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    /// Percent of CPU used
    pub percent: Option<u32>,
    /// Load average (1m, 5m, 15m)
    #[serde(default)]
    pub load_average: HashMap<String, f32>,
}

/// Process statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStats {
    /// CPU usage
    pub cpu: Option<ProcessCpuStats>,
    /// Open file descriptors
    pub open_file_descriptors: Option<FileDescriptorStats>,
}

/// Process CPU statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessCpuStats {
    /// Percent of CPU used by the process
    pub percent: Option<u32>,
}

/// File descriptor statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDescriptorStats {
    /// Min open file descriptors
    pub min: Option<u32>,
    /// Max open file descriptors
    pub max: Option<u32>,
    /// Average open file descriptors
    pub avg: Option<f32>,
}

/// JVM statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JvmStats {
    /// Maximum heap memory
    pub max_uptime_in_millis: Option<u64>,
    /// JVM versions
    pub versions: Vec<JvmVersion>,
    /// Memory pools
    pub mem: JvmMemoryStats,
    /// Thread statistics
    pub threads: Option<u32>,
}

/// JVM version information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JvmVersion {
    /// Version string
    pub version: String,
    /// VM name
    pub vm_name: String,
    /// VM version
    pub vm_version: String,
    /// VM vendor
    pub vm_vendor: String,
    /// Count of nodes with this JVM
    pub count: u32,
}

/// JVM memory statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JvmMemoryStats {
    /// Heap usage statistics
    pub heap_used_in_bytes: Option<u64>,
    /// Heap max
    pub heap_max_in_bytes: Option<u64>,
}

/// File system statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemStats {
    /// Total size
    pub total_in_bytes: Option<u64>,
    /// Free space
    pub free_in_bytes: Option<u64>,
    /// Available space
    pub available_in_bytes: Option<u64>,
}

/// Network type statistics
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTypeStats {
    /// Transport types
    pub transport_types: Option<HashMap<String, u32>>,
    /// HTTP types
    pub http_types: Option<HashMap<String, u32>>,
}

/// Response from the cluster state API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStateResponse {
    /// Cluster name
    pub cluster_name: String,
    /// Cluster UUID
    pub cluster_uuid: String,
    /// Version information
    pub version: Option<i64>,
    /// State UUID
    pub state_uuid: Option<String>,
    /// Master node information (older OpenSearch versions)
    pub master_node: Option<String>,
    /// Cluster manager node information (newer OpenSearch versions)
    pub cluster_manager_node: Option<String>,
    /// List of blocks
    #[serde(default)]
    pub blocks: ClusterBlocks,
    /// Nodes information
    pub nodes: HashMap<String, ClusterNodeInfo>,
    /// Metadata
    pub metadata: ClusterMetadata,
    /// Routing table
    pub routing_table: RoutingTable,
    /// Routing nodes
    pub routing_nodes: RoutingNodes,
    /// Custom cluster information
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Cluster blocks
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterBlocks {
    /// Global blocks
    #[serde(default)]
    pub global: HashMap<String, ClusterBlock>,
    /// Blocks by index
    #[serde(default)]
    pub indices: HashMap<String, HashMap<String, ClusterBlock>>,
}

/// Cluster block information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterBlock {
    /// Block description
    pub description: String,
    /// Block reason
    pub reason: String,
    /// Retry after time
    pub retry_after: Option<String>,
}

/// Node information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeInfo {
    /// Node name
    pub name: String,
    /// Transport address
    pub transport_address: String,
    /// Attributes
    pub attributes: Option<HashMap<String, String>>,
}

/// Cluster metadata
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMetadata {
    /// Cluster UUID
    pub cluster_uuid: String,
    /// Whether cluster UUID is committed
    pub cluster_uuid_committed: Option<bool>,
    /// Cluster coordination
    pub cluster_coordination: ClusterCoordination,
    /// Templates
    #[serde(default)]
    pub templates: HashMap<String, serde_json::Value>,
    /// Indices metadata
    #[serde(default)]
    pub indices: HashMap<String, IndexMetadata>,
    /// Index graveyard
    pub index_graveyard: Option<serde_json::Value>,
}

/// Cluster coordination information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCoordination {
    /// Current term
    pub term: u64,
    /// Last committed configuration
    pub last_committed_config: Vec<String>,
    /// Last accepted configuration
    pub last_accepted_config: Vec<String>,
    /// Voting configuration exclusions
    pub voting_config_exclusions: Vec<VotingConfigExclusion>,
}

/// Voting configuration exclusion
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingConfigExclusion {
    /// Node ID
    pub node_id: Option<String>,
    /// Node name
    pub node_name: Option<String>,
}

/// Index metadata
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    /// Index version
    pub version: Option<i64>,
    /// Mapping version
    pub mapping_version: Option<i64>,
    /// Settings version
    pub settings_version: Option<i64>,
    /// Aliases version
    pub aliases_version: Option<i64>,
    /// Routing num shards
    pub routing_num_shards: Option<i64>,
    /// Index state
    pub state: String,
    /// Primary terms
    #[serde(default)]
    pub primary_terms: HashMap<String, u64>,
    /// In-sync allocation IDs
    #[serde(default)]
    pub in_sync_allocations: HashMap<String, Vec<String>>,
    /// Settings
    pub settings: IndexSettings,
    /// Mappings
    #[serde(default)]
    pub mappings: HashMap<String, serde_json::Value>,
    /// Aliases
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Rollover info
    pub rollover_info: Option<serde_json::Value>,
    /// System index flag
    pub system: Option<bool>,
}

/// Index settings
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSettings {
    /// Index settings
    pub index: IndexSettingsDetails,
}

/// Index settings details
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSettingsDetails {
    /// Replication type
    pub replication: Option<HashMap<String, String>>,
    /// Hidden flag
    pub hidden: Option<String>,
    /// Creation date
    pub creation_date: String,
    /// Number of shards
    pub number_of_shards: String,
    /// Auto expand replicas
    pub auto_expand_replicas: Option<String>,
    /// Provided name
    pub provided_name: Option<String>,
    /// Number of replicas
    pub number_of_replicas: String,
    /// UUID
    pub uuid: String,
    /// Version
    pub version: HashMap<String, String>,
}

/// Routing table
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTable {
    /// Indices routing
    pub indices: Option<HashMap<String, IndexRoutingTable>>,
}

/// Index routing table
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRoutingTable {
    /// Shards routing
    pub shards: Option<HashMap<String, Vec<ShardRouting>>>,
}

/// Shard routing information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardRouting {
    /// State of the shard
    pub state: String,
    /// Whether this is primary
    pub primary: bool,
    /// Node where the shard is allocated
    pub node: Option<String>,
    /// Whether the shard is relocating
    pub relocating_node: Option<String>,
    /// Shard information
    pub shard: u32,
    /// Index name
    pub index: String,
}

/// Routing nodes
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingNodes {
    /// Unassigned shards
    pub unassigned: Vec<ShardRouting>,
    /// Node assignments
    pub nodes: Option<HashMap<String, Vec<ShardRouting>>>,
}

/// Metadata about node info request execution
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodesMetadata {
    /// Total number of nodes queried
    pub total: u32,
    /// Number of nodes that responded successfully
    pub successful: u32,
    /// Number of nodes that failed to respond
    pub failed: u32,
}

/// Response from the nodes info API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodesInfoResponse {
    /// Metadata about the node info request execution
    #[serde(rename = "_nodes")]
    pub nodes_metadata: NodesMetadata,
    /// Cluster name
    pub cluster_name: String,
    /// Information about nodes in the cluster
    pub nodes: HashMap<String, NodeInfo>,
}

/// Detailed information about a node
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node name
    pub name: String,
    /// Transport address
    pub transport_address: String,
    /// Host name/IP
    pub host: String,
    /// IP address
    pub ip: String,
    /// Version information
    pub version: String,
    /// Build information
    pub build: Option<String>,
    /// Build type
    pub build_type: Option<String>,
    /// Build hash
    pub build_hash: Option<String>,
    /// Total indexing buffer
    pub total_indexing_buffer: Option<u64>,
    /// HTTP information
    pub http: Option<NodeHttpInfo>,
    /// Network information
    pub network: Option<NodeNetworkInfo>,
    /// JVM information
    pub jvm: Option<NodeJvmInfo>,
    /// Operating system information
    pub os: Option<NodeOsInfo>,
    /// Process information
    pub process: Option<NodeProcessInfo>,
    /// Thread pool information
    pub thread_pool: Option<HashMap<String, ThreadPoolInfo>>,
    /// Transport information
    pub transport: Option<TransportInfo>,
    /// Settings
    pub settings: Option<HashMap<String, serde_json::Value>>,
    /// Plugins
    pub plugins: Option<Vec<NodePlugin>>,
    /// Modules
    pub modules: Option<Vec<NodePlugin>>,
    /// Ingest processors
    pub ingest: Option<IngestInfo>,
    /// Aggregations
    pub aggregations: Option<HashMap<String, AggregationInfo>>,
    /// Search pipelines
    pub search_pipelines: Option<SearchPipelineInfo>,
    /// Roles of this node (master, data, etc.)
    #[serde(default)]
    pub roles: Vec<String>,
    /// Attributes
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// Thread pool information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolInfo {
    /// Type of thread pool
    #[serde(rename = "type")]
    pub pool_type: String,
    /// Size of thread pool
    pub size: Option<i32>,
    /// Core size for scaling thread pools
    pub core: Option<i32>,
    /// Max size for scaling thread pools
    pub max: Option<i32>,
    /// Keep alive time
    pub keep_alive: Option<String>,
    /// Queue size
    pub queue_size: i32,
}

/// Transport information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportInfo {
    /// Bound addresses
    pub bound_address: Vec<String>,
    /// Publish address
    pub publish_address: Option<String>,
    /// Profiles
    pub profiles: Option<HashMap<String, serde_json::Value>>,
}

/// Ingest processor information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestInfo {
    /// Available processors
    pub processors: Vec<HashMap<String, serde_json::Value>>,
}

/// Aggregation information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationInfo {
    /// Types supported by this aggregation
    pub types: Vec<String>,
}

/// Search pipeline information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPipelineInfo {
    /// Request processors
    pub request_processors: Vec<HashMap<String, String>>,
    /// Response processors
    pub response_processors: Vec<HashMap<String, String>>,
}

/// HTTP information for a node
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHttpInfo {
    /// Whether HTTP is enabled
    pub enabled: Option<bool>,
    /// Bound address
    pub bound_address: Vec<String>,
    /// Publish address
    pub publish_address: String,
    /// Maximum content length
    pub max_content_length_in_bytes: Option<u64>,
}

/// Network information for a node
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeNetworkInfo {
    /// Refresh interval
    pub refresh_interval: Option<i64>,
    /// Available interfaces
    pub interfaces: NodeNetworkInterfaces,
}

/// Network interfaces information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeNetworkInterfaces {
    /// Network interfaces
    pub lo: Option<Vec<NodeNetworkInterface>>,
    /// Other interfaces
    #[serde(flatten)]
    pub other: HashMap<String, Vec<NodeNetworkInterface>>,
}

/// Network interface information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeNetworkInterface {
    /// Interface address
    pub address: String,
    /// Interface name
    pub name: Option<String>,
    /// MAC address
    pub mac_address: Option<String>,
}

/// JVM information for a node
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeJvmInfo {
    /// Process ID
    pub pid: Option<i64>,
    /// JVM version
    pub version: String,
    /// VM name
    pub vm_name: String,
    /// VM version
    pub vm_version: String,
    /// VM vendor
    pub vm_vendor: String,
    /// Memory statistics
    pub mem: NodeJvmMemoryInfo,
    /// GC collectors
    pub gc_collectors: Option<Vec<String>>,
    /// Memory pools
    pub memory_pools: Option<Vec<String>>,
    /// Input arguments
    pub input_arguments: Option<Vec<String>>,
}

/// JVM memory information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeJvmMemoryInfo {
    /// Heap initial in bytes
    pub heap_init_in_bytes: Option<u64>,
    /// Heap max in bytes
    pub heap_max_in_bytes: Option<u64>,
    /// Non-heap initial in bytes
    pub non_heap_init_in_bytes: Option<u64>,
    /// Non-heap max in bytes
    pub non_heap_max_in_bytes: Option<u64>,
    /// Direct memory max in bytes
    pub direct_max_in_bytes: Option<u64>,
}

/// Operating system information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeOsInfo {
    /// OS name
    pub name: Option<String>,
    /// Pretty name
    pub pretty_name: Option<String>,
    /// Architecture
    pub arch: String,
    /// Available processors
    pub available_processors: Option<i32>,
    /// Allocated processors
    pub allocated_processors: Option<i32>,
}

/// Process information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeProcessInfo {
    /// Process ID
    pub id: Option<i64>,
    /// Refresh interval
    pub refresh_interval_in_millis: Option<i64>,
    /// Process features
    pub mlockall: Option<bool>,
}

/// Plugin information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePlugin {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// OpenSearch version
    pub opensearch_version: Option<String>,
    /// Java version
    pub java_version: Option<String>,
    /// Description
    pub description: String,
    /// Class name
    pub classname: String,
    /// Custom folder name
    #[serde(default)]
    pub custom_foldername: String,
    /// Whether it has native controller
    pub has_native_controller: bool,
    /// Extended plugins
    #[serde(default)]
    pub extended_plugins: Vec<String>,
    /// Optional extended plugins
    #[serde(default)]
    pub optional_extended_plugins: Vec<String>,
    /// Whether this is a bundled JDK
    pub bundled_jdk: Option<bool>,
    /// Whether it's using a bundled JDK
    pub using_bundled_jdk: Option<bool>,
    /// Catch all for other fields we might not explicitly model
    #[serde(flatten)]
    pub other: Option<HashMap<String, serde_json::Value>>,
}

/// Response from the cluster settings API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterSettingsResponse {
    /// Persistent settings
    pub persistent: HashMap<String, serde_json::Value>,
    /// Transient settings
    pub transient: HashMap<String, serde_json::Value>,
    /// Default settings
    pub defaults: Option<HashMap<String, serde_json::Value>>,
}

/// Request for the cluster put settings API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ClusterSettingsRequest {
    /// Persistent settings
    #[builder(default)]
    persistent: Option<HashMap<String, serde_json::Value>>,
    /// Transient settings
    #[builder(default)]
    transient: Option<HashMap<String, serde_json::Value>>,
}

impl ClusterSettingsRequest {
    /// Create a new builder for ClusterSettingsRequest
    pub fn builder() -> ClusterSettingsRequestBuilder {
        ClusterSettingsRequestBuilder::default()
    }
}

/// Builder for the cluster allocation explain API
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct AllocationExplainRequest {
    /// Index name
    #[builder(default)]
    index: Option<String>,
    /// Shard ID
    #[builder(default)]
    shard: Option<i32>,
    /// Whether this is a primary shard
    #[builder(default)]
    primary: Option<bool>,
}

impl AllocationExplainRequest {
    /// Create a new builder for AllocationExplainRequest
    pub fn builder() -> AllocationExplainRequestBuilder {
        AllocationExplainRequestBuilder::default()
    }
}

/// Response from the cluster allocation explain API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationExplainResponse {
    /// Index name
    pub index: String,
    /// Shard ID
    pub shard: i32,
    /// Whether this is a primary shard
    pub primary: bool,
    /// Current state of the shard
    pub current_state: String,
    /// Unassigned information
    pub unassigned_info: Option<UnassignedInfo>,
    /// Decisions for allocation
    pub allocation_decisions: Option<Vec<AllocationDecision>>,
    /// Current node
    pub current_node: Option<ClusterNodeShardInfo>,
    /// Reason for allocation
    pub explanation: Option<String>,
}

/// Unassigned shard information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignedInfo {
    /// Reason for being unassigned
    pub reason: String,
    /// When the shard became unassigned
    pub at: String,
    /// Last allocation status
    pub last_allocation_status: Option<String>,
    /// Failure details
    pub details: Option<String>,
    /// Allocation delay
    pub allocation_delay_in_millis: Option<i64>,
    /// Unassigned for time in milliseconds
    pub unassigned_for_in_millis: i64,
}

/// Decision for allocating a shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationDecision {
    /// Node ID
    pub node_id: String,
    /// Node name
    pub node_name: String,
    /// Transport address
    pub transport_address: String,
    /// Node attributes
    pub node_attributes: HashMap<String, String>,
    /// Decision - whether allocation is allowed
    pub decision: String,
    /// Explanation for the decision
    pub explanation: String,
    /// Store information
    pub store: Option<StoreInfo>,
    /// Deciders explaining the decision
    pub deciders: Option<Vec<DeciderInfo>>,
}

/// Store information for a shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreInfo {
    /// Whether the shard is found
    pub found: bool,
    /// Store allocation ID
    pub in_sync: Option<bool>,
    /// Store allocation ID
    pub allocation_id: Option<String>,
}

/// Decider information explaining allocation decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeciderInfo {
    /// Decider name
    pub decider: String,
    /// Decision
    pub decision: String,
    /// Explanation
    pub explanation: String,
}

/// Current node info for a shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeShardInfo {
    /// Node ID
    pub node_id: String,
    /// Node name
    pub node_name: String,
    /// Transport address
    pub transport_address: String,
    /// Node attributes
    pub node_attributes: HashMap<String, String>,
}

/// Response from the pending tasks API
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTasksResponse {
    /// List of pending tasks
    pub tasks: Vec<PendingTask>,
}

/// A pending task in the cluster
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTask {
    /// Insertion order
    pub insert_order: i64,
    /// Task priority
    pub priority: String,
    /// Source of the task
    pub source: String,
    /// Time in queue
    pub time_in_queue_millis: i64,
    /// Formatted time in queue
    pub time_in_queue: String,
}

impl ClusterNamespace {
    /// Create a new cluster namespace with the given client
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self { client }
    }

    /// Get cluster health
    ///
    /// Returns information about the health of the cluster.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// let health = client.cluster().health().await?;
    /// println!("Cluster status: {:?}", health.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn health(&self) -> Result<ClusterHealthResponse, Error> {
        self.client
            .request::<(), _>(Method::GET, "/_cluster/health", None)
            .await
    }

    /// Get cluster stats
    ///
    /// Returns statistics about the cluster.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// let stats = client.cluster().stats().await?;
    /// println!("Number of nodes: {}", stats.nodes.count.total);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn stats(&self) -> Result<ClusterStatsResponse, Error> {
        self.client
            .request::<(), _>(Method::GET, "/_cluster/stats", None)
            .await
    }

    /// Get cluster state
    ///
    /// Returns a comprehensive view of the cluster state.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// let state = client.cluster().state().await?;
    /// println!("Cluster name: {}", state.cluster_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn state(&self) -> Result<ClusterStateResponse, Error> {
        self.client
            .request::<(), _>(Method::GET, "/_cluster/state", None)
            .await
    }

    /// Get information about the nodes in the cluster
    ///
    /// Returns information about nodes in the cluster including settings, attributes, and plugins.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// let nodes_info = client.cluster().nodes_info().await?;
    /// for (node_id, node) in nodes_info.nodes {
    ///     println!("Node: {} ({})", node.name, node_id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn nodes_info(&self) -> Result<NodesInfoResponse, Error> {
        self.client
            .request::<(), _>(Method::GET, "/_nodes", None)
            .await
    }

    /// Get cluster settings
    ///
    /// Returns the current cluster settings including default settings if requested.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// let settings = client.cluster().get_settings().await?;
    /// println!("Persistent settings: {:?}", settings.persistent);
    /// println!("Transient settings: {:?}", settings.transient);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_settings(&self) -> Result<ClusterSettingsResponse, Error> {
        self.client
            .request::<(), _>(Method::GET, "/_cluster/settings", None)
            .await
    }

    /// Update cluster settings
    ///
    /// Updates the cluster settings either persistently or transiently.
    ///
    /// # Arguments
    ///
    /// * `settings` - The settings to update
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// # use std::collections::HashMap;
    /// # use serde_json::json;
    /// use opensearch_api::cluster::ClusterSettingsRequest;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// // Create a settings update using the builder
    /// let mut transient = HashMap::new();
    /// transient.insert(
    ///     "cluster.routing.allocation.disk.threshold_enabled".to_string(),
    ///     json!(false)
    /// );
    ///
    /// let settings = ClusterSettingsRequest::builder()
    ///     .transient(transient)
    ///     .build()?;
    ///
    /// // Update the settings
    /// let result = client.cluster().put_settings(settings).await?;
    /// println!("Updated settings: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn put_settings(
        &self,
        settings: ClusterSettingsRequest,
    ) -> Result<ClusterSettingsResponse, Error> {
        self.client
            .request::<_, _>(Method::PUT, "/_cluster/settings", Some(&settings))
            .await
    }

    /// Explain cluster allocation
    ///
    /// Explains why a shard is or isn't allocated to a node.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// use opensearch_api::cluster::AllocationExplainRequest;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// // Get allocation explanation for an unassigned shard
    /// let explanation = client.cluster().allocation_explain(None).await?;
    /// println!("Allocation explanation: {:?}", explanation.explanation);
    ///
    /// // Optionally, specify a specific shard to explain
    /// let request = AllocationExplainRequest::builder()
    ///     .index("my-index")
    ///     .shard(0)
    ///     .primary(true)
    ///     .build()?;
    ///     
    /// let specific_explanation = client.cluster().allocation_explain(Some(request)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn allocation_explain(
        &self,
        request: Option<AllocationExplainRequest>,
    ) -> Result<AllocationExplainResponse, Error> {
        match request {
            Some(req) => {
                self.client
                    .request::<_, _>(Method::POST, "/_cluster/allocation/explain", Some(&req))
                    .await
            }
            None => {
                self.client
                    .request::<(), _>(Method::GET, "/_cluster/allocation/explain", None)
                    .await
            }
        }
    }

    /// Get pending tasks
    ///
    /// Returns a list of any cluster-level changes that have not yet been executed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::Client;
    /// # use anyhow::Result;
    /// #
    /// # async fn example() -> Result<()> {
    /// let client = Client::builder()
    ///     .base_url("https://localhost:9200")
    ///     .username("admin")
    ///     .password("admin")
    ///     .build()?;
    ///
    /// // Get pending tasks
    /// let pending = client.cluster().pending_tasks().await?;
    /// println!("Number of pending tasks: {}", pending.tasks.len());
    /// for task in pending.tasks {
    ///     println!("Task: {} ({})", task.source, task.priority);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pending_tasks(&self) -> Result<PendingTasksResponse, Error> {
        self.client
            .request::<(), _>(Method::GET, "/_cluster/pending_tasks", None)
            .await
    }
}

impl crate::client::Client {
    /// Access the cluster namespace
    pub fn cluster(&self) -> ClusterNamespace {
        ClusterNamespace::new(self.clone())
    }
}

// Complex Rust service module with various patterns
// This file contains multiple structs, enums, traits, and complex logic
// to test the ChallengeGenerator with realistic code patterns

use std::collections::{HashMap, BTreeMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

/// Configuration struct for the service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub max_connections: usize,
    pub timeout: Duration,
    pub retry_count: u32,
    pub enable_logging: bool,
    pub cache_size: usize,
    pub worker_threads: usize,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_logging: true,
            cache_size: 1000,
            worker_threads: 4,
        }
    }
}

/// Error types for the service
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceError {
    ConnectionFailed(String),
    Timeout,
    InvalidInput(String),
    CacheFull,
    WorkerPanic,
    ConfigurationError(String),
    NetworkError { code: u16, message: String },
    ParseError { line: usize, column: usize, details: String },
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            ServiceError::Timeout => write!(f, "Operation timed out"),
            ServiceError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ServiceError::CacheFull => write!(f, "Cache is full"),
            ServiceError::WorkerPanic => write!(f, "Worker thread panicked"),
            ServiceError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            ServiceError::NetworkError { code, message } => {
                write!(f, "Network error {}: {}", code, message)
            }
            ServiceError::ParseError { line, column, details } => {
                write!(f, "Parse error at {}:{}: {}", line, column, details)
            }
        }
    }
}

impl std::error::Error for ServiceError {}

/// Request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: String,
    pub method: RequestMethod,
    pub data: RequestData,
    pub headers: HashMap<String, String>,
    pub timestamp: u64,
    pub priority: Priority,
}

/// HTTP-like request methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

/// Request data variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestData {
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Text(String),
    FormData(HashMap<String, String>),
    Empty,
}

/// Priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub id: String,
    pub status: ResponseStatus,
    pub data: Option<ResponseData>,
    pub headers: HashMap<String, String>,
    pub processing_time: Duration,
    pub worker_id: usize,
}

/// Response status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    NotFound = 404,
    InternalError = 500,
    ServiceUnavailable = 503,
}

/// Response data variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseData {
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Text(String),
    Stream(String), // URL or identifier for streaming data
}

/// Cache entry with metadata
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    data: T,
    created_at: Instant,
    access_count: u64,
    last_accessed: Instant,
    size: usize,
}

impl<T> CacheEntry<T> {
    fn new(data: T, size: usize) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            access_count: 0,
            last_accessed: now,
            size,
        }
    }

    fn access(&mut self) -> &T {
        self.access_count += 1;
        self.last_accessed = Instant::now();
        &self.data
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }

    fn score(&self) -> f64 {
        // LFU + LRU hybrid scoring
        let frequency_score = self.access_count as f64;
        let recency_score = 1.0 / (self.last_accessed.elapsed().as_secs_f64() + 1.0);
        frequency_score * 0.7 + recency_score * 0.3
    }
}

/// Thread-safe LRU cache implementation
pub struct LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    capacity: usize,
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    access_order: Arc<Mutex<Vec<K>>>,
    current_size: Arc<Mutex<usize>>,
    ttl: Duration,
    stats: Arc<Mutex<CacheStats>>,
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
    pub capacity: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        Self {
            capacity,
            data: Arc::new(RwLock::new(HashMap::new())),
            access_order: Arc::new(Mutex::new(Vec::new())),
            current_size: Arc::new(Mutex::new(0)),
            ttl,
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.write().unwrap();
        let mut stats = self.stats.lock().unwrap();

        if let Some(entry) = data.get_mut(key) {
            if entry.is_expired(self.ttl) {
                data.remove(key);
                self.remove_from_access_order(key);
                stats.misses += 1;
                None
            } else {
                let value = entry.access().clone();
                self.move_to_front(key);
                stats.hits += 1;
                Some(value)
            }
        } else {
            stats.misses += 1;
            None
        }
    }

    pub fn put(&self, key: K, value: V, size: usize) -> Result<(), ServiceError> {
        let mut data = self.data.write().unwrap();
        let mut current_size = self.current_size.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        // Check if we need to evict entries
        while *current_size + size > self.capacity && !data.is_empty() {
            if let Err(_) = self.evict_lru(&mut data, &mut *current_size, &mut stats) {
                return Err(ServiceError::CacheFull);
            }
        }

        if *current_size + size > self.capacity {
            return Err(ServiceError::CacheFull);
        }

        // Remove old entry if exists
        if let Some(old_entry) = data.remove(&key) {
            *current_size -= old_entry.size;
            self.remove_from_access_order(&key);
        }

        // Insert new entry
        let entry = CacheEntry::new(value, size);
        data.insert(key.clone(), entry);
        *current_size += size;
        stats.size = data.len();

        self.add_to_front(key);
        Ok(())
    }

    fn evict_lru(&self, data: &mut HashMap<K, CacheEntry<V>>, current_size: &mut usize, stats: &mut CacheStats) -> Result<(), ServiceError> {
        let access_order = self.access_order.lock().unwrap();
        if let Some(lru_key) = access_order.last() {
            if let Some(entry) = data.remove(lru_key) {
                *current_size -= entry.size;
                stats.evictions += 1;
                stats.size = data.len();
                drop(access_order);
                self.remove_from_access_order(lru_key);
                Ok(())
            } else {
                Err(ServiceError::CacheFull)
            }
        } else {
            Err(ServiceError::CacheFull)
        }
    }

    fn move_to_front(&self, key: &K) {
        let mut access_order = self.access_order.lock().unwrap();
        if let Some(pos) = access_order.iter().position(|x| x == key) {
            let key = access_order.remove(pos);
            access_order.insert(0, key);
        }
    }

    fn add_to_front(&self, key: K) {
        let mut access_order = self.access_order.lock().unwrap();
        access_order.insert(0, key);
    }

    fn remove_from_access_order(&self, key: &K) {
        let mut access_order = self.access_order.lock().unwrap();
        if let Some(pos) = access_order.iter().position(|x| x == key) {
            access_order.remove(pos);
        }
    }

    pub fn stats(&self) -> CacheStats {
        self.stats.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        let mut data = self.data.write().unwrap();
        let mut access_order = self.access_order.lock().unwrap();
        let mut current_size = self.current_size.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        data.clear();
        access_order.clear();
        *current_size = 0;
        stats.size = 0;
    }
}

/// Worker thread implementation
pub struct Worker {
    id: usize,
    config: ServiceConfig,
    cache: Arc<LruCache<String, ServiceResponse>>,
    request_queue: Arc<Mutex<Vec<ServiceRequest>>>,
    response_sender: Arc<Mutex<HashMap<String, ServiceResponse>>>,
    is_running: Arc<Mutex<bool>>,
    metrics: Arc<Mutex<WorkerMetrics>>,
}

#[derive(Debug, Default, Clone)]
pub struct WorkerMetrics {
    pub requests_processed: u64,
    pub errors_encountered: u64,
    pub average_processing_time: Duration,
    pub total_processing_time: Duration,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Worker {
    pub fn new(
        id: usize,
        config: ServiceConfig,
        cache: Arc<LruCache<String, ServiceResponse>>,
        request_queue: Arc<Mutex<Vec<ServiceRequest>>>,
        response_sender: Arc<Mutex<HashMap<String, ServiceResponse>>>,
    ) -> Self {
        Self {
            id,
            config,
            cache,
            request_queue,
            response_sender,
            is_running: Arc::new(Mutex::new(false)),
            metrics: Arc::new(Mutex::new(WorkerMetrics::default())),
        }
    }

    pub fn start(&self) -> Result<thread::JoinHandle<()>, ServiceError> {
        let id = self.id;
        let config = self.config.clone();
        let cache = Arc::clone(&self.cache);
        let request_queue = Arc::clone(&self.request_queue);
        let response_sender = Arc::clone(&self.response_sender);
        let is_running = Arc::clone(&self.is_running);
        let metrics = Arc::clone(&self.metrics);

        {
            let mut running = is_running.lock().unwrap();
            *running = true;
        }

        let handle = thread::spawn(move || {
            let mut local_metrics = WorkerMetrics::default();

            loop {
                let should_continue = {
                    let running = is_running.lock().unwrap();
                    *running
                };

                if !should_continue {
                    break;
                }

                // Get next request
                let request = {
                    let mut queue = request_queue.lock().unwrap();
                    queue.pop()
                };

                if let Some(req) = request {
                    let start_time = Instant::now();

                    // Check cache first
                    let cached_response = cache.get(&req.id);

                    let response = if let Some(cached) = cached_response {
                        local_metrics.cache_hits += 1;
                        cached
                    } else {
                        local_metrics.cache_misses += 1;

                        // Process request
                        match Self::process_request(&req, &config, id) {
                            Ok(resp) => {
                                // Cache the response
                                let response_size = Self::estimate_response_size(&resp);
                                if let Err(e) = cache.put(req.id.clone(), resp.clone(), response_size) {
                                    if config.enable_logging {
                                        eprintln!("Worker {}: Failed to cache response: {:?}", id, e);
                                    }
                                }
                                resp
                            }
                            Err(e) => {
                                local_metrics.errors_encountered += 1;
                                ServiceResponse {
                                    id: req.id.clone(),
                                    status: ResponseStatus::InternalError,
                                    data: Some(ResponseData::Text(format!("Error: {}", e))),
                                    headers: HashMap::new(),
                                    processing_time: start_time.elapsed(),
                                    worker_id: id,
                                }
                            }
                        }
                    };

                    let processing_time = start_time.elapsed();
                    local_metrics.requests_processed += 1;
                    local_metrics.total_processing_time += processing_time;
                    local_metrics.average_processing_time =
                        local_metrics.total_processing_time / local_metrics.requests_processed as u32;

                    // Send response
                    {
                        let mut sender = response_sender.lock().unwrap();
                        sender.insert(req.id.clone(), response);
                    }
                } else {
                    // No requests available, sleep briefly
                    thread::sleep(Duration::from_millis(10));
                }

                // Update metrics periodically
                if local_metrics.requests_processed % 100 == 0 {
                    let mut global_metrics = metrics.lock().unwrap();
                    *global_metrics = local_metrics.clone();
                }
            }

            // Final metrics update
            let mut global_metrics = metrics.lock().unwrap();
            *global_metrics = local_metrics;
        });

        Ok(handle)
    }

    pub fn stop(&self) {
        let mut running = self.is_running.lock().unwrap();
        *running = false;
    }

    pub fn metrics(&self) -> WorkerMetrics {
        self.metrics.lock().unwrap().clone()
    }

    fn process_request(
        request: &ServiceRequest,
        config: &ServiceConfig,
        worker_id: usize,
    ) -> Result<ServiceResponse, ServiceError> {
        // Simulate processing time based on request complexity
        let processing_delay = Self::calculate_processing_delay(&request);
        thread::sleep(processing_delay);

        // Validate request
        Self::validate_request(request)?;

        // Process based on method
        let data = match &request.method {
            RequestMethod::Get => Self::handle_get_request(request)?,
            RequestMethod::Post => Self::handle_post_request(request)?,
            RequestMethod::Put => Self::handle_put_request(request)?,
            RequestMethod::Delete => Self::handle_delete_request(request)?,
            RequestMethod::Patch => Self::handle_patch_request(request)?,
            RequestMethod::Head => None,
            RequestMethod::Options => Self::handle_options_request(request)?,
        };

        // Create response headers
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Worker-Id".to_string(), worker_id.to_string());
        headers.insert("X-Request-Id".to_string(), request.id.clone());
        headers.insert("X-Processing-Time".to_string(), processing_delay.as_millis().to_string());

        Ok(ServiceResponse {
            id: request.id.clone(),
            status: ResponseStatus::Success,
            data,
            headers,
            processing_time: processing_delay,
            worker_id,
        })
    }

    fn calculate_processing_delay(request: &ServiceRequest) -> Duration {
        let base_delay = Duration::from_millis(10);
        let priority_multiplier = match request.priority {
            Priority::Critical => 0.5,
            Priority::High => 0.75,
            Priority::Normal => 1.0,
            Priority::Low => 1.5,
        };

        let data_size_factor = match &request.data {
            RequestData::Json(value) => value.to_string().len() as f64 / 1000.0,
            RequestData::Binary(data) => data.len() as f64 / 1000.0,
            RequestData::Text(text) => text.len() as f64 / 1000.0,
            RequestData::FormData(form) => {
                form.values().map(|v| v.len()).sum::<usize>() as f64 / 1000.0
            }
            RequestData::Empty => 0.0,
        };

        let total_multiplier = priority_multiplier * (1.0 + data_size_factor);
        Duration::from_millis((base_delay.as_millis() as f64 * total_multiplier) as u64)
    }

    fn validate_request(request: &ServiceRequest) -> Result<(), ServiceError> {
        if request.id.is_empty() {
            return Err(ServiceError::InvalidInput("Request ID cannot be empty".to_string()));
        }

        if request.id.len() > 255 {
            return Err(ServiceError::InvalidInput("Request ID too long".to_string()));
        }

        // Validate data size
        let data_size = match &request.data {
            RequestData::Json(value) => value.to_string().len(),
            RequestData::Binary(data) => data.len(),
            RequestData::Text(text) => text.len(),
            RequestData::FormData(form) => {
                form.values().map(|v| v.len()).sum::<usize>()
            }
            RequestData::Empty => 0,
        };

        if data_size > 10_000_000 { // 10MB limit
            return Err(ServiceError::InvalidInput("Request data too large".to_string()));
        }

        Ok(())
    }

    fn handle_get_request(request: &ServiceRequest) -> Result<Option<ResponseData>, ServiceError> {
        // Simulate GET processing
        let response_data = serde_json::json!({
            "message": "GET request processed successfully",
            "request_id": request.id,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "headers": request.headers,
        });

        Ok(Some(ResponseData::Json(response_data)))
    }

    fn handle_post_request(request: &ServiceRequest) -> Result<Option<ResponseData>, ServiceError> {
        // Simulate POST processing with data validation
        let processed_data = match &request.data {
            RequestData::Json(value) => {
                // Simulate JSON processing
                let mut result = value.clone();
                if let Some(obj) = result.as_object_mut() {
                    obj.insert("processed".to_string(), serde_json::Value::Bool(true));
                    obj.insert("processor_id".to_string(), serde_json::Value::String(request.id.clone()));
                }
                result
            }
            RequestData::Text(text) => {
                serde_json::json!({
                    "original_text": text,
                    "processed": true,
                    "length": text.len(),
                    "word_count": text.split_whitespace().count(),
                })
            }
            RequestData::FormData(form) => {
                serde_json::json!({
                    "form_fields": form,
                    "field_count": form.len(),
                    "processed": true,
                })
            }
            RequestData::Binary(data) => {
                serde_json::json!({
                    "binary_size": data.len(),
                    "processed": true,
                    "checksum": format!("{:x}", md5::compute(data)),
                })
            }
            RequestData::Empty => {
                serde_json::json!({
                    "message": "Empty data processed",
                    "processed": true,
                })
            }
        };

        Ok(Some(ResponseData::Json(processed_data)))
    }

    fn handle_put_request(request: &ServiceRequest) -> Result<Option<ResponseData>, ServiceError> {
        // Simulate PUT processing (update operation)
        let response_data = serde_json::json!({
            "message": "Resource updated successfully",
            "request_id": request.id,
            "updated_at": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "data_size": match &request.data {
                RequestData::Json(v) => v.to_string().len(),
                RequestData::Binary(d) => d.len(),
                RequestData::Text(t) => t.len(),
                RequestData::FormData(f) => f.values().map(|v| v.len()).sum::<usize>(),
                RequestData::Empty => 0,
            },
        });

        Ok(Some(ResponseData::Json(response_data)))
    }

    fn handle_delete_request(request: &ServiceRequest) -> Result<Option<ResponseData>, ServiceError> {
        // Simulate DELETE processing
        let response_data = serde_json::json!({
            "message": "Resource deleted successfully",
            "request_id": request.id,
            "deleted_at": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });

        Ok(Some(ResponseData::Json(response_data)))
    }

    fn handle_patch_request(request: &ServiceRequest) -> Result<Option<ResponseData>, ServiceError> {
        // Simulate PATCH processing (partial update)
        let response_data = serde_json::json!({
            "message": "Resource partially updated",
            "request_id": request.id,
            "patched_at": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "patch_data": &request.data,
        });

        Ok(Some(ResponseData::Json(response_data)))
    }

    fn handle_options_request(_request: &ServiceRequest) -> Result<Option<ResponseData>, ServiceError> {
        // CORS preflight response
        Ok(Some(ResponseData::Json(serde_json::json!({
            "allowed_methods": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"],
            "allowed_headers": ["Content-Type", "Authorization", "X-Request-Id"],
            "max_age": 86400,
        }))))
    }

    fn estimate_response_size(response: &ServiceResponse) -> usize {
        let base_size = std::mem::size_of::<ServiceResponse>();
        let headers_size = response.headers.iter()
            .map(|(k, v)| k.len() + v.len())
            .sum::<usize>();

        let data_size = match &response.data {
            Some(ResponseData::Json(value)) => value.to_string().len(),
            Some(ResponseData::Binary(data)) => data.len(),
            Some(ResponseData::Text(text)) => text.len(),
            Some(ResponseData::Stream(url)) => url.len(),
            None => 0,
        };

        base_size + headers_size + data_size + response.id.len()
    }
}

/// Main service orchestrator
pub struct ComplexService {
    config: ServiceConfig,
    workers: Vec<Worker>,
    cache: Arc<LruCache<String, ServiceResponse>>,
    request_queue: Arc<Mutex<Vec<ServiceRequest>>>,
    response_store: Arc<Mutex<HashMap<String, ServiceResponse>>>,
    worker_handles: Vec<thread::JoinHandle<()>>,
    request_counter: Arc<Mutex<u64>>,
    service_metrics: Arc<Mutex<ServiceMetrics>>,
}

#[derive(Debug, Default, Clone)]
pub struct ServiceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: Duration,
    pub total_response_time: Duration,
    pub cache_hit_rate: f64,
    pub active_workers: usize,
    pub queue_size: usize,
}

impl ComplexService {
    pub fn new(config: ServiceConfig) -> Result<Self, ServiceError> {
        Self::validate_config(&config)?;

        let cache = Arc::new(LruCache::new(
            config.cache_size,
            Duration::from_secs(300), // 5 minute TTL
        ));

        let request_queue = Arc::new(Mutex::new(Vec::new()));
        let response_store = Arc::new(Mutex::new(HashMap::new()));

        let mut workers = Vec::new();
        for i in 0..config.worker_threads {
            let worker = Worker::new(
                i,
                config.clone(),
                Arc::clone(&cache),
                Arc::clone(&request_queue),
                Arc::clone(&response_store),
            );
            workers.push(worker);
        }

        Ok(Self {
            config,
            workers,
            cache,
            request_queue,
            response_store,
            worker_handles: Vec::new(),
            request_counter: Arc::new(Mutex::new(0)),
            service_metrics: Arc::new(Mutex::new(ServiceMetrics::default())),
        })
    }

    fn validate_config(config: &ServiceConfig) -> Result<(), ServiceError> {
        if config.max_connections == 0 {
            return Err(ServiceError::ConfigurationError(
                "max_connections must be greater than 0".to_string()
            ));
        }

        if config.worker_threads == 0 {
            return Err(ServiceError::ConfigurationError(
                "worker_threads must be greater than 0".to_string()
            ));
        }

        if config.cache_size == 0 {
            return Err(ServiceError::ConfigurationError(
                "cache_size must be greater than 0".to_string()
            ));
        }

        if config.timeout.as_secs() == 0 {
            return Err(ServiceError::ConfigurationError(
                "timeout must be greater than 0".to_string()
            ));
        }

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), ServiceError> {
        if !self.worker_handles.is_empty() {
            return Err(ServiceError::ConfigurationError(
                "Service is already running".to_string()
            ));
        }

        for worker in &self.workers {
            let handle = worker.start()?;
            self.worker_handles.push(handle);
        }

        if self.config.enable_logging {
            println!("ComplexService started with {} workers", self.config.worker_threads);
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), ServiceError> {
        // Stop all workers
        for worker in &self.workers {
            worker.stop();
        }

        // Wait for all worker threads to finish
        while let Some(handle) = self.worker_handles.pop() {
            if let Err(_) = handle.join() {
                return Err(ServiceError::WorkerPanic);
            }
        }

        if self.config.enable_logging {
            println!("ComplexService stopped");
        }

        Ok(())
    }

    pub fn submit_request(&self, mut request: ServiceRequest) -> Result<String, ServiceError> {
        // Generate request ID if not provided
        if request.id.is_empty() {
            let mut counter = self.request_counter.lock().unwrap();
            *counter += 1;
            request.id = format!("req_{:08x}", *counter);
        }

        // Validate request
        Self::validate_request_limits(&request, &self.config)?;

        // Add to queue
        {
            let mut queue = self.request_queue.lock().unwrap();
            if queue.len() >= self.config.max_connections {
                return Err(ServiceError::CacheFull); // Queue full, reusing error type
            }

            // Insert based on priority
            let insert_pos = queue.iter().position(|r| r.priority < request.priority)
                .unwrap_or(queue.len());
            queue.insert(insert_pos, request.clone());
        }

        // Update metrics
        {
            let mut metrics = self.service_metrics.lock().unwrap();
            metrics.total_requests += 1;
            metrics.queue_size = self.request_queue.lock().unwrap().len();
        }

        Ok(request.id)
    }

    fn validate_request_limits(request: &ServiceRequest, config: &ServiceConfig) -> Result<(), ServiceError> {
        // Implement rate limiting and validation logic here
        let data_size = match &request.data {
            RequestData::Json(value) => value.to_string().len(),
            RequestData::Binary(data) => data.len(),
            RequestData::Text(text) => text.len(),
            RequestData::FormData(form) => {
                form.values().map(|v| v.len()).sum::<usize>()
            }
            RequestData::Empty => 0,
        };

        // Example size limit: 50MB
        if data_size > 50_000_000 {
            return Err(ServiceError::InvalidInput(
                "Request data exceeds maximum size limit".to_string()
            ));
        }

        Ok(())
    }

    pub fn get_response(&self, request_id: &str) -> Option<ServiceResponse> {
        let mut store = self.response_store.lock().unwrap();
        store.remove(request_id)
    }

    pub fn wait_for_response(&self, request_id: &str, timeout: Duration) -> Result<ServiceResponse, ServiceError> {
        let start = Instant::now();

        loop {
            if let Some(response) = self.get_response(request_id) {
                // Update success metrics
                {
                    let mut metrics = self.service_metrics.lock().unwrap();
                    metrics.successful_requests += 1;
                    let response_time = start.elapsed();
                    metrics.total_response_time += response_time;
                    metrics.average_response_time =
                        metrics.total_response_time / metrics.successful_requests as u32;
                }
                return Ok(response);
            }

            if start.elapsed() > timeout {
                // Update failure metrics
                {
                    let mut metrics = self.service_metrics.lock().unwrap();
                    metrics.failed_requests += 1;
                }
                return Err(ServiceError::Timeout);
            }

            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn metrics(&self) -> ServiceMetrics {
        let mut metrics = self.service_metrics.lock().unwrap().clone();
        metrics.active_workers = self.worker_handles.len();
        metrics.queue_size = self.request_queue.lock().unwrap().len();

        let cache_stats = self.cache.stats();
        metrics.cache_hit_rate = cache_stats.hit_rate();

        metrics
    }

    pub fn worker_metrics(&self) -> Vec<WorkerMetrics> {
        self.workers.iter().map(|w| w.metrics()).collect()
    }

    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn health_check(&self) -> Result<serde_json::Value, ServiceError> {
        let metrics = self.metrics();
        let cache_stats = self.cache_stats();
        let worker_metrics = self.worker_metrics();

        Ok(serde_json::json!({
            "status": "healthy",
            "service_metrics": {
                "total_requests": metrics.total_requests,
                "successful_requests": metrics.successful_requests,
                "failed_requests": metrics.failed_requests,
                "average_response_time_ms": metrics.average_response_time.as_millis(),
                "cache_hit_rate": metrics.cache_hit_rate,
                "active_workers": metrics.active_workers,
                "queue_size": metrics.queue_size,
            },
            "cache_stats": {
                "hits": cache_stats.hits,
                "misses": cache_stats.misses,
                "evictions": cache_stats.evictions,
                "size": cache_stats.size,
                "capacity": cache_stats.capacity,
                "hit_rate": cache_stats.hit_rate(),
            },
            "worker_metrics": worker_metrics,
            "config": {
                "max_connections": self.config.max_connections,
                "timeout_secs": self.config.timeout.as_secs(),
                "retry_count": self.config.retry_count,
                "cache_size": self.config.cache_size,
                "worker_threads": self.config.worker_threads,
            }
        }))
    }
}
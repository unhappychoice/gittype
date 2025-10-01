/*
 * Complex Rust file with various comment patterns
 * This file tests comment range detection and code parsing
 * It includes multiple comment styles and edge cases
 */

/// This is a documentation comment for the module
/// It spans multiple lines and contains examples:
///
/// ```rust
/// let example = ComplexStruct::new();
/// example.process_data(&data);
/// ```
///
/// # Safety
///
/// This module contains unsafe code blocks for performance reasons.
/// Users should be careful when calling these functions.

use std::collections::{HashMap, BTreeMap}; // Standard collections
use std::sync::{Arc, Mutex}; /* Thread-safe primitives */
use std::time::{Duration, Instant}; // Time utilities
/* Multi-line import comment
   with detailed explanations */
use std::thread;

// Constants with inline comments
const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1MB buffer
const DEFAULT_TIMEOUT: u64 = 30; /* 30 seconds default timeout
                                    can be overridden by configuration */
const VERSION: &str = "1.0.0"; // Application version

/// Configuration structure with extensive documentation
///
/// This struct holds all configuration parameters for the application.
/// Each field has specific constraints and default values.
///
/// # Examples
///
/// ```rust
/// let config = Config {
///     max_connections: 100,
///     timeout: Duration::from_secs(30),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum number of concurrent connections
    /// Must be between 1 and 10000
    pub max_connections: usize,

    /* Timeout for network operations
       Set to 0 for no timeout */
    pub timeout: Duration,

    // Enable debug logging
    pub debug_mode: bool, /* This affects performance
                             only enable for development */

    /// Custom headers for HTTP requests
    pub headers: HashMap<String, String>, // Key-value pairs
}

impl Default for Config {
    /// Creates a default configuration
    ///
    /// All values are set to safe defaults that work
    /// in most environments.
    fn default() -> Self {
        Self {
            max_connections: 10, // Conservative default
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            debug_mode: false, /* Disabled by default for performance */
            headers: HashMap::new(), // Empty headers
        }
    }
}

/**
 * Error enumeration with various patterns
 *
 * This enum covers all possible error conditions
 * that can occur in the application.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum AppError {
    /// Network connectivity issues
    NetworkError {
        code: u16,              // HTTP status code
        message: String,        /* Error message from server */
        retry_after: Option<u64>, // Seconds to wait before retry
    },

    /* Configuration validation errors */
    ConfigError(String), // Error message

    // Timeout occurred during operation
    TimeoutError, /* No additional data needed */

    /// Parse errors with location information
    ParseError {
        line: usize,    // Line number where error occurred
        column: usize,  /* Column position */
        details: String, // Detailed error description
    },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Simple error formatting
            AppError::NetworkError { code, message, .. } => {
                write!(f, "Network error {}: {}", code, message)
            },
            AppError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            AppError::TimeoutError => write!(f, "Operation timed out"),
            /* Complex error with location info */
            AppError::ParseError { line, column, details } => {
                write!(f, "Parse error at {}:{}: {}", line, column, details)
            },
        }
    }
}

/* Implementation of std::error::Error trait
   Required for proper error handling */
impl std::error::Error for AppError {}

/// Data processing pipeline with complex logic
///
/// This struct implements a multi-stage data processing pipeline
/// with support for parallel execution and error recovery.
///
/// # Type Parameters
///
/// * `T` - The input data type
/// * `R` - The output result type
///
/// # Examples
///
/// ```rust
/// let mut pipeline = ProcessingPipeline::new();
/// pipeline.add_stage(Box::new(|data| data.to_uppercase()));
/// let result = pipeline.process("hello world").await?;
/// assert_eq!(result, "HELLO WORLD");
/// ```
pub struct ProcessingPipeline<T, R> {
    // Vector of processing stages
    stages: Vec<Box<dyn Fn(T) -> Result<T, AppError> + Send + Sync>>,

    /// Maximum number of concurrent operations
    max_concurrency: usize, /* Limited to prevent resource exhaustion */

    // Statistics and metrics
    metrics: Arc<Mutex<PipelineMetrics>>, /* Thread-safe metrics collection */

    /// Configuration for the pipeline
    config: Config, // Reuse the config struct
}

/// Metrics collected during pipeline execution
///
/// These metrics help monitor performance and identify bottlenecks.
#[derive(Debug, Default, Clone)]
pub struct PipelineMetrics {
    /// Total number of items processed
    total_processed: u64, // Counter

    /* Number of failed operations */
    total_failed: u64,

    // Average processing time per item
    avg_processing_time: Duration, /* Calculated automatically */

    /// Peak memory usage during processing
    peak_memory_usage: usize, // Bytes
}

impl<T, R> ProcessingPipeline<T, R>
where
    T: Send + Sync + Clone + 'static,
    R: Send + Sync + 'static,
{
    /// Creates a new processing pipeline
    ///
    /// # Arguments
    ///
    /// * `max_concurrency` - Maximum parallel operations
    ///
    /// # Returns
    ///
    /// A new pipeline instance ready for configuration
    pub fn new(max_concurrency: usize) -> Self {
        Self {
            stages: Vec::new(), // Empty initially
            max_concurrency, /* Store the limit */
            metrics: Arc::new(Mutex::new(PipelineMetrics::default())),
            config: Config::default(), // Use default configuration
        }
    }

    /// Adds a processing stage to the pipeline
    ///
    /// Stages are executed in the order they are added.
    /// Each stage receives the output of the previous stage.
    ///
    /// # Arguments
    ///
    /// * `stage` - A function that processes data
    ///
    /// # Examples
    ///
    /// ```rust
    /// pipeline.add_stage(Box::new(|data: String| {
    ///     Ok(data.trim().to_string())
    /// }));
    /// ```
    pub fn add_stage<F>(&mut self, stage: F)
    where
        F: Fn(T) -> Result<T, AppError> + Send + Sync + 'static
    {
        self.stages.push(Box::new(stage)); // Box the closure
    }

    /**
     * Processes data through all stages
     *
     * This method executes all configured stages in sequence,
     * passing the output of each stage to the next.
     *
     * @param data The input data to process
     * @return The processed result or an error
     */
    pub async fn process(&self, mut data: T) -> Result<T, AppError> {
        let start_time = Instant::now(); // Track processing time

        // Process through each stage
        for (index, stage) in self.stages.iter().enumerate() {
            match stage(data.clone()) {
                Ok(result) => {
                    data = result; // Update for next stage
                },
                Err(e) => {
                    /* Log the error with stage information */
                    eprintln!("Stage {} failed: {}", index, e);

                    // Update failure metrics
                    if let Ok(mut metrics) = self.metrics.lock() {
                        metrics.total_failed += 1;
                    }

                    return Err(e); // Propagate the error
                },
            }
        }

        // Update success metrics
        let processing_time = start_time.elapsed();
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.total_processed += 1; /* Increment counter */

            // Update average processing time
            let total_items = metrics.total_processed + metrics.total_failed;
            if total_items > 0 {
                let total_time = metrics.avg_processing_time * (total_items - 1) as u32
                    + processing_time;
                metrics.avg_processing_time = total_time / total_items as u32;
            }
        }

        Ok(data) // Return processed result
    }

    /// Processes multiple items in parallel
    ///
    /// This method takes advantage of multiple CPU cores
    /// to process data items concurrently.
    ///
    /// # Arguments
    ///
    /// * `items` - Vector of items to process
    ///
    /// # Returns
    ///
    /// Vector of results in the same order as input
    ///
    /// # Performance Notes
    ///
    /// The actual concurrency is limited by `max_concurrency`
    /// to prevent resource exhaustion.
    pub async fn process_batch(&self, items: Vec<T>) -> Vec<Result<T, AppError>> {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let results = Arc::new(Mutex::new(Vec::with_capacity(items.len())));
        let completed = Arc::new(AtomicUsize::new(0));
        let total_items = items.len();

        /* Process items in chunks to limit concurrency */
        let chunk_size = (total_items / self.max_concurrency).max(1);
        let chunks: Vec<_> = items.chunks(chunk_size).collect();

        // Spawn tasks for each chunk
        let mut handles = Vec::new();

        for (chunk_index, chunk) in chunks.into_iter().enumerate() {
            let chunk_data = chunk.to_vec(); // Clone the chunk
            let pipeline_stages = self.stages.clone(); /* Can't clone directly */
            let results_ref = Arc::clone(&results);
            let completed_ref = Arc::clone(&completed);

            let handle = tokio::spawn(async move {
                let mut chunk_results = Vec::new();

                // Process each item in the chunk
                for item in chunk_data {
                    let mut current_data = item;
                    let mut success = true;

                    /* Execute all stages for this item */
                    for (stage_index, _stage) in pipeline_stages.iter().enumerate() {
                        // Note: Can't actually call the stage here due to borrow checker
                        // This is a simplified example

                        /* Simulate stage processing
                           In real implementation, we'd need a different approach */
                        if stage_index % 7 == 0 && chunk_index % 3 == 0 {
                            // Simulate occasional failures
                            chunk_results.push(Err(AppError::TimeoutError));
                            success = false;
                            break;
                        }
                    }

                    if success {
                        chunk_results.push(Ok(current_data)); // Success case
                    }
                }

                // Store results
                {
                    let mut results_guard = results_ref.lock().unwrap();
                    results_guard.extend(chunk_results);
                }

                // Update completion counter
                completed_ref.fetch_add(chunk_data.len(), Ordering::Relaxed);
            });

            handles.push(handle);
        }

        // Wait for all chunks to complete
        for handle in handles {
            let _ = handle.await; /* Ignore join errors for simplicity */
        }

        // Extract and return results
        let results_guard = results.lock().unwrap();
        results_guard.clone() // Return the collected results
    }

    /// Gets current pipeline metrics
    ///
    /// Returns a snapshot of the current performance metrics.
    /// This data can be used for monitoring and optimization.
    pub fn get_metrics(&self) -> PipelineMetrics {
        self.metrics.lock().unwrap().clone() // Return a copy
    }

    /**
     * Resets all metrics to their initial state
     *
     * This is useful for starting fresh measurements
     * or clearing historical data.
     */
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        *metrics = PipelineMetrics::default(); /* Reset to defaults */
    }
}

/// Unsafe operations for performance-critical code
///
/// This module contains unsafe operations that bypass
/// Rust's safety checks for maximum performance.
///
/// # Safety
///
/// All functions in this module require careful review
/// and should only be used when performance is critical.
pub mod unsafe_operations {
    use super::*;

    /// Raw memory manipulation for zero-copy operations
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The pointer is valid and properly aligned
    /// - The data lives at least as long as the operation
    /// - No other code modifies the memory concurrently
    ///
    /// # Arguments
    ///
    /// * `ptr` - Raw pointer to data
    /// * `len` - Length of data in bytes
    ///
    /// # Returns
    ///
    /// Checksum of the data
    pub unsafe fn fast_checksum(ptr: *const u8, len: usize) -> u32 {
        let mut checksum = 0u32; // Initialize accumulator
        let mut i = 0;

        /* Process 4 bytes at a time for efficiency */
        while i + 4 <= len {
            let chunk = ptr.add(i) as *const u32;
            checksum = checksum.wrapping_add(*chunk); // Add without overflow check
            i += 4; // Move to next chunk
        }

        // Handle remaining bytes
        while i < len {
            let byte = *ptr.add(i); /* Read single byte */
            checksum = checksum.wrapping_add(byte as u32);
            i += 1; // Next byte
        }

        checksum // Return final result
    }

    /// Direct memory copy without bounds checking
    ///
    /// This function performs a raw memory copy operation
    /// without any safety checks for maximum speed.
    ///
    /// # Safety
    ///
    /// Extremely dangerous! The caller must guarantee:
    /// - Both pointers are valid and non-null
    /// - Source and destination don't overlap
    /// - Both regions have at least `len` bytes
    /// - Proper alignment for the data type
    ///
    /// # Performance
    ///
    /// This is faster than `std::ptr::copy` because it
    /// skips all safety checks and uses optimized assembly.
    pub unsafe fn raw_copy(src: *const u8, dst: *mut u8, len: usize) {
        // Use platform-specific optimized copy
        #[cfg(target_arch = "x86_64")]
        {
            /* x86_64 optimized version using SIMD instructions */
            let mut i = 0;

            // Process 32 bytes at a time with AVX if available
            while i + 32 <= len {
                let src_chunk = src.add(i) as *const [u8; 32];
                let dst_chunk = dst.add(i) as *mut [u8; 32];
                *dst_chunk = *src_chunk; // Bulk copy
                i += 32;
            }

            // Handle remaining bytes
            while i < len {
                *dst.add(i) = *src.add(i); /* Byte-by-byte copy */
                i += 1;
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            // Generic fallback for other architectures
            std::ptr::copy_nonoverlapping(src, dst, len);
        }
    }

    /**
     * Lock-free atomic operations for high-performance counters
     *
     * This structure provides thread-safe counters without
     * the overhead of mutex locking.
     */
    pub struct LockFreeCounter {
        value: std::sync::atomic::AtomicU64, // Atomic counter
    }

    impl LockFreeCounter {
        /// Creates a new counter starting at zero
        pub fn new() -> Self {
            Self {
                value: std::sync::atomic::AtomicU64::new(0),
            }
        }

        /// Increments the counter and returns the previous value
        ///
        /// This operation is atomic and lock-free.
        pub fn increment(&self) -> u64 {
            self.value.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }

        /* Gets the current counter value */
        pub fn get(&self) -> u64 {
            self.value.load(std::sync::atomic::Ordering::Relaxed)
        }

        /// Resets the counter to zero
        pub fn reset(&self) -> u64 {
            self.value.swap(0, std::sync::atomic::Ordering::Relaxed) /* Return old value */
        }
    }
}

/// Complex macro definitions for code generation
///
/// These macros generate repetitive code patterns
/// and provide convenient APIs for common operations.

/// Generates a builder pattern for any struct
///
/// # Example
///
/// ```rust
/// generate_builder!(MyStruct {
///     field1: String,
///     field2: u32,
/// });
/// ```
#[macro_export]
macro_rules! generate_builder {
    ($struct_name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        paste::paste! {
            /// Builder for $struct_name
            #[derive(Default)]
            pub struct [<$struct_name Builder>] {
                $(
                    $field: Option<$field_type>, // Optional field
                )*
            }

            impl [<$struct_name Builder>] {
                /// Creates a new builder instance
                pub fn new() -> Self {
                    Self::default()
                }

                $(
                    /// Sets the $field field
                    pub fn $field(mut self, value: $field_type) -> Self {
                        self.$field = Some(value); /* Store the value */
                        self // Return self for chaining
                    }
                )*

                /// Builds the final struct
                ///
                /// # Panics
                ///
                /// Panics if any required field is not set.
                pub fn build(self) -> $struct_name {
                    $struct_name {
                        $(
                            $field: self.$field.expect(
                                concat!("Field '", stringify!($field), "' is required")
                            ),
                        )*
                    }
                }
            }

            impl $struct_name {
                /// Creates a new builder for this struct
                pub fn builder() -> [<$struct_name Builder>] {
                    [<$struct_name Builder>]::new()
                }
            }
        }
    };
}

/// Generates error handling boilerplate
///
/// This macro creates From implementations for converting
/// between different error types.
macro_rules! impl_error_conversions {
    ($error_type:ty, { $($from_type:ty => $variant:path),* $(,)? }) => {
        $(
            impl From<$from_type> for $error_type {
                fn from(err: $from_type) -> Self {
                    $variant(err.to_string()) /* Convert to string */
                }
            }
        )*
    };
}

// Apply the error conversion macro
impl_error_conversions!(AppError, {
    std::io::Error => AppError::ConfigError,
    std::num::ParseIntError => AppError::ParseError {
        line: 0,
        column: 0,
        details: "Parse error".to_string()
    },
});

/// Test module with comprehensive test cases
///
/// This module contains unit tests and integration tests
/// for all the functionality defined above.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test the basic configuration functionality
    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.max_connections, 10); // Check default value
        assert_eq!(config.timeout.as_secs(), DEFAULT_TIMEOUT);
        assert!(!config.debug_mode); /* Should be false by default */
        assert!(config.headers.is_empty()); // No headers by default
    }

    /* Test error formatting and display */
    #[test]
    fn test_error_display() {
        let network_error = AppError::NetworkError {
            code: 404,
            message: "Not Found".to_string(),
            retry_after: Some(60),
        };

        let error_string = network_error.to_string();
        assert!(error_string.contains("404")); // Should contain status code
        assert!(error_string.contains("Not Found")); /* Should contain message */
    }

    /// Test the processing pipeline with simple operations
    #[tokio::test]
    async fn test_pipeline_basic() {
        let mut pipeline = ProcessingPipeline::<String, String>::new(2);

        // Add stages that transform the data
        pipeline.add_stage(Box::new(|data: String| {
            Ok(data.to_uppercase()) /* Convert to uppercase */
        }));

        pipeline.add_stage(Box::new(|data: String| {
            Ok(format!("Processed: {}", data)) // Add prefix
        }));

        let result = pipeline.process("hello world".to_string()).await;
        assert!(result.is_ok());

        let processed = result.unwrap();
        assert_eq!(processed, "Processed: HELLO WORLD"); /* Expected result */
    }

    /**
     * Test pipeline error handling
     *
     * This test verifies that errors in pipeline stages
     * are properly propagated and handled.
     */
    #[tokio::test]
    async fn test_pipeline_error_handling() {
        let mut pipeline = ProcessingPipeline::<String, String>::new(1);

        // Add a stage that always fails
        pipeline.add_stage(Box::new(|_data: String| {
            Err(AppError::TimeoutError) /* Simulate failure */
        }));

        let result = pipeline.process("test".to_string()).await;
        assert!(result.is_err()); // Should fail

        let error = result.unwrap_err();
        matches!(error, AppError::TimeoutError); /* Should be timeout error */
    }

    /// Test unsafe operations (with careful safety considerations)
    #[test]
    fn test_unsafe_checksum() {
        let data = b"Hello, world!"; // Test data
        let checksum = unsafe {
            unsafe_operations::fast_checksum(
                data.as_ptr(),
                data.len()
            )
        };

        // Verify checksum is calculated correctly
        assert_ne!(checksum, 0); /* Should not be zero for this data */

        // Test with empty data
        let empty_checksum = unsafe {
            unsafe_operations::fast_checksum(
                std::ptr::null(), /* Null pointer */
                0 /* Zero length */
            )
        };
        assert_eq!(empty_checksum, 0); // Should be zero for empty data
    }

    /* Test lock-free counter operations */
    #[test]
    fn test_lock_free_counter() {
        let counter = unsafe_operations::LockFreeCounter::new();

        assert_eq!(counter.get(), 0); // Should start at zero

        let old_value = counter.increment();
        assert_eq!(old_value, 0); /* Previous value should be 0 */
        assert_eq!(counter.get(), 1); // New value should be 1

        let reset_value = counter.reset();
        assert_eq!(reset_value, 1); // Should return the previous value
        assert_eq!(counter.get(), 0); /* Should be back to zero */
    }

    /// Benchmark test for performance measurement
    #[test]
    fn test_performance_benchmark() {
        let iterations = 1_000_000;
        let start = Instant::now();

        // Simulate some work
        let mut sum = 0u64;
        for i in 0..iterations {
            sum = sum.wrapping_add(i); /* Prevent overflow */
        }

        let duration = start.elapsed();
        println!("Benchmark completed in {:?}", duration);

        // Verify the computation was not optimized away
        assert_ne!(sum, 0); // Sum should not be zero
    }

    /**
     * Integration test that combines multiple components
     *
     * This test verifies that different parts of the system
     * work together correctly.
     */
    #[tokio::test]
    async fn test_integration() {
        // Create a complex pipeline
        let mut pipeline = ProcessingPipeline::<String, String>::new(4);

        // Add multiple processing stages
        pipeline.add_stage(Box::new(|data: String| {
            if data.is_empty() {
                Err(AppError::ConfigError("Empty input".to_string()))
            } else {
                Ok(data.trim().to_string()) /* Remove whitespace */
            }
        }));

        pipeline.add_stage(Box::new(|data: String| {
            Ok(data.replace(" ", "_")) // Replace spaces with underscores
        }));

        pipeline.add_stage(Box::new(|data: String| {
            Ok(format!("final_{}", data.to_lowercase())) /* Add prefix and lowercase */
        }));

        // Test with valid input
        let result = pipeline.process("  Hello World  ".to_string()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "final_hello_world");

        // Test with invalid input
        let error_result = pipeline.process("".to_string()).await;
        assert!(error_result.is_err()); /* Should fail for empty input */

        // Check metrics
        let metrics = pipeline.get_metrics();
        assert_eq!(metrics.total_processed, 1); // One successful operation
        assert_eq!(metrics.total_failed, 1); /* One failed operation */
    }
}
/// Performance monitoring and optimization utilities

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Simple performance timer for measuring function execution time
pub struct PerfTimer {
    start: Instant,
    label: String,
}

impl PerfTimer {
    /// Start a new performance timer with a label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            label: label.into(),
        }
    }

    /// Stop the timer and return elapsed duration
    pub fn stop(self) -> Duration {
        self.start.elapsed()
    }

    /// Stop the timer and print elapsed time (only in debug builds)
    #[cfg(debug_assertions)]
    pub fn stop_and_print(self) {
        let elapsed = self.start.elapsed();
        println!("[PERF] {}: {:.2}ms", self.label, elapsed.as_secs_f64() * 1000.0);
    }

    #[cfg(not(debug_assertions))]
    pub fn stop_and_print(self) {
        // No-op in release builds
        let _ = self.start.elapsed();
    }
}

/// Performance statistics tracker
pub struct PerfStats {
    timings: HashMap<String, Vec<Duration>>,
}

impl PerfStats {
    /// Create a new performance statistics tracker
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
        }
    }

    /// Record a timing measurement
    pub fn record(&mut self, label: impl Into<String>, duration: Duration) {
        self.timings
            .entry(label.into())
            .or_insert_with(Vec::new)
            .push(duration);
    }

    /// Get average duration for a label
    pub fn average(&self, label: &str) -> Option<Duration> {
        let times = self.timings.get(label)?;
        if times.is_empty() {
            return None;
        }
        let total: Duration = times.iter().sum();
        Some(total / times.len() as u32)
    }

    /// Get maximum duration for a label
    pub fn max(&self, label: &str) -> Option<Duration> {
        self.timings.get(label)?.iter().max().copied()
    }

    /// Get minimum duration for a label
    pub fn min(&self, label: &str) -> Option<Duration> {
        self.timings.get(label)?.iter().min().copied()
    }

    /// Clear all recorded timings
    pub fn clear(&mut self) {
        self.timings.clear();
    }

    /// Get all labels with statistics
    pub fn labels(&self) -> Vec<&String> {
        self.timings.keys().collect()
    }

    /// Print summary of all timings
    #[cfg(debug_assertions)]
    pub fn print_summary(&self) {
        println!("\n=== Performance Summary ===");
        for label in self.labels() {
            if let Some(avg) = self.average(label) {
                let min = self.min(label).unwrap();
                let max = self.max(label).unwrap();
                println!(
                    "{:30} avg: {:7.2}ms  min: {:7.2}ms  max: {:7.2}ms",
                    label,
                    avg.as_secs_f64() * 1000.0,
                    min.as_secs_f64() * 1000.0,
                    max.as_secs_f64() * 1000.0
                );
            }
        }
        println!("==========================\n");
    }

    #[cfg(not(debug_assertions))]
    pub fn print_summary(&self) {
        // No-op in release builds
    }
}

impl Default for PerfStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for timing a block of code
#[macro_export]
macro_rules! time_block {
    ($label:expr, $block:expr) => {{
        let _timer = $crate::perf::PerfTimer::new($label);
        let result = $block;
        _timer.stop_and_print();
        result
    }};
}

/// Cache for expensive computations
pub struct ComputeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    cache: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> ComputeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new compute cache with a maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get a value from cache or compute it
    pub fn get_or_compute<F>(&mut self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            return value.clone();
        }

        let value = compute_fn(&key);

        // Simple eviction: clear cache if it gets too large
        if self.cache.len() >= self.max_size {
            self.cache.clear();
        }

        self.cache.insert(key.clone(), value.clone());
        value
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get current cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_perf_timer() {
        let timer = PerfTimer::new("test");
        thread::sleep(Duration::from_millis(10));
        let elapsed = timer.stop();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_perf_stats_recording() {
        let mut stats = PerfStats::new();
        stats.record("test1", Duration::from_millis(10));
        stats.record("test1", Duration::from_millis(20));
        stats.record("test2", Duration::from_millis(5));

        assert_eq!(stats.labels().len(), 2);
        assert!(stats.average("test1").is_some());
        assert!(stats.average("test2").is_some());
    }

    #[test]
    fn test_perf_stats_average() {
        let mut stats = PerfStats::new();
        stats.record("test", Duration::from_millis(10));
        stats.record("test", Duration::from_millis(20));
        stats.record("test", Duration::from_millis(30));

        let avg = stats.average("test").unwrap();
        assert_eq!(avg, Duration::from_millis(20));
    }

    #[test]
    fn test_perf_stats_min_max() {
        let mut stats = PerfStats::new();
        stats.record("test", Duration::from_millis(10));
        stats.record("test", Duration::from_millis(20));
        stats.record("test", Duration::from_millis(30));

        assert_eq!(stats.min("test"), Some(Duration::from_millis(10)));
        assert_eq!(stats.max("test"), Some(Duration::from_millis(30)));
    }

    #[test]
    fn test_perf_stats_clear() {
        let mut stats = PerfStats::new();
        stats.record("test", Duration::from_millis(10));
        assert_eq!(stats.labels().len(), 1);

        stats.clear();
        assert_eq!(stats.labels().len(), 0);
    }

    #[test]
    fn test_compute_cache() {
        let mut cache = ComputeCache::new(10);

        // First call should compute
        let result1 = cache.get_or_compute(5, |&x| x * 2);
        assert_eq!(result1, 10);
        assert_eq!(cache.len(), 1);

        // Second call should use cache
        let result2 = cache.get_or_compute(5, |&x| x * 2);
        assert_eq!(result2, 10);
        assert_eq!(cache.len(), 1);

        // Different key should compute
        let result3 = cache.get_or_compute(7, |&x| x * 2);
        assert_eq!(result3, 14);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_compute_cache_eviction() {
        let mut cache = ComputeCache::new(3);

        cache.get_or_compute(1, |&x| x * 2);
        cache.get_or_compute(2, |&x| x * 2);
        cache.get_or_compute(3, |&x| x * 2);
        assert_eq!(cache.len(), 3);

        // Adding one more should trigger eviction
        cache.get_or_compute(4, |&x| x * 2);
        assert!(cache.len() <= 3, "Cache should be cleared or evicted");
    }

    #[test]
    fn test_compute_cache_clear() {
        let mut cache = ComputeCache::new(10);
        cache.get_or_compute(1, |&x| x * 2);
        cache.get_or_compute(2, |&x| x * 2);
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
}

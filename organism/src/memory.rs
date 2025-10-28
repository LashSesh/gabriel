//! Mathematical Memory System
//!
//! Long-term storage and retrieval of mathematical knowledge:
//! - Theorem storage with efficient indexing
//! - Proof archival
//! - Associative recall based on similarity
//! - Consolidation of related concepts

use gabriel_core::InformationQuantum;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use ahash::AHashMap;
use tracing::debug;

/// Memory entry for a mathematical concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry<T: Clone> {
    pub id: u64,
    pub content: T,
    pub timestamp: f64,
    /// How many times accessed
    pub access_count: usize,
    /// Last access time
    pub last_access: f64,
    /// Importance score (0.0-1.0)
    pub importance: f64,
    /// Tags for indexing
    pub tags: Vec<String>,
}

impl<T: Clone> MemoryEntry<T> {
    pub fn new(id: u64, content: T, timestamp: f64) -> Self {
        Self {
            id,
            content,
            timestamp,
            access_count: 0,
            last_access: timestamp,
            importance: 0.5,
            tags: Vec::new(),
        }
    }

    pub fn access(&mut self, current_time: f64) {
        self.access_count += 1;
        self.last_access = current_time;

        // Increase importance with access (up to limit)
        self.importance = (self.importance + 0.05).min(1.0);
    }

    pub fn decay(&mut self, current_time: f64, decay_rate: f64) {
        let time_since_access = current_time - self.last_access;
        self.importance *= (-decay_rate * time_since_access).exp();
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum number of entries
    pub max_entries: usize,

    /// Decay rate for importance
    pub decay_rate: f64,

    /// Threshold for forgetting (remove if importance < threshold)
    pub forget_threshold: f64,

    /// Consolidation interval (how often to merge similar memories)
    pub consolidation_interval: f64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            decay_rate: 0.01,
            forget_threshold: 0.1,
            consolidation_interval: 100.0,
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: usize,
    pub total_accesses: u64,
    pub average_importance: f64,
    pub total_forgotten: u64,
    pub consolidation_count: u64,
}

/// Long-term memory system
pub struct MathematicalMemory<T: Clone + Send + Sync + 'static> {
    /// Storage: ID -> Entry
    storage: Arc<RwLock<AHashMap<u64, MemoryEntry<T>>>>,

    /// Tag index: Tag -> Vec<ID>
    tag_index: Arc<RwLock<AHashMap<String, Vec<u64>>>>,

    config: MemoryConfig,

    stats: Arc<RwLock<MemoryStats>>,

    /// Current time
    current_time: Arc<RwLock<f64>>,
}

impl<T: Clone + Send + Sync + 'static> MathematicalMemory<T> {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            storage: Arc::new(RwLock::new(AHashMap::new())),
            tag_index: Arc::new(RwLock::new(AHashMap::new())),
            config,
            stats: Arc::new(RwLock::new(MemoryStats {
                total_entries: 0,
                total_accesses: 0,
                average_importance: 0.5,
                total_forgotten: 0,
                consolidation_count: 0,
            })),
            current_time: Arc::new(RwLock::new(0.0)),
        }
    }

    /// Store a new memory
    pub fn store(&self, entry: MemoryEntry<T>) {
        let id = entry.id;
        let tags = entry.tags.clone();

        // Store entry
        {
            let mut storage = self.storage.write();

            // Check capacity
            if storage.len() >= self.config.max_entries {
                self.forget_least_important();
            }

            storage.insert(id, entry);
        }

        // Update tag index
        {
            let mut tag_index = self.tag_index.write();
            for tag in tags {
                tag_index.entry(tag).or_insert_with(Vec::new).push(id);
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_entries += 1;
        }

        debug!("Stored memory entry {}", id);
    }

    /// Retrieve a memory by ID
    pub fn recall(&self, id: u64) -> Option<T> {
        let current_time = *self.current_time.read();

        let mut storage = self.storage.write();
        if let Some(entry) = storage.get_mut(&id) {
            entry.access(current_time);

            let mut stats = self.stats.write();
            stats.total_accesses += 1;

            Some(entry.content.clone())
        } else {
            None
        }
    }

    /// Search by tag
    pub fn recall_by_tag(&self, tag: &str) -> Vec<T> {
        let tag_index = self.tag_index.read();

        if let Some(ids) = tag_index.get(tag) {
            let current_time = *self.current_time.read();
            let mut storage = self.storage.write();

            ids.iter()
                .filter_map(|id| {
                    storage.get_mut(id).map(|entry| {
                        entry.access(current_time);
                        entry.content.clone()
                    })
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all memories sorted by importance
    pub fn recall_top(&self, limit: usize) -> Vec<T> {
        let current_time = *self.current_time.read();
        let mut storage = self.storage.write();

        let mut entries: Vec<_> = storage.values_mut().collect();
        entries.sort_by(|a, b| {
            b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal)
        });

        entries.into_iter()
            .take(limit)
            .map(|entry| {
                entry.access(current_time);
                entry.content.clone()
            })
            .collect()
    }

    /// Forget least important entry
    fn forget_least_important(&self) {
        let mut storage = self.storage.write();

        if let Some((&id_to_remove, _)) = storage.iter()
            .min_by(|a, b| {
                a.1.importance.partial_cmp(&b.1.importance).unwrap_or(std::cmp::Ordering::Equal)
            })
        {
            storage.remove(&id_to_remove);

            let mut stats = self.stats.write();
            stats.total_forgotten += 1;

            debug!("Forgot entry {} (low importance)", id_to_remove);
        }
    }

    /// Decay all memories
    pub fn decay_all(&self) {
        let current_time = *self.current_time.read();
        let mut storage = self.storage.write();

        for entry in storage.values_mut() {
            entry.decay(current_time, self.config.decay_rate);
        }

        // Remove entries below threshold
        let forget_threshold = self.config.forget_threshold;
        storage.retain(|_, entry| entry.importance >= forget_threshold);

        // Update average importance
        if !storage.is_empty() {
            let total_importance: f64 = storage.values().map(|e| e.importance).sum();
            let avg = total_importance / storage.len() as f64;

            let mut stats = self.stats.write();
            stats.average_importance = avg;
            stats.total_entries = storage.len();
        }
    }

    /// Advance time
    pub fn tick(&self, dt: f64) {
        let mut time = self.current_time.write();
        *time += dt;
    }

    /// Get statistics
    pub fn stats(&self) -> MemoryStats {
        self.stats.read().clone()
    }

    /// Get current memory count
    pub fn count(&self) -> usize {
        self.storage.read().len()
    }

    /// Clear all memories
    pub fn clear(&self) {
        self.storage.write().clear();
        self.tag_index.write().clear();

        let mut stats = self.stats.write();
        stats.total_entries = 0;
    }
}

/// Specialized memory for theorems with similarity-based recall
pub struct TheoremMemory<Q: InformationQuantum> {
    memory: MathematicalMemory<Q>,
    /// Quantum reference for similarity search
    quanta: Arc<RwLock<Vec<Q>>>,
}

impl<Q: InformationQuantum> TheoremMemory<Q> {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            memory: MathematicalMemory::new(config),
            quanta: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Store theorem quantum
    pub fn store_theorem(&self, quantum: Q, tags: Vec<String>) {
        let id = quantum.id();
        let timestamp = *self.memory.current_time.read();

        let entry = MemoryEntry::new(id.clone().into(), quantum.clone(), timestamp)
            .with_tags(tags);

        self.memory.store(entry);

        // Add to quantum list for similarity search
        self.quanta.write().push(quantum);
    }

    /// Recall similar theorems based on resonance
    pub fn recall_similar(&self, query: &Q, top_k: usize) -> Vec<Q> {
        let quanta = self.quanta.read();

        let mut similarities: Vec<_> = quanta.iter()
            .map(|q| (q.resonance(query), q.clone()))
            .collect();

        similarities.sort_by(|a, b| {
            b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
        });

        similarities.into_iter()
            .take(top_k)
            .map(|(_, q)| q)
            .collect()
    }

    /// Recall by tag
    pub fn recall_by_tag(&self, tag: &str) -> Vec<Q> {
        self.memory.recall_by_tag(tag)
    }

    /// Tick time forward
    pub fn tick(&self, dt: f64) {
        self.memory.tick(dt);
    }

    /// Decay memories
    pub fn decay_all(&self) {
        self.memory.decay_all();

        // Sync quantum list with storage
        let storage = self.memory.storage.read();
        let mut quanta = self.quanta.write();

        quanta.retain(|q| {
            let id = q.id();
            storage.contains_key(&id.into())
        });
    }

    pub fn stats(&self) -> MemoryStats {
        self.memory.stats()
    }

    pub fn count(&self) -> usize {
        self.memory.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage() {
        let config = MemoryConfig::default();
        let memory = MathematicalMemory::<String>::new(config);

        let entry = MemoryEntry::new(1, "theorem1".to_string(), 0.0)
            .with_tags(vec!["number_theory".to_string()]);

        memory.store(entry);

        assert_eq!(memory.count(), 1);

        let recalled = memory.recall(1);
        assert_eq!(recalled, Some("theorem1".to_string()));
    }

    #[test]
    fn test_tag_recall() {
        let config = MemoryConfig::default();
        let memory = MathematicalMemory::<String>::new(config);

        memory.store(
            MemoryEntry::new(1, "prime_theorem".to_string(), 0.0)
                .with_tags(vec!["primes".to_string()])
        );

        memory.store(
            MemoryEntry::new(2, "another_prime_theorem".to_string(), 1.0)
                .with_tags(vec!["primes".to_string()])
        );

        let results = memory.recall_by_tag("primes");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_memory_decay() {
        let config = MemoryConfig {
            decay_rate: 0.1,
            forget_threshold: 0.3,
            ..Default::default()
        };

        let memory = MathematicalMemory::<String>::new(config);

        memory.store(MemoryEntry::new(1, "theorem".to_string(), 0.0));

        memory.tick(5.0);
        memory.decay_all();

        // After decay, importance should drop
        let stats = memory.stats();
        assert!(stats.average_importance < 0.5);
    }

    #[test]
    fn test_capacity_limit() {
        let config = MemoryConfig {
            max_entries: 3,
            ..Default::default()
        };

        let memory = MathematicalMemory::<String>::new(config);

        memory.store(MemoryEntry::new(1, "t1".to_string(), 0.0));
        memory.store(MemoryEntry::new(2, "t2".to_string(), 1.0));
        memory.store(MemoryEntry::new(3, "t3".to_string(), 2.0));
        memory.store(MemoryEntry::new(4, "t4".to_string(), 3.0));

        // Should still be at max capacity
        assert_eq!(memory.count(), 3);
    }
}

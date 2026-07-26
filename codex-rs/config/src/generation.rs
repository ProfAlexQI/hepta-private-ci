use std::fmt;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

/// Process-local source for monotonic effective-config generations.
///
/// Config loaders freeze a [`ConfigGeneration`] before they begin reading
/// configuration. Runtime invalidation publishes a newer value through the
/// same source, allowing consumers to reject snapshots that were loaded
/// against an older source generation.
#[derive(Clone, Default)]
pub struct ConfigGenerationSource {
    current: Arc<AtomicU64>,
}

impl ConfigGenerationSource {
    pub fn new(initial_generation: u64) -> Self {
        Self {
            current: Arc::new(AtomicU64::new(initial_generation)),
        }
    }

    pub fn current(&self) -> u64 {
        self.current.load(Ordering::Acquire)
    }

    pub fn freeze(&self) -> ConfigGeneration {
        ConfigGeneration {
            source: self.clone(),
            value: self.current(),
        }
    }

    /// Returns whether `generation` was frozen from this exact process-local source.
    ///
    /// The numeric value alone is not provenance: two independent sources can
    /// legitimately hold the same value while representing unrelated config
    /// lifetimes.
    pub fn is_source_of(&self, generation: &ConfigGeneration) -> bool {
        Arc::ptr_eq(&self.current, &generation.source.current)
    }

    /// Publishes `generation` without allowing the source to move backwards.
    pub fn publish(&self, generation: u64) -> u64 {
        self.current
            .fetch_max(generation, Ordering::AcqRel)
            .max(generation)
    }
}

impl fmt::Debug for ConfigGenerationSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConfigGenerationSource")
            .field("current", &self.current())
            .finish()
    }
}

/// Generation frozen onto one effective runtime configuration snapshot.
///
/// This type intentionally has no serialization representation. It is runtime
/// provenance for a loaded snapshot, not part of `config.toml` or app-server
/// protocol payloads.
#[derive(Clone)]
pub struct ConfigGeneration {
    source: ConfigGenerationSource,
    value: u64,
}

impl ConfigGeneration {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn source(&self) -> ConfigGenerationSource {
        self.source.clone()
    }

    pub fn is_current(&self) -> bool {
        self.value == self.source.current()
    }
}

impl Default for ConfigGeneration {
    fn default() -> Self {
        ConfigGenerationSource::default().freeze()
    }
}

impl fmt::Debug for ConfigGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConfigGeneration")
            .field("value", &self.value)
            .finish()
    }
}

impl PartialEq for ConfigGeneration {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ConfigGeneration {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_generation_does_not_change_when_source_advances() {
        let source = ConfigGenerationSource::default();
        let generation_zero = source.freeze();

        assert_eq!(source.publish(1), 1);
        assert_eq!(generation_zero.value(), 0);
        assert!(!generation_zero.is_current());
        assert_eq!(source.freeze().value(), 1);
        assert!(source.freeze().is_current());
    }

    #[test]
    fn equal_numeric_generations_from_distinct_sources_are_not_same_provenance() {
        let expected_source = ConfigGenerationSource::new(7);
        let unrelated_source = ConfigGenerationSource::new(7);
        let expected = expected_source.freeze();
        let unrelated = unrelated_source.freeze();

        assert!(expected_source.is_source_of(&expected));
        assert!(!expected_source.is_source_of(&unrelated));
        assert_eq!(expected.value(), unrelated.value());
    }
}

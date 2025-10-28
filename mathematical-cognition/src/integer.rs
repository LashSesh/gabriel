//! Integer representation as information quanta

use crate::{MathId, MathQuantum, DifficultyLevel, normalize_similarity};
use gabriel_core::InformationQuantum;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Integer number with mathematical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathInteger {
    pub id: MathId,
    pub value: i64,
    /// Mathematical properties (even, odd, prime, composite, etc.)
    pub properties: HashSet<String>,
    /// How many times this integer appeared in successful proofs
    pub proof_count: usize,
}

impl MathInteger {
    pub fn new(value: i64) -> Self {
        let mut properties = HashSet::new();

        // Detect basic properties
        if value == 0 {
            properties.insert("zero".to_string());
        }
        if value == 1 {
            properties.insert("unity".to_string());
        }
        if value > 0 {
            properties.insert("positive".to_string());
        } else if value < 0 {
            properties.insert("negative".to_string());
        }
        if value % 2 == 0 {
            properties.insert("even".to_string());
        } else {
            properties.insert("odd".to_string());
        }

        // Prime detection (simple)
        if is_prime(value) {
            properties.insert("prime".to_string());
        } else if value > 1 {
            properties.insert("composite".to_string());
        }

        // Perfect square
        let sqrt = (value.abs() as f64).sqrt();
        if sqrt.fract() == 0.0 {
            properties.insert("perfect_square".to_string());
        }

        Self {
            id: MathId::from_hash(&value),
            value,
            properties,
            proof_count: 0,
        }
    }

    pub fn with_properties(value: i64, properties: HashSet<String>) -> Self {
        Self {
            id: MathId::from_hash(&value),
            value,
            properties,
            proof_count: 0,
        }
    }
}

/// Simple primality test
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

impl InformationQuantum for MathInteger {
    type Id = MathId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Integers resonate based on:
        // 1. Shared mathematical properties
        // 2. Numerical proximity
        // 3. Divisibility relationships

        let property_similarity = crate::structural_similarity(&self.properties, &other.properties);

        // Numerical proximity (0.0-1.0, closer = higher)
        let diff = (self.value - other.value).abs() as f64;
        let proximity = 1.0 / (1.0 + diff / 100.0);

        // Divisibility bonus
        let mut divisibility = 0.0;
        if self.value != 0 && other.value % self.value == 0 {
            divisibility = 0.2;
        } else if other.value != 0 && self.value % other.value == 0 {
            divisibility = 0.2;
        }

        // Weighted combination
        normalize_similarity(
            0.5 * property_similarity +
            0.3 * proximity +
            0.2 * divisibility
        )
    }

    fn energy(&self) -> f64 {
        // Energy = complexity + usage in proofs
        let magnitude = (self.value.abs() as f64).ln().max(0.1);
        let property_richness = self.properties.len() as f64 * 0.1;
        let proof_experience = (self.proof_count as f64 * 0.05).min(0.5);

        normalize_similarity(0.3 + magnitude * 0.1 + property_richness + proof_experience)
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusion creates a new integer that blends properties
        // Weighted average of values
        let new_value = ((self.value as f64 * (1.0 - weight)) +
                        (other.value as f64 * weight)) as i64;

        // Merge properties
        let mut new_properties = self.properties.clone();
        new_properties.extend(other.properties.iter().cloned());

        // Average proof count
        let new_proof_count = ((self.proof_count as f64 * (1.0 - weight)) +
                              (other.proof_count as f64 * weight)) as usize;

        Self {
            id: MathId::from_hash(&new_value),
            value: new_value,
            properties: new_properties,
            proof_count: new_proof_count,
        }
    }
}

impl MathQuantum for MathInteger {
    fn complexity(&self) -> f64 {
        // Complexity based on magnitude and properties
        let magnitude_complexity = (self.value.abs() as f64).ln() / 10.0;
        let property_complexity = self.properties.len() as f64 * 0.05;

        normalize_similarity(magnitude_complexity + property_complexity)
    }

    fn mathematical_similarity(&self, other: &Self) -> f64 {
        // Same as resonance for integers
        self.resonance(other)
    }

    fn is_provable(&self) -> bool {
        // All integer properties are decidable
        true
    }

    fn certainty(&self) -> f64 {
        // Integers are certain
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_detection() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(17));
        assert!(!is_prime(4));
        assert!(!is_prime(1));
    }

    #[test]
    fn test_integer_properties() {
        let n = MathInteger::new(12);
        assert!(n.properties.contains("even"));
        assert!(n.properties.contains("composite"));
        assert!(n.properties.contains("positive"));
    }

    #[test]
    fn test_integer_resonance() {
        let a = MathInteger::new(12);
        let b = MathInteger::new(12);
        let c = MathInteger::new(100);

        // Identical integers have high resonance
        assert!(a.resonance(&b) > 0.9);

        // Distant integers have lower resonance
        assert!(a.resonance(&c) < 0.5);
    }

    #[test]
    fn test_integer_fusion() {
        let a = MathInteger::new(10);
        let b = MathInteger::new(20);

        let fused = a.fuse(&b, 0.5);
        assert_eq!(fused.value, 15);
    }
}

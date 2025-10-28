//! Complex number representation for complex analysis

use crate::{MathId, MathQuantum, normalize_similarity};
use gabriel_core::InformationQuantum;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};

/// Complex number with analytical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathComplex {
    pub id: MathId,
    pub value: Complex64,
    /// On unit circle?
    pub on_unit_circle: bool,
    /// Is a root of unity?
    pub is_root_of_unity: bool,
    /// Related to zeta function zeros?
    pub is_critical_strip: bool,
    /// Usage in complex analysis theorems
    pub analysis_count: usize,
}

impl MathComplex {
    pub fn new(re: f64, im: f64) -> Self {
        let value = Complex64::new(re, im);
        let modulus = value.norm();

        let on_unit_circle = (modulus - 1.0).abs() < 1e-6;

        // Check if root of unity (crude approximation)
        let is_root_of_unity = on_unit_circle && {
            let angle = value.arg();
            let n_approx = std::f64::consts::TAU / angle;
            (n_approx.round() - n_approx).abs() < 0.01
        };

        // Critical strip: 0 < Re(s) < 1
        let is_critical_strip = re > 0.0 && re < 1.0;

        Self {
            id: MathId::from_hash(&(re.to_bits(), im.to_bits())),
            value,
            on_unit_circle,
            is_root_of_unity,
            is_critical_strip,
            analysis_count: 0,
        }
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    pub fn modulus(&self) -> f64 {
        self.value.norm()
    }

    pub fn argument(&self) -> f64 {
        self.value.arg()
    }

    /// For Riemann zeta: check if near critical line Re(s) = 1/2
    pub fn distance_to_critical_line(&self) -> f64 {
        (self.value.re - 0.5).abs()
    }
}

impl InformationQuantum for MathComplex {
    type Id = MathId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Complex resonance based on:
        // 1. Distance in complex plane
        // 2. Shared analytical properties
        // 3. Argument similarity

        let distance = (self.value - other.value).norm();
        let proximity = 1.0 / (1.0 + distance);

        let mut property_match = 0.0;
        if self.on_unit_circle && other.on_unit_circle {
            property_match += 0.3;
        }
        if self.is_root_of_unity && other.is_root_of_unity {
            property_match += 0.2;
        }
        if self.is_critical_strip && other.is_critical_strip {
            property_match += 0.3; // Important for Riemann hypothesis
        }

        // Argument similarity (angular)
        let arg_diff = (self.argument() - other.argument()).abs();
        let arg_similarity = 1.0 / (1.0 + arg_diff);

        normalize_similarity(
            0.4 * proximity +
            0.3 * property_match +
            0.3 * arg_similarity
        )
    }

    fn energy(&self) -> f64 {
        // Energy = distance from origin + special properties + usage
        let magnitude_energy = self.modulus().ln().max(0.0) / 10.0;

        let mut special_bonus = 0.0;
        if self.on_unit_circle {
            special_bonus += 0.15;
        }
        if self.is_root_of_unity {
            special_bonus += 0.2;
        }
        if self.is_critical_strip {
            special_bonus += 0.25; // Critical strip is key for Riemann
        }

        let usage = (self.analysis_count as f64 * 0.05).min(0.3);

        normalize_similarity(0.3 + magnitude_energy + special_bonus + usage)
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Linear interpolation in complex plane
        let new_value = self.value * (1.0 - weight) + other.value * weight;
        Self::new(new_value.re, new_value.im)
    }
}

impl MathQuantum for MathComplex {
    fn complexity(&self) -> f64 {
        // Complex analysis is more advanced than real analysis
        let base_complexity = 0.6;
        let magnitude = self.modulus().ln().max(0.0) / 20.0;

        let critical_bonus = if self.is_critical_strip { 0.2 } else { 0.0 };

        normalize_similarity(base_complexity + magnitude + critical_bonus)
    }

    fn mathematical_similarity(&self, other: &Self) -> f64 {
        self.resonance(other)
    }

    fn is_provable(&self) -> bool {
        // Complex number properties are decidable
        true
    }

    fn certainty(&self) -> f64 {
        // Complex numbers are certain
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_complex_creation() {
        let z = MathComplex::new(3.0, 4.0);
        assert!((z.modulus() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_unit_circle() {
        let z = MathComplex::from_polar(1.0, PI / 4.0);
        assert!(z.on_unit_circle);
    }

    #[test]
    fn test_critical_strip() {
        let z1 = MathComplex::new(0.5, 14.134); // Near first Riemann zero
        let z2 = MathComplex::new(2.0, 5.0);    // Outside critical strip

        assert!(z1.is_critical_strip);
        assert!(!z2.is_critical_strip);
    }

    #[test]
    fn test_critical_line_distance() {
        let on_line = MathComplex::new(0.5, 10.0);
        let off_line = MathComplex::new(0.8, 10.0);

        assert!(on_line.distance_to_critical_line() < 1e-6);
        assert!((off_line.distance_to_critical_line() - 0.3).abs() < 1e-6);
    }
}

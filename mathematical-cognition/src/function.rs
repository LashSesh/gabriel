//! Mathematical function representation

use crate::{MathId, MathQuantum, MathDomain, normalize_similarity, levenshtein_distance};
use gabriel_core::InformationQuantum;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Category of mathematical function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FunctionType {
    Polynomial,      // x^n + ...
    Rational,        // P(x)/Q(x)
    Exponential,     // e^x, a^x
    Logarithmic,     // ln(x), log(x)
    Trigonometric,   // sin, cos, tan
    Special,         // Gamma, zeta, etc.
    Composite,       // f(g(x))
    Unknown,
}

/// Mathematical function as information quantum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathFunction {
    pub id: MathId,
    pub name: String,
    pub function_type: FunctionType,
    pub expression: String,
    pub domain: MathDomain,
    /// Function properties
    pub properties: HashSet<String>,
    /// How many times used in successful proofs
    pub usage_count: usize,
}

impl MathFunction {
    pub fn new(name: String, function_type: FunctionType, expression: String) -> Self {
        let id = MathId::from_hash(&expression);

        let mut properties = HashSet::new();

        // Add type-based properties
        match function_type {
            FunctionType::Polynomial => {
                properties.insert("continuous".to_string());
                properties.insert("differentiable".to_string());
            }
            FunctionType::Exponential => {
                properties.insert("continuous".to_string());
                properties.insert("positive".to_string());
            }
            FunctionType::Special => {
                properties.insert("advanced".to_string());
            }
            _ => {}
        }

        let domain = match function_type {
            FunctionType::Special => MathDomain::ComplexAnalysis,
            FunctionType::Trigonometric => MathDomain::Analysis,
            _ => MathDomain::Algebra,
        };

        Self {
            id,
            name,
            function_type,
            expression,
            domain,
            properties,
            usage_count: 0,
        }
    }

    pub fn with_properties(mut self, props: Vec<String>) -> Self {
        self.properties.extend(props);
        self
    }

    pub fn add_property(&mut self, prop: String) {
        self.properties.insert(prop);
    }

    pub fn is_analytic(&self) -> bool {
        self.properties.contains("analytic") ||
        matches!(self.function_type, FunctionType::Polynomial | FunctionType::Exponential)
    }
}

impl InformationQuantum for MathFunction {
    type Id = MathId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn resonance(&self, other: &Self) -> f64 {
        // Functions resonate based on:
        // 1. Same type
        // 2. Shared properties
        // 3. Syntactic similarity

        let type_match = if self.function_type == other.function_type {
            0.5
        } else {
            0.0
        };

        let property_similarity = crate::structural_similarity(&self.properties, &other.properties);

        let max_len = self.expression.len().max(other.expression.len()).max(1);
        let edit_distance = levenshtein_distance(&self.expression, &other.expression);
        let text_similarity = 1.0 - (edit_distance as f64 / max_len as f64);

        normalize_similarity(
            0.4 * type_match +
            0.4 * property_similarity +
            0.2 * text_similarity
        )
    }

    fn energy(&self) -> f64 {
        // Energy = complexity of function type + properties + usage
        let type_energy = match self.function_type {
            FunctionType::Polynomial => 0.2,
            FunctionType::Rational => 0.3,
            FunctionType::Exponential => 0.4,
            FunctionType::Logarithmic => 0.4,
            FunctionType::Trigonometric => 0.5,
            FunctionType::Special => 0.8, // Zeta, Gamma, etc.
            FunctionType::Composite => 0.6,
            FunctionType::Unknown => 0.3,
        };

        let property_richness = (self.properties.len() as f64 * 0.05).min(0.3);
        let usage = (self.usage_count as f64 * 0.05).min(0.3);

        normalize_similarity(type_energy + property_richness + usage)
    }

    fn fuse(&self, other: &Self, weight: f64) -> Self {
        // Fusion creates a composite function
        let new_name = format!("{}∘{}", self.name, other.name);
        let new_expression = format!("{}({})", self.expression, other.expression);

        let mut fused = MathFunction::new(
            new_name,
            FunctionType::Composite,
            new_expression,
        );

        // Merge properties
        fused.properties = self.properties.union(&other.properties).cloned().collect();

        // Weighted usage
        fused.usage_count = ((self.usage_count as f64 * (1.0 - weight)) +
                            (other.usage_count as f64 * weight)) as usize;

        fused
    }
}

impl MathQuantum for MathFunction {
    fn complexity(&self) -> f64 {
        // Function complexity
        let base = match self.function_type {
            FunctionType::Polynomial => 0.3,
            FunctionType::Rational => 0.4,
            FunctionType::Exponential => 0.5,
            FunctionType::Logarithmic => 0.5,
            FunctionType::Trigonometric => 0.6,
            FunctionType::Special => 0.9, // Riemann zeta!
            FunctionType::Composite => 0.7,
            FunctionType::Unknown => 0.4,
        };

        normalize_similarity(base)
    }

    fn mathematical_similarity(&self, other: &Self) -> f64 {
        self.resonance(other)
    }

    fn is_provable(&self) -> bool {
        // Function properties are generally provable
        true
    }

    fn certainty(&self) -> f64 {
        // Functions are well-defined
        1.0
    }
}

/// The Riemann Zeta function
pub fn riemann_zeta() -> MathFunction {
    MathFunction::new(
        "ζ".to_string(),
        FunctionType::Special,
        "ζ(s) = Σ_{n=1}^∞ 1/n^s".to_string(),
    )
    .with_properties(vec![
        "analytic".to_string(),
        "meromorphic".to_string(),
        "fundamental".to_string(),
        "number_theory".to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_creation() {
        let f = MathFunction::new(
            "f".to_string(),
            FunctionType::Polynomial,
            "x^2 + 1".to_string(),
        );

        assert!(f.properties.contains("continuous"));
        assert!(f.properties.contains("differentiable"));
    }

    #[test]
    fn test_riemann_zeta() {
        let zeta = riemann_zeta();
        assert_eq!(zeta.name, "ζ");
        assert_eq!(zeta.function_type, FunctionType::Special);
        assert!(zeta.properties.contains("analytic"));
        assert!(zeta.is_analytic());
    }

    #[test]
    fn test_function_resonance() {
        let f1 = MathFunction::new(
            "f".to_string(),
            FunctionType::Polynomial,
            "x^2".to_string(),
        );

        let f2 = MathFunction::new(
            "g".to_string(),
            FunctionType::Polynomial,
            "x^3".to_string(),
        );

        let f3 = riemann_zeta();

        // Similar polynomial functions
        assert!(f1.resonance(&f2) > 0.6);

        // Very different functions
        assert!(f1.resonance(&f3) < 0.3);
    }

    #[test]
    fn test_function_fusion() {
        let f = MathFunction::new(
            "f".to_string(),
            FunctionType::Exponential,
            "e^x".to_string(),
        );

        let g = MathFunction::new(
            "g".to_string(),
            FunctionType::Polynomial,
            "x^2".to_string(),
        );

        let composed = f.fuse(&g, 0.5);
        assert_eq!(composed.function_type, FunctionType::Composite);
        assert!(composed.expression.contains("e^x"));
    }
}

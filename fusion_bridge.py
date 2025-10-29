#!/usr/bin/env python3
"""
Fusion Bridge: Python interface between Triton scoring and Gabriel organism.

This module wraps Triton's analytical scoring functions and provides a unified
API for the fusion layer to evaluate resonance cycles quantitatively.
"""

import json
import numpy as np
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass, asdict
import triton_core
import triton_scorer


@dataclass
class FusionPacket:
    """Unified data model for fusion layer communication.
    
    Attributes:
        resonance_tensor: N-dimensional tensor from Gabriel resonance computation
        coherence_score: Triton coherence evaluation (psi)
        entropy_score: Information entropy measure (derived from rho)
        stability_index: Overall stability metric (derived from D)
    """
    resonance_tensor: List[float]
    coherence_score: float
    entropy_score: float
    stability_index: float
    cycle_id: int
    timestamp: float
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        return {
            'resonance_tensor': self.resonance_tensor,
            'coherence_score': float(self.coherence_score),
            'entropy_score': float(self.entropy_score),
            'stability_index': float(self.stability_index),
            'cycle_id': int(self.cycle_id),
            'timestamp': float(self.timestamp),
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'FusionPacket':
        """Create from dictionary."""
        return cls(**data)


class TritonBridge:
    """Bridge between Triton scoring framework and Gabriel organism."""
    
    def __init__(self, seed: int = 42, coherence_window: int = 8):
        """Initialize Triton bridge.
        
        Args:
            seed: Random seed for reproducibility
            coherence_window: Window size for coherence calculations
        """
        self.seed = seed
        self.coherence_window = coherence_window
        
        # Initialize Triton evaluator (using target embedding for coherence)
        # Create a default target embedding for resonance evaluation
        target_embedding = np.array([0.5, 0.5, 0.5, 0.5, 0.5], dtype=np.float64)
        self.evaluator = triton_core.InformationAlchemyEvaluator(
            target_embedding=target_embedding,
            coherence_window=coherence_window
        )
        
        # Metrics tracking
        self.evaluation_count = 0
        self.history: List[FusionPacket] = []
    
    def evaluate_coherence(self, tensor: np.ndarray) -> float:
        """Evaluate coherence of a resonance tensor.
        
        Args:
            tensor: Input resonance tensor (1D array)
            
        Returns:
            Coherence score in [0.0, 1.0]
        """
        # Ensure tensor is the right shape for Triton (5D)
        if len(tensor) < 5:
            # Pad to 5 dimensions
            padded = np.zeros(5, dtype=np.float64)
            padded[:len(tensor)] = tensor[:5] if len(tensor) >= 5 else tensor
            tensor = padded
        elif len(tensor) > 5:
            # Take first 5 components
            tensor = tensor[:5]
        
        # Normalize to unit vector
        norm = np.linalg.norm(tensor)
        if norm > 1e-12:
            tensor = tensor / norm
        
        # Use Triton evaluator to get spectral signature
        signature = self.evaluator.evaluate(tensor)
        
        return signature.psi
    
    def score_density(self, tensor: np.ndarray) -> float:
        """Calculate density/structure score (rho).
        
        Args:
            tensor: Input resonance tensor
            
        Returns:
            Density score in [0.0, 1.0]
        """
        # Prepare tensor same way as coherence
        if len(tensor) < 5:
            padded = np.zeros(5, dtype=np.float64)
            padded[:len(tensor)] = tensor[:5] if len(tensor) >= 5 else tensor
            tensor = padded
        elif len(tensor) > 5:
            tensor = tensor[:5]
        
        norm = np.linalg.norm(tensor)
        if norm > 1e-12:
            tensor = tensor / norm
        
        signature = self.evaluator.evaluate(tensor)
        return signature.rho
    
    def calculate_stability(self, tensor: np.ndarray) -> Tuple[float, float]:
        """Calculate stability index and entropy from tensor.
        
        Args:
            tensor: Input resonance tensor
            
        Returns:
            Tuple of (stability_index, entropy_score)
        """
        # Prepare tensor
        if len(tensor) < 5:
            padded = np.zeros(5, dtype=np.float64)
            padded[:len(tensor)] = tensor[:5] if len(tensor) >= 5 else tensor
            tensor = padded
        elif len(tensor) > 5:
            tensor = tensor[:5]
        
        norm = np.linalg.norm(tensor)
        if norm > 1e-12:
            tensor = tensor / norm
        
        # Get full spectral signature
        signature = self.evaluator.evaluate(tensor)
        
        # Stability is the resonance score D = psi * rho * omega
        stability_index = signature.D
        
        # Entropy is inverse of density (higher rho = lower entropy)
        entropy_score = 1.0 - signature.rho
        
        return stability_index, entropy_score
    
    def fusion_cycle(self, resonance_tensor: List[float], cycle_id: int, 
                    timestamp: float) -> FusionPacket:
        """Execute complete fusion cycle evaluation.
        
        This is the main API function that:
        1. Reads Gabriel output tensors
        2. Routes them through Triton scorer
        3. Returns updated feedback weights
        
        Args:
            resonance_tensor: Resonance tensor from Gabriel organism
            cycle_id: Cycle identifier
            timestamp: Current time
            
        Returns:
            FusionPacket with all evaluated metrics
        """
        # Convert to numpy array
        tensor_np = np.array(resonance_tensor, dtype=np.float64)
        
        # Evaluate all metrics
        coherence = self.evaluate_coherence(tensor_np)
        density = self.score_density(tensor_np)
        stability, entropy = self.calculate_stability(tensor_np)
        
        # Create fusion packet
        packet = FusionPacket(
            resonance_tensor=resonance_tensor,
            coherence_score=coherence,
            entropy_score=entropy,
            stability_index=stability,
            cycle_id=cycle_id,
            timestamp=timestamp
        )
        
        # Track in history
        self.history.append(packet)
        self.evaluation_count += 1
        
        return packet
    
    def get_feedback_weights(self, packet: FusionPacket) -> Dict[str, float]:
        """Convert fusion packet to feedback weights for Gabriel.
        
        Args:
            packet: FusionPacket with evaluation results
            
        Returns:
            Dictionary of feedback weights for metabolic update
        """
        # Map Triton scores to Gabriel feedback parameters
        return {
            'hebbian_modulation': packet.coherence_score,  # Higher coherence = stronger learning
            'energy_boost': packet.stability_index,  # Higher stability = more energy
            'entropy_reduction': 1.0 - packet.entropy_score,  # Lower entropy = more order
            'pruning_threshold': 1.0 - packet.coherence_score,  # Lower coherence = more pruning
        }
    
    def compute_gradient(self, packet: FusionPacket) -> Optional[np.ndarray]:
        """Compute gradient hint for Hebbian update (optional).
        
        Args:
            packet: Current fusion packet
            
        Returns:
            Gradient vector if available, None otherwise
        """
        if len(self.history) < 2:
            return None
        
        # Approximate gradient from recent history
        prev_packet = self.history[-2]
        
        delta_stability = packet.stability_index - prev_packet.stability_index
        delta_coherence = packet.coherence_score - prev_packet.coherence_score
        
        # Create gradient hint (direction of improvement)
        tensor_np = np.array(packet.resonance_tensor[:5], dtype=np.float64)
        
        # Scale by improvement
        improvement_factor = delta_stability + delta_coherence
        gradient = tensor_np * improvement_factor
        
        return gradient
    
    def export_diagnostics(self, filename: str = "fusion_diagnostics.json"):
        """Export fusion metrics to JSON file.
        
        Args:
            filename: Output filename
        """
        diagnostics = {
            "total_cycles": int(self.evaluation_count),
            "average_coherence": float(np.mean([p.coherence_score for p in self.history])) if self.history else 0.0,
            "average_stability": float(np.mean([p.stability_index for p in self.history])) if self.history else 0.0,
            "average_entropy": float(np.mean([p.entropy_score for p in self.history])) if self.history else 0.0,
            "energy_drift": float(self._calculate_energy_drift()),
            "convergence_rate": float(self._calculate_convergence_rate()),
            "history": [p.to_dict() for p in self.history[-100:]]  # Last 100 cycles
        }
        
        with open(filename, 'w') as f:
            json.dump(diagnostics, f, indent=2)
    
    def _calculate_energy_drift(self) -> float:
        """Calculate energy drift over recent cycles."""
        if len(self.history) < 2:
            return 0.0
        
        # Energy proxy: stability_index
        recent = self.history[-10:]
        if len(recent) < 2:
            return 0.0
        
        energies = [p.stability_index for p in recent]
        drift = max(energies) - min(energies)
        return drift
    
    def _calculate_convergence_rate(self) -> float:
        """Calculate convergence rate (improvement over time)."""
        if len(self.history) < 10:
            return 0.0
        
        # Linear fit of stability over last cycles
        recent = self.history[-50:]
        stabilities = [p.stability_index for p in recent]
        
        # Simple slope calculation
        x = np.arange(len(stabilities))
        slope = np.polyfit(x, stabilities, 1)[0]
        
        return float(slope)


def export_fusion_state(bridge: TritonBridge, filename: str = "fusion_state.json"):
    """Export complete fusion state for Hilbert-Pólya-Metatron integration.
    
    Args:
        bridge: TritonBridge instance
        filename: Output filename
    """
    state = {
        "fusion_version": "1.0.0",
        "total_evaluations": int(bridge.evaluation_count),
        "coherence_window": int(bridge.coherence_window),
        "seed": int(bridge.seed),
        "final_metrics": {
            "coherence": float(bridge.history[-1].coherence_score) if bridge.history else 0.0,
            "stability": float(bridge.history[-1].stability_index) if bridge.history else 0.0,
            "entropy": float(bridge.history[-1].entropy_score) if bridge.history else 0.0,
        },
        "history_summary": {
            "total_cycles": int(len(bridge.history)),
            "average_coherence": float(np.mean([p.coherence_score for p in bridge.history])) if bridge.history else 0.0,
            "average_stability": float(np.mean([p.stability_index for p in bridge.history])) if bridge.history else 0.0,
        }
    }
    
    with open(filename, 'w') as f:
        json.dump(state, f, indent=2)


# Example usage for testing
if __name__ == "__main__":
    # Create bridge
    bridge = TritonBridge(seed=42)
    
    # Simulate some fusion cycles
    print("Testing Fusion Bridge...")
    for i in range(10):
        # Mock resonance tensor from Gabriel
        tensor = list(np.random.randn(8))
        
        # Execute fusion cycle
        packet = bridge.fusion_cycle(tensor, cycle_id=i, timestamp=float(i))
        
        # Get feedback weights
        weights = bridge.get_feedback_weights(packet)
        
        print(f"Cycle {i}: coherence={packet.coherence_score:.3f}, "
              f"stability={packet.stability_index:.3f}, "
              f"entropy={packet.entropy_score:.3f}")
    
    # Export diagnostics
    bridge.export_diagnostics()
    export_fusion_state(bridge)
    
    print("\n✓ Fusion bridge test complete")
    print(f"✓ Diagnostics exported to fusion_diagnostics.json")
    print(f"✓ State exported to fusion_state.json")

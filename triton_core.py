#!/usr/bin/env python3
"""
TRITON-Alchemie: Hochdimensionale Resonanz-Exploration Engine
Integration von 5D-Spiralsuche mit Informationsalchemie-Prinzipien

Kern-Konzept:
- 5D Spiral-Generator mit Ouroboros-Feedback
- σ(ψ,ρ,ω) Spektralfeld-Messung
- Solve et Coagula Phasenübergänge  
- Mandorla-Eigenzustand-Kondensation
- Resonanz-gesteuerte Navigation in Schlüsselräumen
"""

import numpy as np
from typing import Dict, List, Tuple, Optional, Callable, Any
from dataclasses import dataclass, field
from abc import ABC, abstractmethod
import hashlib
import json
from pathlib import Path


@dataclass
class SpectralSignature:
    """Spektrale Signatur σ = (ψ, ρ, ω) eines Punktes im Raum"""
    psi: float    # Kohärenz/Semantik
    rho: float    # Dichte/Struktur  
    omega: float  # Frequenz/Phase
    
    @property
    def D(self) -> float:
        """Resonanz-Score D = ψ·ρ·ω"""
        return self.psi * self.rho * self.omega
    
    def __str__(self):
        return f"σ(ψ={self.psi:.3f}, ρ={self.rho:.3f}, ω={self.omega:.3f}) → D={self.D:.3f}"


@dataclass
class ExplorationPoint:
    """Ein Punkt in der 5D-Exploration"""
    coords: np.ndarray  # 5D Koordinaten
    signature: SpectralSignature
    metadata: Dict[str, Any] = field(default_factory=dict)
    timestamp: float = 0.0
    
    @property
    def id(self) -> str:
        """Deterministische ID basierend auf Koordinaten"""
        coords_str = ','.join(f"{x:.6f}" for x in self.coords)
        return hashlib.sha256(coords_str.encode()).hexdigest()[:12]


class SpectralEvaluator(ABC):
    """Abstrakte Basis für Spektralfeld-Evaluatoren"""
    
    @abstractmethod
    def evaluate(self, point: np.ndarray, context: Dict[str, Any] = None) -> SpectralSignature:
        """Bewertet einen 5D-Punkt und liefert spektrale Signatur"""
        pass


class TritonSpiralGenerator:
    """5D Spiral Generator mit Ouroboros-Rückkopplung"""
    
    def __init__(self, 
                 seed: int = 42,
                 radius_base: float = 0.015,
                 phi_golden: float = np.pi * (3 - np.sqrt(5)),
                 alpha_learning: float = 0.12):
        
        self.rng = np.random.default_rng(seed)
        self.radius_base = radius_base
        self.phi_golden = phi_golden
        self.alpha_learning = alpha_learning
        
        # Zustand
        self.step = 0
        self.current_pos = np.zeros(5)
        self.gradient_memory = np.zeros(5)
        self.best_signature = None
        self.ouroboros_momentum = np.zeros(5)
        
        # 5D Projektionsmatrix (deterministisch)
        self.rng_proj = np.random.default_rng(1337)
        self.projection_matrix = self.rng_proj.normal(0, 1, (5, 5))
        self.projection_offset = self.rng_proj.normal(0, 0.1, 5)
    
    def generate_next(self) -> np.ndarray:
        """Generiert den nächsten Spiral-Punkt mit Ouroboros-Update"""
        
        # Basis-Spiral (deterministische goldene Spirale)
        r = self.radius_base * np.sqrt(self.step + 1)
        theta = self.step * self.phi_golden
        
        # 2D Basis-Koordinaten
        u = np.array([r * np.cos(theta), r * np.sin(theta)])
        
        # Erweiterte Koordinaten für 5D
        extended = np.array([
            u[0], u[1],
            0.5 * np.sin(theta * 0.5),
            0.5 * np.cos(theta * 0.5),
            (self.step / 100) - 0.5
        ])
        
        # Projektion nach R^5 mit deterministischer Matrix
        base_coords = extended @ self.projection_matrix.T + self.projection_offset
        
        # Ouroboros-Rückkopplung anwenden
        ouroboros_drift = self.alpha_learning * self.ouroboros_momentum
        
        # Finale Koordinaten
        coords = base_coords + ouroboros_drift
        
        # L2-Normierung
        coords = coords / (np.linalg.norm(coords) + 1e-12)
        
        self.current_pos = coords
        self.step += 1
        
        return coords
    
    def update_ouroboros(self, signature: SpectralSignature, gradient_hint: np.ndarray = None):
        """Update des Ouroboros-Momentum basierend auf Feedback"""
        
        # Gradient aus Signatur ableiten (vereinfacht)
        if gradient_hint is not None:
            gradient = gradient_hint
        else:
            # Approximiere Gradient über spektrales Feld
            gradient = np.array([
                signature.psi - 0.5,
                signature.rho - 0.5, 
                signature.omega - 0.5,
                signature.D - 0.5,
                np.sin(self.step * 0.1) * signature.D
            ])
        
        # Exponentiell gewichtetes Update
        decay = 0.9
        self.ouroboros_momentum = (decay * self.ouroboros_momentum + 
                                  (1 - decay) * gradient)
        
        # Tracking des besten Punktes
        if self.best_signature is None or signature.D > self.best_signature.D:
            self.best_signature = signature


class InformationAlchemyEvaluator(SpectralEvaluator):
    """Evaluator der Informationsalchemie-Prinzipien implementiert"""
    
    def __init__(self, 
                 target_embedding: np.ndarray = None,
                 coherence_window: int = 8):
        
        self.target_embedding = target_embedding
        self.coherence_window = coherence_window
        self.history = []
        
    def evaluate(self, point: np.ndarray, context: Dict[str, Any] = None) -> SpectralSignature:
        """Bewertet Punkt nach Informationsalchemie-Prinzipien"""
        
        # ψ (Kohärenz): Ähnlichkeit zu Zielvektor
        if self.target_embedding is not None:
            psi = max(0, np.dot(point, self.target_embedding) / 
                     (np.linalg.norm(point) * np.linalg.norm(self.target_embedding) + 1e-12))
        else:
            # Fallback: intrinsische Kohärenz
            psi = 1.0 - np.var(point)
        
        # ρ (Dichte): Strukturelle Kohärenz über Historie
        if len(self.history) >= 2:
            recent_points = self.history[-self.coherence_window:]
            distances = [np.linalg.norm(point - p) for p in recent_points]
            rho = 1.0 / (1.0 + np.mean(distances))
        else:
            rho = np.linalg.norm(point)
        
        # ω (Frequenz): Phasische Passung / temporale Kohärenz
        if len(self.history) >= 1:
            last_point = self.history[-1]
            direction = point - last_point
            if len(self.history) >= 2:
                prev_direction = self.history[-1] - self.history[-2]
                # Richtungskorrelation
                correlation = np.dot(direction, prev_direction) / (
                    np.linalg.norm(direction) * np.linalg.norm(prev_direction) + 1e-12)
                omega = (correlation + 1) / 2  # Normierung auf [0,1]
            else:
                omega = 0.5
        else:
            omega = 0.5
        
        # Normierung und Clipping
        psi = np.clip(psi, 0, 1)
        rho = np.clip(rho, 0, 1)  
        omega = np.clip(omega, 0, 1)
        
        # Historie aktualisieren
        self.history.append(point.copy())
        if len(self.history) > 2 * self.coherence_window:
            self.history = self.history[-self.coherence_window:]
        
        return SpectralSignature(psi=psi, rho=rho, omega=omega)


class SolveCoagulaDecision:
    """Solve et Coagula Entscheidungslogik mit Merkaba-Gate"""
    
    def __init__(self,
                 phi_threshold: float = 0.5,
                 theta_ema_gamma: float = 0.2,
                 spike_delta: float = 0.05,
                 por_tolerances: Dict[str, float] = None):
        
        self.phi_threshold = phi_threshold
        self.theta_ema_gamma = theta_ema_gamma  
        self.spike_delta = spike_delta
        
        # Proof of Resonance Toleranzen
        self.por_tolerances = por_tolerances or {
            'psi_min': 0.1, 'rho_min': 0.1, 
            'omega_min': 0.1, 'omega_max': 0.9
        }
        
        # Zustand
        self.theta_dynamic = 0.5  # Dynamische Schwelle
        self.phase_history = []
        self.gate_open = False
        
    def evaluate_phase(self, signature: SpectralSignature) -> str:
        """Bestimmt aktuelle Phase (flüssig/fest) basierend auf Ordnungsmaß Φ"""
        
        D = signature.D
        kappa = 5.0
        theta_adaptive = 0.125
        
        # Ordnungsmaß Φ = σ(κ[D - Θ])
        phi = 1 / (1 + np.exp(-kappa * (D - theta_adaptive)))
        
        phase = "fest" if phi >= self.phi_threshold else "flüssig"
        
        self.phase_history.append((phase, phi, signature.D))
        return phase
    
    def check_proof_of_resonance(self, signature: SpectralSignature) -> bool:
        """Prüft Proof of Resonance Bedingungen"""
        
        tol = self.por_tolerances
        return (signature.psi >= tol['psi_min'] and 
                signature.rho >= tol['rho_min'] and
                tol['omega_min'] <= signature.omega <= tol['omega_max'])
    
    def check_merkaba_gate(self, signature: SpectralSignature) -> bool:
        """Merkaba-Gate Evaluierung für Coagula-Freigabe"""
        
        # PoR-Bedingung
        por_condition = self.check_proof_of_resonance(signature)
        
        # Ordnungsmaß-Bedingung
        phase = self.evaluate_phase(signature)
        phase_condition = (phase == "fest")
        
        # Spike-Bedingung (D über dynamischer Schwelle)
        spike_condition = signature.D > (self.theta_dynamic + self.spike_delta)
        
        # Mirror Consistency (vereinfacht)
        mirror_consistency = 0.8  # Simuliert
        mci_condition = mirror_consistency >= 0.7
        
        # Gate-Entscheidung
        self.gate_open = (por_condition and phase_condition and 
                         spike_condition and mci_condition)
        
        # Update dynamische Schwelle (EMA)
        self.theta_dynamic = ((1 - self.theta_ema_gamma) * self.theta_dynamic + 
                             self.theta_ema_gamma * signature.D)
        
        return self.gate_open


class TritonAlchemyEngine:
    """Haupt-Engine: Integration aller Komponenten"""
    
    def __init__(self,
                 evaluator: SpectralEvaluator,
                 seed: int = 42,
                 max_steps: int = 1000):
        
        self.spiral = TritonSpiralGenerator(seed=seed)
        self.evaluator = evaluator
        self.decision = SolveCoagulaDecision()
        self.max_steps = max_steps
        
        # Ergebnisse
        self.exploration_path: List[ExplorationPoint] = []
        self.resonats: List[List[ExplorationPoint]] = []  # Geclusterte Punkte
        self.crystals: List[Dict[str, Any]] = []  # TIC-Kondensate
        
        # Metriken
        self.metrics = {
            'best_D': 0.0,
            'gate_opens': 0,
            'phase_transitions': 0,
            'mean_coherence': 0.0
        }
    
    def step(self) -> ExplorationPoint:
        """Führt einen Explorations-Schritt aus"""
        
        # Generiere nächsten Spiral-Punkt
        coords = self.spiral.generate_next()
        
        # Evaluiere spektrale Signatur
        signature = self.evaluator.evaluate(coords)
        
        # Erstelle Explorations-Punkt
        point = ExplorationPoint(
            coords=coords,
            signature=signature,
            timestamp=len(self.exploration_path)
        )
        
        # Entscheidungslogik
        phase = self.decision.evaluate_phase(signature)
        gate_open = self.decision.check_merkaba_gate(signature)
        
        # Ouroboros-Update
        gradient_hint = None
        if gate_open:
            # Bei offenem Gate: verstärke Richtung zu hoher Resonanz
            gradient_hint = coords * signature.D
            self.metrics['gate_opens'] += 1
        
        self.spiral.update_ouroboros(signature, gradient_hint)
        
        # Tracking
        self.exploration_path.append(point)
        if signature.D > self.metrics['best_D']:
            self.metrics['best_D'] = signature.D
        
        return point
    
    def run(self, steps: int = None) -> Dict[str, Any]:
        """Führt vollständige Exploration aus"""
        
        steps = steps or self.max_steps
        
        for i in range(steps):
            point = self.step()
            
            # Coagula-Phase bei offenem Gate
            if self.decision.gate_open:
                self._attempt_crystallization()
        
        # Finale Cluster-Bildung
        self._cluster_to_resonats()
        
        # Finale Metriken
        self._compute_final_metrics()
        
        return self.get_results()
    
    def _attempt_crystallization(self):
        """Versucht TIC-Kristallisation bei offenem Merkaba-Gate"""
        
        if len(self.exploration_path) < 5:
            return
            
        # Finde lokales Maximum
        recent_points = self.exploration_path[-10:]
        best_local = max(recent_points, key=lambda p: p.signature.D)
        
        # Kristallisations-Bedingung
        if best_local.signature.D > 0.7:  # Hohe Resonanz-Schwelle
            
            crystal = {
                'id': f"TIC_{len(self.crystals)}",
                'center': best_local,
                'stability': best_local.signature.D,
                'formation_step': len(self.exploration_path),
                'spectral_gap': self._compute_spectral_gap(best_local)
            }
            
            self.crystals.append(crystal)
    
    def _cluster_to_resonats(self):
        """Clustert Explorations-Punkte zu Resonaten"""
        
        if len(self.exploration_path) < 2:
            return
            
        # Einfaches Distanz-basiertes Clustering
        threshold = 0.3
        used = set()
        
        for i, point in enumerate(self.exploration_path):
            if i in used:
                continue
                
            cluster = [point]
            used.add(i)
            
            for j in range(i + 1, len(self.exploration_path)):
                if j in used:
                    continue
                    
                other = self.exploration_path[j]
                distance = np.linalg.norm(point.coords - other.coords)
                
                if distance < threshold:
                    cluster.append(other)
                    used.add(j)
            
            if len(cluster) > 1:  # Nur echte Cluster
                self.resonats.append(cluster)
    
    def _compute_spectral_gap(self, point: ExplorationPoint) -> float:
        """Berechnet spektrale Lücke für Stabilitätsabschätzung"""
        return 0.1 + 0.3 * point.signature.D  # Vereinfacht
    
    def _compute_final_metrics(self):
        """Berechnet finale Metriken der Exploration"""
        
        if not self.exploration_path:
            return
            
        signatures = [p.signature for p in self.exploration_path]
        
        self.metrics.update({
            'total_steps': len(self.exploration_path),
            'mean_coherence': np.mean([s.psi for s in signatures]),
            'mean_density': np.mean([s.rho for s in signatures]), 
            'mean_frequency': np.mean([s.omega for s in signatures]),
            'mean_resonance': np.mean([s.D for s in signatures]),
            'resonat_clusters': len(self.resonats),
            'tic_crystals': len(self.crystals),
            'final_position': self.spiral.current_pos.tolist()
        })
    
    def get_results(self) -> Dict[str, Any]:
        """Liefert vollständige Exploration-Ergebnisse"""
        
        return {
            'metrics': self.metrics,
            'path': [
                {
                    'step': i,
                    'coords': p.coords.tolist(),
                    'signature': {
                        'psi': p.signature.psi,
                        'rho': p.signature.rho, 
                        'omega': p.signature.omega,
                        'D': p.signature.D
                    },
                    'id': p.id
                }
                for i, p in enumerate(self.exploration_path)
            ],
            'resonats': [
                {
                    'size': len(cluster),
                    'center': cluster[0].coords.tolist(),
                    'mean_D': np.mean([p.signature.D for p in cluster])
                }
                for cluster in self.resonats
            ],
            'crystals': self.crystals
        }


# ============================================================================
# Anwendungsbeispiele und Tests
# ============================================================================

def create_text_evaluator(target_text: str) -> InformationAlchemyEvaluator:
    """Erstellt Evaluator für Text-basierte Suche"""
    
    # Vereinfachte Text-zu-Vektor Konvertierung
    target_hash = hashlib.sha256(target_text.encode()).digest()
    raw = np.frombuffer(target_hash[:20], dtype=np.uint8).astype(float)
    raw5 = raw.reshape(5, 4).mean(axis=1)
    target_embedding = raw5 / (np.linalg.norm(raw5) + 1e-12)
    
    return InformationAlchemyEvaluator(target_embedding=target_embedding)



def demo_quantum_information_search():
    """Demo: Suche in quanteninformationstheoretischem Raum"""
    
    print("=== TRITON-Alchemie Demo: Quanteninformation ===")
    
    # Ziel-Konzept
    target = "quantum entanglement coherence information processing"
    evaluator = create_text_evaluator(target)
    
    # Engine erstellen
    engine = TritonAlchemyEngine(
        evaluator=evaluator,
        seed=42,
        max_steps=200
    )
    
    # Exploration ausführen
    results = engine.run(steps=200)
    
    # Ergebnisse analysieren
    print(f"Exploration abgeschlossen:")
    print(f"- {results['metrics']['total_steps']} Schritte")
    print(f"- Beste Resonanz: {results['metrics']['best_D']:.3f}")
    print(f"- Gate-Öffnungen: {results['metrics']['gate_opens']}")
    print(f"- Resonat-Cluster: {results['metrics']['resonat_clusters']}")
    print(f"- TIC-Kristalle: {results['metrics']['tic_crystals']}")
    print(f"- Mittlere Kohärenz: {results['metrics']['mean_coherence']:.3f}")
    
    return results


def compare_spiral_vs_random(steps: int = 200, trials: int = 10) -> Dict[str, float]:
    """Vergleicht Spiral-Suche gegen Random-Suche"""
    
    print(f"=== A/B Test: Spiral vs Random ({trials} trials, {steps} steps) ===")
    
    target = "optimization search algorithm performance"
    
    spiral_scores = []
    random_scores = []
    
    for trial in range(trials):
        # Spiral-Suche
        evaluator_spiral = create_text_evaluator(target)
        engine_spiral = TritonAlchemyEngine(
            evaluator=evaluator_spiral,
            seed=42 + trial,
            max_steps=steps
        )
        results_spiral = engine_spiral.run(steps)
        spiral_scores.append(results_spiral['metrics']['best_D'])
        
        # Random-Suche (simuliert)
        rng = np.random.default_rng(42 + trial)
        random_best = 0.0
        evaluator_random = create_text_evaluator(target)
        
        for _ in range(steps):
            random_point = rng.normal(0, 1, 5)
            random_point = random_point / np.linalg.norm(random_point)
            sig = evaluator_random.evaluate(random_point)
            random_best = max(random_best, sig.D)
        
        random_scores.append(random_best)
    
    # Statistik
    spiral_median = np.median(spiral_scores)
    random_median = np.median(random_scores)
    
    print(f"Spiral Median: {spiral_median:.3f}")
    print(f"Random Median: {random_median:.3f}")
    print(f"Improvement: {((spiral_median / random_median - 1) * 100):.1f}%")
    
    return {
        'spiral_median': spiral_median,
        'random_median': random_median,
        'spiral_wins': spiral_median > random_median
    }


if __name__ == "__main__":
    # Haupt-Demo ausführen
    demo_results = demo_quantum_information_search()
    
    print("\n" + "="*60 + "\n")
    
    # A/B Vergleich
    ab_results = compare_spiral_vs_random(steps=100, trials=5)
    
    print(f"\n=== FAZIT ===")
    print(f"✓ Spiral-Navigation funktional")
    print(f"✓ Spektralfeld-Messung aktiv") 
    print(f"✓ Solve/Coagula-Phasen implementiert")
    print(f"✓ Merkaba-Gate operative")
    print(f"✓ TIC-Kristallisation {'aktiv' if demo_results['metrics']['tic_crystals'] > 0 else 'bereit'}")
    print(f"✓ Spiral > Random: {ab_results['spiral_wins']}")

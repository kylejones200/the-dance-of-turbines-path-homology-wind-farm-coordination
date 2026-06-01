#!/usr/bin/env python3
"""Python vs Rust kernel benchmark."""

from __future__ import annotations

import time
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "src"))
from compute_kernel import turbine_pairwise_distances  # noqa: E402

def main() -> None:
    n = 60
    x = np.ascontiguousarray(np.cos(np.arange(n) * 0.5) * 100.0)
    y = np.ascontiguousarray(np.sin(np.arange(n) * 0.5) * 100.0)
    t0 = time.perf_counter()
    for _ in range(200):
        turbine_pairwise_distances(x, y)
    py_s = time.perf_counter() - t0
    try:
        import the_dance_of_turbines_path_homology_wind_farm_coordination_rs as rs
    except ImportError:
        print("Build: maturin develop --release -m rust/py/Cargo.toml")
        print(f"Python {py_s:.3f}s")
        return
    rs_s = rs.bench_kernel_py(x, y, 500)
    print(f"Python {py_s:.3f}s Rust {rs_s:.3f}s speedup {py_s / max(rs_s, 1e-9):.1f}x")
    np.testing.assert_allclose(
        turbine_pairwise_distances(x, y),
        np.asarray(rs.turbine_pairwise_distances_py(x, y)),
        rtol=1e-10,
    )
    print("Correctness: OK")

if __name__ == "__main__":
    main()

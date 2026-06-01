# Repository

Companion code for a Medium article.

## Business context

Wind farms are not collections of isolated turbines but interconnected systems where machines influence each other through wake interactions, shared grid connections, and coordinated control responses. When weather fronts sweep across a farm, turbines do not respond independently—power outputs rise and fall in patterns that propagate spatially from upwind to downwind machines. When grid operators issue curtailment commands, reductions occur simultaneously or in cascades depending on control architecture. When one turbine faults and trips offline, neighbors may adjust to compensate or, in severe cases, trip themselves through grid disturbances.

Understanding these coordination patterns matters for farm-level optimization. If power variations propagate predictably from turbine A to turbine B with a consistent lag, coordinated yaw control can steer A's wake away from B before the variation arrives. If multiple turbines respond simultaneously to mesoscale weather patterns, farm-level forecasting that exploits spatial correlation outperforms turbine-by-turbine prediction. If fault propagation follows specific spatial paths, protection systems can be designed to interrupt cascades before they spread.

Traditional analysis treats turbines as nodes in a spatial graph with edges based on physical proximity or wake interactions. Cross-correlation analysis identifies which turbines respond similarly, and lag analysis estimates propagation delays. These approaches capture pairwise relationships but miss higher-order structure—how chains of influences create paths through the farm, how multiple paths converge or diverge, and how feedback loops close when downstream effects circle back to affect upstream machines.

## Setup

1. Copy `.env.example` to `.env` and set `NREL_API_KEY` (free at [developer.nrel.gov/signup](https://developer.nrel.gov/signup/)). Optionally set `NREL_EMAIL` for large downloads.
2. Adjust non-secret NREL settings in `config.yaml` (`nrel.lat`, `nrel.lon`, `nrel.years`, etc.).
3. Install dependencies: `uv sync` (or `pip install -e .`).

Runnable scripts load `config.yaml` and read secrets from `.env` via `python-dotenv` (see `nrel_wtk.py`).

## Rust performance port

Side-by-side **Python vs Rust** implementation of the numeric hot loop — turbine pairwise distances. Reference PyO3 benchmark: **see `benchmark_rust.py`** on a release build (local machine; run `benchmark_rust.py` to reproduce).

| Path | Role |
|------|------|
| `src/compute_kernel.py` | Python/numpy reference kernel |
| `rust/core/` | Pure Rust library |
| `rust/py/` | PyO3 bindings |
| `rust/bench/` | Standalone CLI benchmark |
| `benchmark_rust.py` | Python vs Rust timing + correctness check |

```bash
# Rust-only CLI benchmark
cd rust && cargo run --release -p the_dance_of_turbines_path_homology_wind_farm_coordination_bench

# Python vs Rust (PyO3)
pip install maturin numpy
maturin develop --release -m rust/py/Cargo.toml
python benchmark_rust.py
```

Python ML training, solvers, and orchestration stay in Python; Rust targets the numeric hot loops. Stochastic generators validate output shapes; deterministic kernels match at tight floating-point tolerance.


## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).
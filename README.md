# The Dance of Turbines

Companion code for the Medium article on detecting coordinated responses in wind farms using **persistent path homology** on directed lead–lag networks.

Classifies three coordination regimes — wake propagation, grid events, and oscillatory instabilities — from simulated multi-turbine power time series built on NREL wind patterns.

## Quick start

```bash
# Install dependencies
uv sync   # or: pip install -e .

# Configure NREL API access (free key at developer.nrel.gov/signup)
cp .env.example .env
# Edit .env and set NREL_API_KEY

# Run the full analysis pipeline
python farm_coordination.py

# Optional: regenerate the demo animation
python create_animation.py
```

Non-secret settings (site location, years, model hyperparameters) live in `config.yaml`. Secrets (`NREL_API_KEY`, optional `NREL_EMAIL`) stay in `.env` — never commit that file.

## Project layout

| Path | Description |
|------|-------------|
| `article.md` | Full article text with embedded code |
| `farm_coordination.py` | Main pipeline: fetch data → features → train → visualize |
| `nrel_wtk.py` | NREL Wind Toolkit CSV fetch via `config.yaml` + `.env` |
| `create_animation.py` | Generates `images/coordination_animation.gif` |
| `images/` | Pre-built figures and animation (also written by the pipeline) |
| `src/compute_kernel.py` | Python/numpy reference kernel |
| `rust/` | Rust port of the numeric hot loop (PyO3 + CLI benchmark) |
| `benchmark_rust.py` | Python vs Rust timing and correctness check |

## Rust performance port

Side-by-side **Python vs Rust** implementation of turbine pairwise distances.

```bash
# Rust-only CLI benchmark
cd rust && cargo run --release -p the_dance_of_turbines_path_homology_wind_farm_coordination_bench

# Python vs Rust (PyO3)
pip install maturin numpy
maturin develop --release -m rust/py/Cargo.toml
python benchmark_rust.py
```

Python handles ML training, solvers, and orchestration; Rust targets the numeric hot loops.

## Business context

Wind farms are not collections of isolated turbines but interconnected systems where machines influence each other through wake interactions, shared grid connections, and coordinated control responses. When weather fronts sweep across a farm, turbines do not respond independently — power outputs rise and fall in patterns that propagate spatially from upwind to downwind machines. When grid operators issue curtailment commands, reductions occur simultaneously or in cascades depending on control architecture. When one turbine faults and trips offline, neighbors may adjust to compensate or, in severe cases, trip themselves through grid disturbances.

Understanding these coordination patterns matters for farm-level optimization. If power variations propagate predictably from turbine A to turbine B with a consistent lag, coordinated yaw control can steer A's wake away from B before the variation arrives. If multiple turbines respond simultaneously to mesoscale weather patterns, farm-level forecasting that exploits spatial correlation outperforms turbine-by-turbine prediction. If fault propagation follows specific spatial paths, protection systems can be designed to interrupt cascades before they spread.

Traditional analysis treats turbines as nodes in a spatial graph with edges based on physical proximity or wake interactions. Cross-correlation analysis identifies which turbines respond similarly, and lag analysis estimates propagation delays. These approaches capture pairwise relationships but miss higher-order structure — how chains of influences create paths through the farm, how multiple paths converge or diverge, and how feedback loops close when downstream effects circle back to affect upstream machines.

## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).

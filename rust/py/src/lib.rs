use the_dance_of_turbines_path_homology_wind_farm_coordination_core::turbine_pairwise_distances;
use numpy::{PyArray1, PyReadonlyArray1, IntoPyArray};
use pyo3::prelude::*;

#[pyfunction]
fn turbine_pairwise_distances_py<'py>(
    py: Python<'py>,
    x: PyReadonlyArray1<f64>,
    y: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    Ok(turbine_pairwise_distances(x.as_slice()?, y.as_slice()?).into_pyarray(py))
}

#[pyfunction]
#[pyo3(signature = (x, y, iterations=500))]
fn bench_kernel_py(
    x: PyReadonlyArray1<f64>,
    y: PyReadonlyArray1<f64>,
    iterations: usize,
) -> PyResult<f64> {
    let xb = x.as_slice()?.to_vec();
    let yb = y.as_slice()?.to_vec();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = turbine_pairwise_distances(&xb, &yb);
    }
    Ok(start.elapsed().as_secs_f64())
}

#[pymodule]
fn the_dance_of_turbines_path_homology_wind_farm_coordination_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(turbine_pairwise_distances_py, m)?)?;
    m.add_function(wrap_pyfunction!(bench_kernel_py, m)?)?;
    Ok(())
}

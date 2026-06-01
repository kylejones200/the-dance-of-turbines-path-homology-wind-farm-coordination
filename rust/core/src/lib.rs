//! Pairwise Euclidean distances between turbine positions.

pub fn turbine_pairwise_distances(x: &[f64], y: &[f64]) -> Vec<f64> {
    assert_eq!(x.len(), y.len());
    let n = x.len();
    let mut out = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            let dx = x[i] - x[j];
            let dy = y[i] - y[j];
            out[i * n + j] = (dx * dx + dy * dy).sqrt();
        }
    }
    out
}

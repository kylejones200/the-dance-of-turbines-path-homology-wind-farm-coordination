use the_dance_of_turbines_path_homology_wind_farm_coordination_core::turbine_pairwise_distances;

fn main() {
    let n = 60usize;
    let x: Vec<f64> = (0..n).map(|i| (i as f64 * 0.5).cos() * 100.0).collect();
    let y: Vec<f64> = (0..n).map(|i| (i as f64 * 0.5).sin() * 100.0).collect();
    for _ in 0..500 {
        let _ = turbine_pairwise_distances(&x, &y);
    }
}

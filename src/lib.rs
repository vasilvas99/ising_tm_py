use numpy::ToPyArray;
use parallel_ising_tm as itm;
use pyo3::prelude::*;


/// Calculates the 2D Ising transfer matrix directly (no-BD) in parallel
/// n - int - number of spins in the chain
// jnn - (-1.0 default) - coupling constant
/// temp - non-dimensionalized temperature.
#[pymodule]
fn ising_tm_py(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "generate_ising_tm")]
    /// Calculates the 2D Ising transfer matrix directly (no-BD) in parallel
    /// n - int - number of spins in the chain
    // jnn - (-1.0 default) - coupling constant
    /// temp - non-dimensionalized temperature.
    fn generate_ising_tm<'py>(
        _py: Python<'py>,
        n: u32,
        jnn: f64,
        temp: f64,
    ) -> &'py numpy::PyArray2<f64> {
        let tm = itm::tm_n_parr(n, jnn, temp);
        tm.to_pyarray(_py)
    }
    Ok(())
}
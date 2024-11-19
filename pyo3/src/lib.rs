// #[cfg(all(target_os = "linux", not(use_mimalloc)))]
// use jemallocator::Jemalloc;
// #[cfg(any(not(target_os = "linux"), use_mimalloc))]
// use mimalloc::MiMalloc;

// #[global_allocator]
// #[cfg(all(target_os = "linux", not(use_mimalloc)))]
// static ALLOC: Jemalloc = Jemalloc;

// #[global_allocator]
// #[cfg(any(not(target_os = "linux"), use_mimalloc))]
// static ALLOC: MiMalloc = MiMalloc;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

// // Calculate the mean of a column
// #[pyfunction]
// fn calculate_mean_pl(a: PyDataFrame, colname: String, handle_nan: bool) -> Option<f64> {
//     let df: DataFrame = a.into();

//     let df = if handle_nan {
//         df.lazy()
//             .select([col(&colname).fill_nan(lit(Null))])
//             .collect()
//             .expect("Unable to coerce NaN values.")
//     } else {
//         df
//     };

//     df.column(&colname)
//         .ok()
//         .and_then(|col| col.mean())
// }

// // Calculate the median of a column
// #[pyfunction]
// fn calculate_median_pl(a: PyDataFrame, colname: String, handle_nan: bool) -> Option<f64> {
//     let df: DataFrame = a.into();
//     let df = match handle_nan {
//         true => {
//             df
//                 .lazy()
//                 .select([col(&colname).fill_nan(lit(NULL))])
//                 .collect()
//                 .expect("Unable to coerce NaN values.")
//         },
//         false => df
//     };

//     df[colname.as_str()].median()
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn polars_speed_test(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(calculate_mean_pl, m)?)?;
//     m.add_function(wrap_pyfunction!(calculate_median_pl, m)?)?;

//     Ok(())
// }
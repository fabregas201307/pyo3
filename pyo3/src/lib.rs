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

// Calculate the mean of a column
#[pyfunction]
fn calculate_mean_pl(a: PyDataFrame, colname: String, handle_nan: bool) -> Option<f64> {
    // Convert PyDataFrame to Polars DataFrame
    let df: DataFrame = a.into();

    // Check if the column exists
    if !df.get_column_names().contains(&colname.as_str()) {
        return None; // Return None if the column doesn't exist
    }

    // Handle NaN values if required
    let column = if handle_nan {
        df.column(&colname)
            .ok()
            .map(|col| col.f64()
                .ok()
                .map(|float_col| {
                    float_col
                        .apply(|v| if v.is_nan() { Some(0.0) } else { Some(v.expect("REASON")) })
                        .into_series()
                })
            )
            .flatten()
    } else {
        df.column(&colname).ok().map(|col| col.clone())
    };

    // Calculate the mean
    column.and_then(|col| col.mean())
}


// Calculate the median of a column
#[pyfunction]
fn calculate_median_pl(a: PyDataFrame, colname: String, handle_nan: bool) -> Option<f64> {
    // Convert PyDataFrame to Polars DataFrame
    let df: DataFrame = a.into();

    // Check if the column exists
    if !df.get_column_names().contains(&colname.as_str()) {
        return None; // Return None if the column doesn't exist
    }

    // Handle NaN values if required
    let column = if handle_nan {
        df.column(&colname)
            .ok()
            .map(|col| col.f64()
                .ok()
                .map(|float_col| {
                    float_col
                        .apply(|v| if v.is_nan() { Some(0.0) } else { Some(v.expect("REASON")) })
                        .into_series()
                })
            )
            .flatten()
    } else {
        df.column(&colname).ok().map(|col| col.clone())
    };

    // Calculate the mean
    column.and_then(|col| col.median())
}

// /// A Python module implemented in Rust.
// #[pymodule]
// fn polars_speed_test(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(calculate_mean_pl, m)?)?;
//     m.add_function(wrap_pyfunction!(calculate_median_pl, m)?)?;

//     Ok(())
// }
extern crate serde;
// extern crate serde_json;
// extern crate polars;
use log::{debug, error, info, warn};
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json;
// use serde;
use std::clone::Clone;


use ndarray::{Array, Array2};
// use ndarray_stats:SummaryStatisticsExt;
use statrs::statistics::Statistics;
// use statsmodels::regression::linear_model::OLS;
use polars::prelude::*;
// use polars;
use tokio;
// use linregress::Linregress;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

use std::fmt;



/// Wrapper for polars::prelude::DataFrame to expose to Python.
#[pyclass]
struct PyDataFrame {
    df: DataFrame,
}

#[pymethods]
impl PyDataFrame {
    #[new]
    fn new() -> Self {
        PyDataFrame { df: DataFrame::default() }
    }

    // Example method to expose DataFrame's height to Python
    fn height(&self) -> PyResult<usize> {
        Ok(self.df.height())
    }

    /// Slices the DataFrame and returns a new PyDataFrame with the slice.
    /// - `start`: The starting index of the slice.
    /// - `end`: The ending index of the slice.
    fn slice(&self, start: usize, end: usize) -> PyResult<PyDataFrame> {
        // Perform bounds checking or handle errors as needed.
        let sliced_df = self.df.slice(start as i64, (end - start) as i64);
        Ok(PyDataFrame { df: sliced_df })
    }

    /// Retrieves a column by name and returns it as a Python object.
    /// - `name`: The name of the column to retrieve.
    fn get_column(&self, name: &str) -> PyResult<PyObject> {
        match self.df.column(name) {
            Ok(column) => {
                // Assuming you have a PySeries wrapper similar to PyDataFrame
                // let py_series = PySeries { series: column.clone() };
                // Ok(Py::new(py, py_series)?)

                // Or convert the column to a Python list for simplicity
                let gil = Python::acquire_gil();
                let py = gil.python();
                let list: Vec<_> = column.f64().unwrap().into_iter().map(|opt_val| {
                    match opt_val {
                        Some(val) => val.to_object(py),
                        None => py.None(),
                    }
                }).collect();
                Ok(list.to_object(py))
            },
            Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Column '{}' not found", name),
            )),
        }
    }

}


// Assuming PyDataFrame is a struct, we will manually implement Clone.
impl Clone for PyDataFrame {
    fn clone(&self) -> Self {
        // Create a new PyDataFrame instance with cloned data from `self`
        // This is a simplified example. You would need to clone each field.
        PyDataFrame {
            // Assuming `data` is a field in PyDataFrame that needs to be cloned.
            // This requires that the type of `data` also implements `Clone`.
            data: self.data.clone(),
            // Repeat for other fields as necessary.
        }
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn my_polars_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDataFrame>()?;
    Ok(())
}


//// define pyfunction to fill OLS regression, given a dataframe
#[pyfunction]
fn rolling_ols_regression(df: PyDataFrame, n: usize) -> PyResult<DataFrame> {
    /*
    df: polars dataframe, columns are ['date', 'identifier', 'residual', 'y',
                                        'x', 'slope', 'intercept', 'rvalue']
        only one unique identifier is allowed, as this is a rolling regression
    n: window size to fit the ols regression
    */
    let nrows = df.height();
    let mut a: Vec<f64> = Vec::new();
    let mut b1: Vec<f64> = Vec::new();
    let mut rss: Vec<f64> = Vec::new();
    for i in n..nrows {
        let tmp = df.slice(i - n, i);
        let x = tmp.get_column("x").unwrap().f64().unwrap();
        let y = tmp.get_column("y").unwrap().f64().unwrap();
        let data = vec![("Y", y), ("X", x)];
        let data = RegressionDataBuilder::new().build_from(data)?;
        let formula = "Y ~ X";
        let model = FormulaRegressionBuilder::new()
            .data(&data)
            .formula(formula)
            .fit()?;

        let parameters: Vec<_> = model.iter_parameter_pairs().collect();
        let pvalues: Vec<_> = model.iter_p_value_pairs().collect();
        let standard_errors: Vec<_> = model.iter_se_pairs().collect();
        // get the intercept and slope from the model        
        let my_parameter = parameters.iter().find(|&&(key, _)| key == "X").map(|&(_, value)| value);
        let my_pvalue = pvalues.iter().find(|&&(key, _)| key == "X").map(|&(_, value)| value);
        let my_standard_error = standard_errors.iter().find(|&&(key, _)| key == "X").map(|&(_, value)| value);

        if let Some(value) = my_parameter {
            println!("Value of X: {}", value);
            a.push(value);
        } else {
            println!("X not found");
        }


        if let Some(value) = my_pvalue {
            println!("Value of X: {}", value);
            b1.push(value);
        } else {
            println!("X not found");
        }

        if let Some(value) = my_standard_error {
            println!("Value of X: {}", value);
            rss.push(value);
        } else {
            println!("X not found");
        }

        // a.push(my_parameter);
        // b1.push(my_pvalue);
        // rss.push(my_standard_error);
    }
    let a = Series::new("slope", a);
    let b1 = Series::new("intercept", b1);
    let rss = Series::new("rvalue", rss);
    let mut df = df.clone();
    df.add_column(a).unwrap();
    df.add_column(b1).unwrap();
    df.add_column(rss).unwrap();
    Ok(df)
}  


/// Multiply two numbers:
#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

/// Return the sum of a list/vector of numbers
#[pyfunction]
fn list_sum(a: Vec<isize>) -> PyResult<isize> {
    let mut sum: isize = 0;
    for i in a {
        sum += i;
    }
    Ok(sum)
}

/// Word printer:
/// Prints a word to the console n number of times.
/// Optionally, the word is printed in reverse and or in uppercase.
#[pyfunction]
fn word_printer(mut word: String, n: isize, reverse: bool, uppercase: bool) {
    if reverse {
        let mut reversed_word = String::new();
        for c in word.chars().rev() {
            reversed_word.push(c);
        }
        word = reversed_word;
    }
    if uppercase {
        word = word.to_uppercase();
    }
    for _ in 0..n {
        println!("{}", word);
    }
}

/// Print every item of a list to console:
#[pyfunction]
fn vector_printer(a: Vec<String>) {
    for string in a {
        println!("{}", string)
    }
}

// Print all the key values in a dict to console:
#[pyfunction]
fn dict_printer(hm: HashMap<String, String>) {
    for (key, value) in hm {
        println!("{} {}", key, value)
    }
}

/// Print every item in an array to console:
#[pyfunction]
fn array_printer(a: [String; 8]) {
    for string in a {
        println!("{}", string)
    }
}

#[pyfunction]
fn count_occurences(contents: &str, needle: &str) -> usize {
    let mut count = 0;
    for line in contents.lines() {
        for word in line.split(" ") {
            if word == needle || word == format!("{}.", needle) {
                count += 1;
            }
        }
    }
    count
}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn human_says_hi(human_data: String) {
    println!("{}", human_data);
    let human: Human = serde_json::from_str(&human_data).unwrap();

    println!(
        "Now we can work with the struct:\n {:#?}.\n {} is {} years old.",
        human, human.name, human.age,
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct Human {
    name: String,
    age: u8,
}

#[pyfunction]
fn log_different_levels() {
    error!("logging an error");
    warn!("logging a warning");
    info!("logging an info message");
    debug!("logging a debug message");
}

#[pyfunction]
fn log_example() {
    info!("A log message from {}!", "Rust");
}

#[pyfunction]
fn get_fibonacci(number: isize) -> PyResult<u128> {
    if number == 1 {
        return Ok(1);
    } else if number == 2 {
        return Ok(2);
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..number {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    Ok(sum)
}

// Raising an exception in a function called 'greater_than_2', which is defined later on.
// Some additional clarifications can be found here: https://blog.burntsushi.net/rust-error-handling/

// Define 'MyError' as a custom exception:
#[derive(Debug)]
struct MyError {
    /*
    the 'message' field that is used later on
    to be able print any message.
    */
    pub msg: &'static str,
}

// Implement the 'Error' trait for 'MyError':
impl std::error::Error for MyError {}

// Implement the 'Display' trait for 'MyError':
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error from Rust: {}", self.msg)
    }
}

// Implement the 'From' trait for 'MyError'.
// Used to do value-to-value conversions while consuming the input value.
impl std::convert::From<MyError> for PyErr {
    fn from(err: MyError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}

#[pyfunction]
// The function 'greater_than_2' raises an exception if the input value is 2 or less.
fn greater_than_2(number: isize) -> Result<isize, MyError> {
    if number <= 2 {
        return Err(MyError {
            msg: "number is less than or equal to 2",
        });
    } else {
        return Ok(number);
    }
}

#[pyclass]
pub struct RustStruct {
    #[pyo3(get, set)]
    pub data: String,
    #[pyo3(get, set)]
    pub vector: Vec<u8>,
}
#[pymethods]
impl RustStruct {
    #[new]
    pub fn new(data: String, vector: Vec<u8>) -> RustStruct {
        RustStruct { data, vector }
    }
    pub fn printer(&self) {
        println!("{}", self.data);
        for i in &self.vector {
            println!("{}", i);
        }
    }
    pub fn extend_vector(&mut self, extension: Vec<u8>) {
        println!("Extending the vector.");
        for i in extension {
            self.vector.push(i);
        }
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(word_printer, m)?)?;
    m.add_function(wrap_pyfunction!(vector_printer, m)?)?;
    m.add_function(wrap_pyfunction!(dict_printer, m)?)?;
    m.add_function(wrap_pyfunction!(array_printer, m)?)?;
    m.add_function(wrap_pyfunction!(count_occurences, m)?)?;
    m.add_function(wrap_pyfunction!(human_says_hi, m)?)?;
    m.add_wrapped(wrap_pyfunction!(log_example))?;
    m.add_wrapped(wrap_pyfunction!(log_different_levels))?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(greater_than_2, m)?)?;
    m.add_class::<RustStruct>()?;

    Ok(())
}

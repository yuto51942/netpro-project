mod applyhistory;
mod exception;
mod fileopration;
mod showhistory;
mod loopsum;

pub use loopsum::loop_sum;
pub use applyhistory::apply_history;
use ctrlc;
pub use fileopration::FileOperation;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
pub use showhistory::show_history;
use std::{path::Path, thread, time};

#[pyfunction]
fn fop(file_path: String, dir: String) -> PyResult<()> {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let _path = Path::new(&file_path);
    let directory = Path::new(&dir);
    let fp = FileOperation::new(_path, directory).unwrap();
    let sleep_time = time::Duration::from_nanos(100000000);

    while !fp.check().unwrap() {
        thread::sleep(sleep_time);
    }
    Ok(())
}

#[pyfunction]
fn show(dir: String, use_pager: bool) -> PyResult<()> {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let cache_dir = Path::new(&dir);
    show_history(cache_dir, use_pager).unwrap();
    Ok(())
}

#[pyfunction]
fn apply(hash: String, dir: String, target: String) -> PyResult<()> {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let cache_dir = Path::new(&dir);
    let _target = Path::new(&target);
    apply_history(hash, _target, cache_dir).unwrap();
    Ok(())
}

#[pyfunction]
fn test_loop(max: usize) -> PyResult<usize> {
    ctrlc::set_handler(|| std::process::exit(2)).unwrap();
    let ans = loop_sum(max);
    Ok(ans)
}

#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fop, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(apply, m)?)?;
    m.add_function(wrap_pyfunction!(test_loop, m)?)?;

    Ok(())
}

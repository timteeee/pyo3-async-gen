use pyo3::prelude::*;

#[pyclass]
pub struct Counter {
    curr: u32,
    count_to: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.count_to {
            None
        } else {
            self.curr += 1;
            Some(self.curr)
        }
    }
}

#[pymethods]
impl Counter {
    #[new]
    #[pyo3(signature = (count_to = 5))]
    fn new(count_to: Option<u32>) -> Self {
        Self {
            curr: 0,
            count_to: count_to.expect("default should be 5"),
        }
    }

    fn __iter__(_self: Py<Self>) -> Py<Self> {
        _self
    }

    fn __next__(&mut self) -> Option<u32> {
        self.next()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _internal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Counter>()?;

    Ok(())
}

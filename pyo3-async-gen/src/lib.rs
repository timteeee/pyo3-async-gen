use std::{sync::Arc, task::Poll, time::Duration};

use futures::Stream;
use pyo3::{exceptions::PyStopAsyncIteration, prelude::*};
use pyo3_asyncio_0_21 as pyo3_asyncio;
use tokio::sync::Mutex;

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

#[pyclass]
pub struct AsyncCounter {
    curr: Arc<Mutex<u32>>,
    count_to: u32,
}

#[pymethods]
impl AsyncCounter {
    #[new]
    #[pyo3(signature = (count_to = 5))]
    fn new(count_to: Option<u32>) -> Self {
        Self {
            curr: Arc::new(Mutex::new(0)),
            count_to: count_to.expect("default should be 5"),
        }
    }

    fn __aiter__(_self: Py<Self>) -> Py<Self> {
        _self
    }

    fn __anext__(&mut self, py: Python<'_>) -> PyResult<PyObject> {
        let curr = self.curr.clone();
        let count_to = self.count_to;

        let fut = pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut guard = curr.lock().await;

            if *guard >= count_to {
                Err(PyStopAsyncIteration::new_err("exhausted"))
            } else {
                *guard += 1;

                let duration = Duration::new(0, *guard * 1_000_000 * 100);
                tokio::time::sleep(duration).await;

                Ok(*guard)
            }
        });

        Ok(fut?.into())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _internal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Counter>()?;
    m.add_class::<AsyncCounter>()?;

    Ok(())
}

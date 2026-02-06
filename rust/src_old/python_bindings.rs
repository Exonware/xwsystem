// #exonware/xwsystem/rust/src_old/python_bindings.rs
//! Python bindings for xwsystem_rust using PyO3
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyDict;
#[cfg(feature = "python")]
use crate::version;
#[cfg(feature = "python")]
use crate::dummy;

/// Python module for version management
#[pymodule]
fn version_module(_py: Python, m: &PyModule) -> PyResult<()> {
    // Expose version constants
    m.add("VERSION", version::VERSION)?;
    m.add("VERSION_MAJOR", version::VERSION_MAJOR)?;
    m.add("VERSION_MINOR", version::VERSION_MINOR)?;
    m.add("VERSION_PATCH", version::VERSION_PATCH)?;
    m.add("VERSION_BUILD", version::VERSION_BUILD)?;
    m.add("VERSION_SUFFIX", version::VERSION_SUFFIX)?;
    m.add("VERSION_STRING", version::VERSION_STRING)?;

    // Expose version functions
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(get_version_info, m)?)?;
    m.add_function(wrap_pyfunction!(get_version_dict, m)?)?;
    m.add_function(wrap_pyfunction!(is_dev_version, m)?)?;
    m.add_function(wrap_pyfunction!(is_release_version, m)?)?;

    // Expose VersionInfo class
    m.add_class::<PyVersionInfo>()?;

    Ok(())
}

/// Get the current version string.
#[pyfunction]
#[cfg(feature = "python")]
fn get_version() -> &'static str {
    version::get_version()
}

/// Get version as a tuple (major, minor, patch, build).
#[pyfunction]
#[cfg(feature = "python")]
fn get_version_info() -> (u32, u32, u32, Option<u32>) {
    version::get_version_info()
}

/// Get version information as a dictionary.
#[pyfunction]
#[cfg(feature = "python")]
fn get_version_dict(py: Python) -> PyResult<PyObject> {
    let info = version::get_version_dict();
    let dict = PyDict::new(py);
    dict.set_item("version", info.version)?;
    dict.set_item("major", info.major)?;
    dict.set_item("minor", info.minor)?;
    dict.set_item("patch", info.patch)?;
    dict.set_item("build", info.build)?;
    dict.set_item("suffix", info.suffix)?;
    Ok(dict.into())
}

/// Check if this is a development version.
#[pyfunction]
#[cfg(feature = "python")]
fn is_dev_version() -> bool {
    version::is_dev_version()
}

/// Check if this is a release version.
#[pyfunction]
#[cfg(feature = "python")]
fn is_release_version() -> bool {
    version::is_release_version()
}

/// Python wrapper for VersionInfo struct
#[pyclass]
#[cfg(feature = "python")]
#[derive(Clone)]
struct PyVersionInfo {
    version: String,
    major: u32,
    minor: u32,
    patch: u32,
    build: Option<u32>,
    suffix: String,
}

#[pymethods]
#[cfg(feature = "python")]
impl PyVersionInfo {
    #[new]
    fn new() -> Self {
        let info = version::get_version_dict();
        Self {
            version: info.version,
            major: info.major,
            minor: info.minor,
            patch: info.patch,
            build: info.build,
            suffix: info.suffix,
        }
    }

    #[getter]
    fn version(&self) -> &str {
        &self.version
    }

    #[getter]
    fn major(&self) -> u32 {
        self.major
    }

    #[getter]
    fn minor(&self) -> u32 {
        self.minor
    }

    #[getter]
    fn patch(&self) -> u32 {
        self.patch
    }

    #[getter]
    fn build(&self) -> Option<u32> {
        self.build
    }

    #[getter]
    fn suffix(&self) -> &str {
        &self.suffix
    }

    fn __repr__(&self) -> String {
        format!(
            "VersionInfo(version='{}', major={}, minor={}, patch={}, build={:?}, suffix='{}')",
            self.version, self.major, self.minor, self.patch, self.build, self.suffix
        )
    }

    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("version", &self.version)?;
        dict.set_item("major", self.major)?;
        dict.set_item("minor", self.minor)?;
        dict.set_item("patch", self.patch)?;
        dict.set_item("build", self.build)?;
        dict.set_item("suffix", &self.suffix)?;
        Ok(dict.into())
    }
}

/// Python module for dummy functions
#[pymodule]
#[cfg(feature = "python")]
fn dummy_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dummy_complicated, m)?)?;
    m.add_class::<PyDummyResult>()?;
    Ok(())
}

/// A complicated dummy function that takes 5 inputs plus variable args
#[pyfunction]
#[cfg(feature = "python")]
#[pyo3(signature = (input1, input2, input3, input4, input5, *args))]
fn dummy_complicated(
    input1: i32,
    input2: i32,
    input3: i32,
    input4: String,
    input5: bool,
    args: Vec<i32>,
) -> PyDummyResult {
    let result = dummy::dummy_complicated(input1, input2, input3, input4, input5, args);
    PyDummyResult {
        output1: result.output1,
        output2: result.output2,
    }
}

/// Python wrapper for DummyResult struct
#[pyclass]
#[cfg(feature = "python")]
#[derive(Clone)]
struct PyDummyResult {
    output1: i64,
    output2: String,
}

#[pymethods]
#[cfg(feature = "python")]
impl PyDummyResult {
    #[getter]
    fn output1(&self) -> i64 {
        self.output1
    }

    #[getter]
    fn output2(&self) -> &str {
        &self.output2
    }

    fn __repr__(&self) -> String {
        format!("DummyResult(output1={}, output2='{}')", self.output1, self.output2)
    }

    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("output1", self.output1)?;
        dict.set_item("output2", &self.output2)?;
        Ok(dict.into())
    }
}

/// Main Python module entry point
#[pymodule]
#[cfg(feature = "python")]
fn _xwsystem_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    let version_submodule = PyModule::new(_py, "version")?;
    version_module(_py, version_submodule)?;
    m.add_submodule(version_submodule)?;
    
    let dummy_submodule = PyModule::new(_py, "dummy")?;
    dummy_module(_py, dummy_submodule)?;
    m.add_submodule(dummy_submodule)?;
    
    Ok(())
}


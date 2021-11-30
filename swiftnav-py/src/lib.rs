use pyo3::prelude::*;

fn make_coords_submodule(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "coords")?;

    #[pyclass]
    struct LLHDegrees(swiftnav::coords::LLHDegrees);
    #[pymethods]
    impl LLHDegrees {
        #[new]
        fn new(lat: f64, lon: f64, height: f64) -> Self {
            Self(swiftnav::coords::LLHDegrees::new(lat, lon, height))
        }
        #[getter]
        fn latitude(&self) -> PyResult<f64> {
            Ok(self.0.latitude())
        }
        #[getter]
        fn longitude(&self) -> PyResult<f64> {
            Ok(self.0.longitude())
        }
        #[getter]
        fn height(&self) -> PyResult<f64> {
            Ok(self.0.height())
        }
        fn to_ecef(&self) -> ECEF {
            ECEF(self.0.to_radians().to_ecef())
        }
    }
    m.add_class::<LLHDegrees>()?;

    #[pyclass]
    struct ECEF(swiftnav::coords::ECEF);
    #[pymethods]
    impl ECEF {
        #[new]
        fn new(x: f64, y: f64, z: f64) -> Self {
            Self(swiftnav::coords::ECEF::new(x, y, z))
        }
        #[getter]
        fn x(&self) -> PyResult<f64> {
            Ok(self.0.x())
        }
        #[getter]
        fn y(&self) -> PyResult<f64> {
            Ok(self.0.y())
        }
        #[getter]
        fn z(&self) -> PyResult<f64> {
            Ok(self.0.z())
        }
    }
    m.add_class::<ECEF>()?;

    parent_module.add_submodule(m)?;

    Ok(())
}

#[pymodule]
fn swiftnav(py: Python, m: &PyModule) -> PyResult<()> {
    make_coords_submodule(py, m)?;

    Ok(())
}
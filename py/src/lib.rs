use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Debug)]
pub struct CepRecord {
    pub cep: &'static str,
    pub uf: &'static str,
    pub localidade: &'static str,
    pub bairro: &'static str,
    pub logradouro: &'static str,
}

impl From<&br_ceps::CepRecord> for CepRecord {
    fn from(v: &br_ceps::CepRecord) -> Self {
        Self {
            cep: v.cep,
            uf: v.uf,
            localidade: v.localidade,
            bairro: v.bairro,
            logradouro: v.logradouro,
        }
    }
}

#[pyfunction]
pub fn get(cep: &str) -> Option<CepRecord> {
    br_ceps::get(cep).map(CepRecord::from)
}

#[pymodule]
fn brceps(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)
}

mod ceps;
#[derive(Debug, Clone, Copy)]
pub struct CepRecord {
    pub cep: &'static str,
    pub uf: &'static str,
    pub localidade: &'static str,
    pub bairro: &'static str,
    pub logradouro: &'static str,
}

pub fn get(cep: &str) -> Option<&CepRecord> {
    ceps::CEPS.get(cep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r = &get("01001001").unwrap();
        dbg!(r);
        assert_eq!(r.cep, "01001001");
        assert_eq!(r.uf, "SP");
        assert_eq!(r.localidade, "São Paulo");
        assert_eq!(r.bairro, "Sé");
        assert_eq!(r.logradouro, "Pç da Sé");
    }
}

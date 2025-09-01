use crate::exercicios::ex_038::{
    categorias::Categorias,
    ano_atual::AnoAtual
};

pub struct Atleta {
    _ano_de_nascimento: u32,
    idade: u8,
    categoria: Categorias,
}

impl Atleta {
    pub fn new(
        ano_de_nascimento: u32
    ) -> Self {
        let ano_atual = AnoAtual::new();
        let ano_atual = ano_atual.get_ano();

        let idade: u8 = ((ano_atual as u32) - ano_de_nascimento) as u8;

        let categoria = if idade > 25 {
            Categorias::MASTER
        } else if idade > 19 {
            Categorias::SENIOR
        } else if idade > 14 {
            Categorias::JUNIOR
        } else if idade > 9 {
            Categorias::INFANTIL
        } else {
            Categorias::MIRIM
        };

        Self { 
            _ano_de_nascimento: ano_de_nascimento,
            idade,
            categoria
        }
    }

    pub fn get_idade(
        &self
    ) -> u8 {
        return self.idade;
    }

    pub fn get_categoria(
        &self
    ) -> String {
        match self.categoria {
            Categorias::MASTER => return "MASTER".to_string(),
            Categorias::SENIOR => return "SÊNIOR".to_string(),
            Categorias::JUNIOR => return "JÚNIOR".to_string(),
            Categorias::INFANTIL => return "INFANTIL".to_string(),
            Categorias::MIRIM => return "MIRIM".to_string()
        }
    }
}

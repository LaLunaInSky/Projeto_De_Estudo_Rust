use crate::exercicios::ex_051::ano_atual::AnoAtual;

#[derive(Clone)]
pub struct Pessoa {
    _ano_de_nascimento: u32,
    idade: u8,
    é_maior_de_idade: bool,
}

impl Pessoa {
    pub fn new(
        ano_de_nascimento: u32
    ) -> Self {
        let ano_atual = AnoAtual::new();
        let ano_atual = ano_atual.get_ano() as u32;

        let idade = (
            ano_atual - ano_de_nascimento
        ) as u8;

        let é_maior_de_idade = if idade >= 18 {
            true
        } else {
            false
        };
        
        Self {
            _ano_de_nascimento: ano_de_nascimento,
            idade,
            é_maior_de_idade
        }
    }

    pub fn get_idade(
        &self
    ) -> u8 {
        return self.idade;
    }

    pub fn get_é_maior_de_idade(
        &self
    ) -> bool {
        return self.é_maior_de_idade;
    }
}
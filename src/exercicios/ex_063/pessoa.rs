use crate::exercicios::ex_063::genero::Gênero;

#[derive(Clone)]
pub struct Pessoa {
    idade: u8,
    gênero: Gênero
}

impl Pessoa {
    pub fn new(
        idade: u8,
        gênero: Gênero
    ) -> Self {
        Self {
            idade,
            gênero
        }
    }

    pub fn get_idade(
        &self
    ) -> u8 {
        return self.idade;
    }

    pub fn get_gênero(
        &self
    ) -> Gênero {
        return self.gênero.clone();
    }
}
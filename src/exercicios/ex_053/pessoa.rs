use crate::exercicios::ex_053::generos::Gêneros;

#[derive(Clone)]
pub struct Pessoa {
    nome: String,
    idade: u8,
    gênero: Gêneros,
}

impl Pessoa {
    pub fn new(
        nome: String ,
        idade: u8,
        gênero: Gêneros,
    ) -> Self {        
        Self {
            nome,
            idade,
            gênero
        }
    }

    pub fn get_nome(
        &self
    ) -> String {
        return self.nome.clone();
    }

    pub fn get_idade(
        &self
    ) -> u8 {
        return self.idade;
    }

    pub fn get_gênero(
        &self
    ) -> Gêneros {
        return self.gênero.clone();
    }
}
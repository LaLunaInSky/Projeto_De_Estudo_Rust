use crate::exercicios::ex_063::{
    pessoa::Pessoa,
    genero::Gênero
};

pub struct Pessoas {
    lista_de_pessoas: Vec<Pessoa>,
    quantidade_de_pessoas_maiores_de_18_anos: u8,
    quantidade_de_homens_cadastrados: u8,
    quantidade_de_mulheres_com_menos_de_20_anos: u8
}

impl Pessoas {
    pub fn new() -> Self {
        Self {
            lista_de_pessoas: vec![],
            quantidade_de_pessoas_maiores_de_18_anos: 0,
            quantidade_de_homens_cadastrados: 0,
            quantidade_de_mulheres_com_menos_de_20_anos: 0,
        }
    }

    pub fn adicionar_uma_nova_pessoa(
        &mut self,
        pessoa: Pessoa
    ) {
        if pessoa.get_idade() > 18 {
            self.quantidade_de_pessoas_maiores_de_18_anos += 1;
        }

        if pessoa.get_gênero() == Gênero::MASCULINO {
            self.quantidade_de_homens_cadastrados += 1;
        } else if pessoa.get_idade() < 20 {
            self.quantidade_de_mulheres_com_menos_de_20_anos += 1;
        }

        self.lista_de_pessoas.push(
            pessoa
        );
    }

    pub fn get_lista_de_pessoas(
        &self
    ) -> Vec<Pessoa> {
        return self.lista_de_pessoas.clone();
    }

    pub fn get_quantidade_de_pessoas_maiores_de_18_anos(
        &self
    ) -> u8 {
        return self.quantidade_de_pessoas_maiores_de_18_anos;
    }

    pub fn get_quantidade_de_homens_cadastrados(
        &self
    ) -> u8 {
        return self.quantidade_de_homens_cadastrados;
    }

    pub fn get_quantidade_de_mulheres_com_menos_de_20_anos(
        &self
    ) -> u8 {
        return self.quantidade_de_mulheres_com_menos_de_20_anos;
    }
}
use crate::exercicios::ex_053::{
    pessoa::Pessoa,
    generos::Gêneros
};

pub struct Pessoas {
    lista_de_pessoas: Vec<Pessoa>,
    média_das_idades: u32,
    dados_do_homem_mais_velho: Pessoa,
    lista_de_mulheres_com_menos_de_20_anos: Vec<Pessoa>
}

impl Pessoas {
    pub fn new() -> Self {
        let dados_do_homem_mais_velho = Pessoa::new(
            String::from("fulano de tal"),
            0,
            Gêneros::HOMEM
        );

        Self {
            lista_de_pessoas: vec![],
            média_das_idades: 0,
            dados_do_homem_mais_velho,
            lista_de_mulheres_com_menos_de_20_anos: vec![]
        }
    }

    pub fn adicionar_uma_nova_pessoa(
        &mut self,
        pessoa: Pessoa
    ) {
        if self.lista_de_pessoas.len() == 0 {
            // Calcular a média de idades
            self.média_das_idades = pessoa.get_idade() as u32;

            // Obter os dados do homem mais velho
            if pessoa.get_gênero() == Gêneros::HOMEM {
                self.dados_do_homem_mais_velho = pessoa.clone();
            }

            // Analisar e adicionar na lista de mulheres, caso seja menor que 20 anos
            if pessoa.get_idade() < 20 && pessoa.get_gênero() == Gêneros::MULHER {
                self.lista_de_mulheres_com_menos_de_20_anos.push(
                    pessoa.clone()
                );
            }
        } else {
            // Calcular a média de idades
            let mut soma_das_idades: u32 = 0;

            for pessoa_na_lista in self.get_lista_de_pessoas() {
                soma_das_idades += pessoa_na_lista.get_idade() as u32;
            }

            soma_das_idades += pessoa.get_idade() as u32;

            self.média_das_idades = soma_das_idades / self.get_lista_de_pessoas().len() as u32;

            // Obter os dados do homem mais velho
            if self.get_dados_do_homem_mais_velho().get_idade() < pessoa.get_idade() &&
               pessoa.get_gênero() == Gêneros::HOMEM {
                self.dados_do_homem_mais_velho = pessoa.clone();
            }

            // Analisar e adicionar na lista de mulheres, caso seja menor que 20 anos
            if pessoa.get_idade() < 20 && pessoa.get_gênero() == Gêneros::MULHER {
                self.lista_de_mulheres_com_menos_de_20_anos.push(
                    pessoa.clone()
                );
            }
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

    pub fn get_média_das_idades(
        &self
    ) -> u32 {
        return self.média_das_idades;
    }

    pub fn get_dados_do_homem_mais_velho(
        &self
    ) -> Pessoa {
        return self.dados_do_homem_mais_velho.clone();
    }

    pub fn get_lista_de_mulheres_com_menos_de_20_anos(
        &self
    ) -> Vec<Pessoa> {
        return self.lista_de_mulheres_com_menos_de_20_anos.clone();
    }
}
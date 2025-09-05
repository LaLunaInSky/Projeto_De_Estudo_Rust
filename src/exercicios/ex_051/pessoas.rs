use crate::exercicios::ex_051::pessoa::Pessoa;

pub struct Pessoas {
    _lista_de_pessoas: Vec<Pessoa>,
    quantidade_de_maiores_de_idade: u32,
    quantidade_de_menores_de_idade: u32,
    todas_as_idade: Vec<u8>
}

impl Pessoas {
    pub fn new() -> Self {
        Self {
            _lista_de_pessoas: vec![],
            quantidade_de_maiores_de_idade: 0,
            quantidade_de_menores_de_idade: 0,
            todas_as_idade: vec![]
        }
    }

    pub fn adicionar_uma_nova_pessoa(
        &mut self,
        pessoa: Pessoa
    ) {
        self.todas_as_idade.push(
            pessoa.get_idade()
        );

        if pessoa.get_é_maior_de_idade() {
            self.quantidade_de_maiores_de_idade += 1;
        } else {
            self.quantidade_de_menores_de_idade += 1;
        }

        self._lista_de_pessoas.push(
            pessoa
        );
    }

    pub fn get_quantidade_de_maiores_de_idade(
        &self
    ) -> u32 {
        return self.quantidade_de_maiores_de_idade;
    }

    pub fn get_quantidade_de_menores_de_idade(
        &self
    ) -> u32 {
        return self.quantidade_de_menores_de_idade;
    }

    pub fn get_todas_as_idades(
        &self
    ) -> Vec<u8> {
        return self.todas_as_idade.clone();
    }
}
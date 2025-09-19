use std::collections::HashMap;

pub struct Produtos {
    lista_de_produtos: HashMap<String, f32>
}

impl Produtos {
    pub fn new() -> Self {
        Self {
            lista_de_produtos: HashMap::new()
        }
    }

    pub fn adicionar_um_novo_produto(
        &mut self,
        nome_do_produto: String, 
        preço_do_produto: f32
    ) {
        self.lista_de_produtos.insert(
            nome_do_produto,
            preço_do_produto
        );
    }

    pub fn get_lista_de_produtos(
        &self
    ) -> HashMap<String, f32> {
        return self.lista_de_produtos.clone();
    }
}
use crate::exercicios::ex_064::{
    produto::Produto,
    arredondador_de_precos::arredondadar_preço
};

pub struct Produtos {
    lista_de_produtos: Vec<Produto>,
    preço_total_dos_produtos: f32,
    quantidade_de_produtos_com_o_preço_superior_a_mil: u8,
    dado_do_produto_com_menor_preço: Produto
}

impl Produtos {
    pub fn new() -> Self {
        Self {
            lista_de_produtos: vec![],
            preço_total_dos_produtos: 0.0,
            quantidade_de_produtos_com_o_preço_superior_a_mil: 0,
            dado_do_produto_com_menor_preço: Produto::new(
                String::from("-"),
                0.0
            )
        }
    }

    pub fn adicionar_um_novo_produto(
        &mut self,
        produto: Produto
    ) {
        self.preço_total_dos_produtos += produto.get_preço();

        if produto.get_preço() >= 1000.0 {
            self.quantidade_de_produtos_com_o_preço_superior_a_mil += 1;
        }

        if self.get_lista_de_produtos().len() == 0 {
            self.dado_do_produto_com_menor_preço = produto.clone();
        } else {
            if self.dado_do_produto_com_menor_preço.get_preço() 
                > 
               produto.get_preço() {
                self.dado_do_produto_com_menor_preço = produto.clone();
            }
        }

        self.lista_de_produtos.push(
            produto
        );
    }

    pub fn get_lista_de_produtos(
        &self
    ) -> Vec<Produto> {
        return self.lista_de_produtos.clone();
    }

    pub fn get_quantidade_de_produtos_adicionados(
        &self
    ) -> u8 {
        return self.get_lista_de_produtos().len() as u8;
    }

    pub fn get_preço_total_dos_produtos(
        &self
    ) -> f32 {
        let preço_arredondado = arredondadar_preço(
            self.preço_total_dos_produtos
        );

        return preço_arredondado;
    }

    pub fn get_quantidade_de_produtos_com_o_preço_superior_a_mil(
        &self
    ) -> u8 {
        return self.quantidade_de_produtos_com_o_preço_superior_a_mil;
    }

    pub fn get_dado_do_produto_com_menor_preço(
        &self
    ) -> Produto {
        return self.dado_do_produto_com_menor_preço.clone();
    }
}
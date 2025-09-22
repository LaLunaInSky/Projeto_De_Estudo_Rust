pub struct NúmerosÚnicosEmOrdem {
    lista_de_números_únicos_em_ordem: Vec<u32>
}

impl NúmerosÚnicosEmOrdem {
    pub fn new() -> Self {
        Self {
            lista_de_números_únicos_em_ordem: vec![]
        }
    }

    pub fn adicionar_um_número_novo(
        &mut self,
        número_inteiro: u32
    ) {
        if !self.lista_de_números_únicos_em_ordem.contains(
            &número_inteiro
        ) {
            if self.lista_de_números_únicos_em_ordem.len() > 0 {
                for (
                    index,
                    número
                ) in self.lista_de_números_únicos_em_ordem
                         .clone()
                         .iter()
                         .enumerate() {
                    if número > &número_inteiro {
                        self.lista_de_números_únicos_em_ordem.insert(
                            index, 
                            número_inteiro
                        );

                        break;
                    } else if index == (
                        self.lista_de_números_únicos_em_ordem.len() - 1
                    ) {
                        self.lista_de_números_únicos_em_ordem.push(
                            número_inteiro
                        );

                        break;
                    }
                }
            } else {
                self.lista_de_números_únicos_em_ordem.push(
                    número_inteiro
                );
            }
        }
    }

    pub fn get_lista_de_números_únicos_em_ordem(
        &self
    ) -> Vec<u32> {
        return self.lista_de_números_únicos_em_ordem.clone();
    }
}
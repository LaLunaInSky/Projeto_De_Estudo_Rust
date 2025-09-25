pub struct NúmeroEmOrdemCrescente {
    lista_de_números_em_ordem_crescente: Vec<u32>
}

impl NúmeroEmOrdemCrescente {
    pub fn new() -> Self {
        Self {
            lista_de_números_em_ordem_crescente: vec![]
        }
    }

    pub fn adicionar_um_novo_número(
        &mut self,
        número: u32
    ) {
        if self.lista_de_números_em_ordem_crescente.len() == 0 {
            self.lista_de_números_em_ordem_crescente.push(
                número
            )
        } else {
            if número >= self.lista_de_números_em_ordem_crescente[
                self.lista_de_números_em_ordem_crescente.len() - 1
            ] {
                self.lista_de_números_em_ordem_crescente.push(
                    número
                );
            } else {
                for (
                    index,
                    número_da_lista
                ) in
                    self.lista_de_números_em_ordem_crescente
                    .clone().iter().enumerate() {
                        if número < *número_da_lista {
                            self.lista_de_números_em_ordem_crescente.insert(
                                index, 
                                número
                            );
                            
                            break;
                        }
                }
            }
        }
    }

    pub fn get_lista_de_números_em_ordem_crescente(
        &self
    ) -> Vec<u32> {
        return self.lista_de_números_em_ordem_crescente.clone();
    }
}

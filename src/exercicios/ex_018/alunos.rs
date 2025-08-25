use rand::random_range;

pub struct Alunos {
    pub lista_de_nomes: Vec<String>,
}

impl Alunos {
    pub fn new() -> Self {
        Self {
            lista_de_nomes: vec![]
        }
    }

    pub fn adicionar_um_novo_aluno(
        &mut self,
        nome_do_aluno: String
    ) {
        self.lista_de_nomes.push(
            nome_do_aluno
        );
    }

    pub fn sortear_ordem_de_apresentação(
        &self
    ) -> Vec<String> {
        let mut número_da_ordem_sorteada: Vec<usize> = vec![];

        let lista_de_nomes = self.lista_de_nomes.clone();
        
        let quantidade_de_nomes = lista_de_nomes.len();

        let mut lista_dos_nomes_em_ordem: Vec<String> = vec![];

        for indice in 0..quantidade_de_nomes {
            loop {
                let número_sorteado: usize = random_range(
                    0..quantidade_de_nomes
                );

                if indice == 0 {
                    número_da_ordem_sorteada.push(
                        número_sorteado
                    );

                    break;
                } else {
                    if número_da_ordem_sorteada.contains(&número_sorteado) == false {
                        número_da_ordem_sorteada.push(
                            número_sorteado
                        );

                        break;
                    }

                }
            }
        }

        for index in número_da_ordem_sorteada {
            lista_dos_nomes_em_ordem.push(
                lista_de_nomes[index].clone()
            );
        }

        return lista_dos_nomes_em_ordem;
    }
}
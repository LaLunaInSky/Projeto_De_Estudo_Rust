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

    pub fn adicionar_novo_aluno(
        &mut self,
        nome_do_aluno: String
    ) {
        self.lista_de_nomes.push(
            nome_do_aluno
        );
    }

    pub fn sortear_nome_de_um_aluno(
        &self
    ) -> String {
        let quantidade_de_alunos = self.lista_de_nomes.len();
        
        let número_sorteado: usize = random_range(0..quantidade_de_alunos);
        
        let lista_de_nomes = &self.lista_de_nomes;

        return lista_de_nomes[número_sorteado].clone();
    }
}
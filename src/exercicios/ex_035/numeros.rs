pub struct Números {
    lista_de_números: [u32; 2],
    número_maior: String
}

impl Números {
    pub fn new() -> Self {
        Self {
            lista_de_números: [0, 0],
            número_maior: String::new()
        }
    }

    pub fn adicionar_um_número_a_lista(
        &mut self,
        número: u32,
        indice_da_chamada: usize
    ) {
        self.lista_de_números[indice_da_chamada] = número;
    }

    fn analisar_qual_é_maior_número(
        &mut self
    ) {
        let mut _resposta = String::new();

        if self.lista_de_números[0] > self.lista_de_números[1] {
            _resposta = String::from("O \"Primeiro\" número é o MAIOR!");

        } else if self.lista_de_números[0] < self.lista_de_números[1] {
            _resposta = String::from("O \"Segundo\" número é o MAIOR!");

        } else {
            _resposta = String::from("Não existe número maior!\nAmbos são iguais!");
        }

        self.número_maior = _resposta;
    }

    pub fn get_lista_de_números(
        &self
    ) -> [u32; 2] {
        return self.lista_de_números.clone();
    }

    pub fn get_número_maior(
        &mut self
    ) -> String {
        self.analisar_qual_é_maior_número();

        return self.número_maior.clone();
    }
}
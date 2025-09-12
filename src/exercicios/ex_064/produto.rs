#[derive(Clone)]
pub struct Produto {
    nome: String,
    preço: f32
}

impl Produto {
    pub fn new(
        nome: String,
        preço: f32
    ) -> Self {
        Self {
            nome,
            preço
        }
    }

    pub fn get_nome(
        &self
    ) -> String {
        return self.nome.clone();
    }

    pub fn get_preço(
        &self
    ) -> f32 {
        return self.preço;
    }
}
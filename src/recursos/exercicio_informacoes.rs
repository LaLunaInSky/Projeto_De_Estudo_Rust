pub struct Exercício_Informações {
    cabeçalho: String,
    descrição: String
}

impl Exercício_Informações {
    pub fn new(
        cabeçalho: &String,
        descrição: String
    ) -> Self {
        Self {
            cabeçalho: cabeçalho.clone(),
            descrição
        }
    }

    pub fn mostrar_informações(&self) {
        println!(
            "{}\n{}",
            self.cabeçalho,
            self.descrição
        );
    }
}
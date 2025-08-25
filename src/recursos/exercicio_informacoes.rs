pub struct ExercícioInformações {
    cabeçalho: String,
    descrição: String
}

impl ExercícioInformações {
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
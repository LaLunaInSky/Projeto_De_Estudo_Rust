pub struct NomeCompleto {
    _nome: String,
    pub primeiro_nome: String,
    pub último_nome: String,
}

impl NomeCompleto {
    pub fn new(
        nome: String
    ) -> Self {
        let mut primeiro_nome = String::from("-");
        let mut último_nome = String::from("-");

        let nome_separado: Vec<&str> = nome.split_whitespace().collect();

        for (index, parte_do_nome) in nome_separado.iter().enumerate() {
            if index == 0 {
                primeiro_nome = parte_do_nome.to_string();
            } else if index == (nome_separado.len() - 1) {
                último_nome = parte_do_nome.to_string();
            }
        }

        Self {
            _nome: nome,
            primeiro_nome,
            último_nome
        }
    }
}
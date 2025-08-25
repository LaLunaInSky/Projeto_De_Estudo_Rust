pub struct Nome {
    nome_digitado: String
}

impl Nome {
    pub fn new(
        nome_digitado: String
    ) -> Self {
        Self {
            nome_digitado
        }
    }

    pub fn obter_o_nome_em_minúscula(
        &self
    ) -> String {
        let nome_em_minúscula = self.nome_digitado.to_lowercase();

        return nome_em_minúscula;
    }

    pub fn obter_o_nome_em_maiúscula(
        &self
    ) -> String {
        let nome_em_maiúscula = self.nome_digitado.to_uppercase();

        return nome_em_maiúscula;
    }

    pub fn obter_o_total_de_letras_no_nome(
        &self
    ) -> u32 {
        let mut total_de_letras: u32 = 0;

        for char in self.nome_digitado.chars() {
            if char != ' ' {
                total_de_letras += 1;
            }
        }

        return total_de_letras;
    }
    
    pub fn obter_o_total_de_letras_do_primeiro_nome(
        &self
    ) ->u32 {
        let mut total_letras: u32 = 0;

        for char in self.nome_digitado.chars() {
            if char == ' ' {
                break;
            } else {
                total_letras += 1;
            }
        }

        return total_letras;
    }
}

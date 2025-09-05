use chrono::Utc;

pub struct AnoAtual {
    ano: u16
}

impl AnoAtual {
    pub fn new() -> Self {
        let utc = Utc::now().to_string();
        let mut ano_separado: Vec<char> = vec![];

        for (index, char) in utc.chars().enumerate() {
            if index <= 3 {
                ano_separado.push(char);
            }
        }

        let ano_atual = format!(
            "{}{}{}{}",
            ano_separado[0],
            ano_separado[1],
            ano_separado[2],
            ano_separado[3]
        );

        let ano: u16 = ano_atual.parse().unwrap();
        
        Self {
            ano
        }
    }

    pub fn get_ano(
        &self
    ) -> u16 {
        return self.ano;
    }
}
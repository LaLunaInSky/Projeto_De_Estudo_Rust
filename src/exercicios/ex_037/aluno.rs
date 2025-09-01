pub struct Aluno {
    notas: Vec<f32>,
    média: f32,
    situação_final: String
}

impl Aluno {
    pub fn new() -> Self {
        Self {
            notas: vec![],
            média: 0.0,
            situação_final: String::from("-") 
        }
    }

    pub fn adicionar_uma_nota(
        &mut self,
        nota: f32
    ) {
        self.notas.push(
            nota
        );
    }

    pub fn calcular_a_média(
        &mut self
    ) {
        let mut soma_das_notas: f32 = 0.0;
    
        for nota in &self.notas {
            soma_das_notas += nota;
        }

        let total_de_notas = self.notas.len()
            .to_string()
            .parse::<f32>()
            .unwrap();

        let média = soma_das_notas / total_de_notas;
        let média = format!(
            "{:.1}",
            média
        );

        let média: f32 = média.parse().unwrap();

        self.média = média;

        self.verificar_situação_final();
    }

    fn verificar_situação_final(
        &mut self
    ) {
        let mut situação_final = String::from("REPROVADO");

        if self.média >= 7.0 && self.média <= 10.0 {
            situação_final = String::from("APROVADO");
        } else if self.média >= 5.0 {
            situação_final = String::from("RECUPERAÇÃO");
        }

        self.situação_final = situação_final;
    }

    pub fn get_notas(
        &self
    ) -> Vec<f32> {
        return self.notas.clone();
    }

    pub fn get_média(
        &self
    ) -> f32 {
        return self.média;
    }

    pub fn get_situação_final(
        &self
    ) -> String {
        return self.situação_final.clone();
    }
}
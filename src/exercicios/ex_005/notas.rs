pub struct Notas {
    pub notas: Vec<f32>,
    pub média_final: f32
}

impl Notas {
    pub fn new(
        notas: Vec<f32>
    ) -> Self {
        let mut soma_das_notas: f32 = 0.0;

        for nota in &notas {
            soma_das_notas += nota;
        }

        let média = soma_das_notas / (notas.len() as f32);

        let média = format!(
            "{:.1}",
            média
        );

        match média.parse::<f32>() {
            Ok(média_final) => {
                return Self {
                    notas,
                    média_final
                };
            }
            Err(_) => (),
        }

        Self {
            notas,
            média_final: 0.0
        }
    }
}
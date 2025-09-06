pub struct Pesos {
    pesos: Vec<f32>,
    maior_peso: f32,
    menor_peso: f32,
}

impl Pesos {
    pub fn new() -> Self {
        Self {
            pesos: vec![],
            maior_peso: 0.0,
            menor_peso: 0.0
        }
    }

    pub fn adicionar_novo_peso(
        &mut self,
        peso: f32
    ) {
        if self.pesos.len() == 0 {
            self.maior_peso = peso;
            self.menor_peso = peso;
        } else {
            if self.menor_peso > peso {
                self.menor_peso = peso
            }

            if self.maior_peso < peso {
                self.maior_peso = peso
            }
        }

        self.pesos.push(
            peso
        );
    }

    pub fn get_pesos(
        &self
    ) -> Vec<f32> {
        return self.pesos.clone();
    }

    pub fn get_maior_peso(
        &self
    ) -> f32 {
        return self.maior_peso;
    }

    pub fn get_menor_peso(
        &self
    ) -> f32 {
        return self.menor_peso;
    }
}
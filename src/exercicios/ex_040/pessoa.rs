use crate::exercicios::ex_040::arredondador_de_numeros_reais::arrendondar_um_número_real;

pub struct Pessoa {
    peso: f32,
    altura: f32,
    imc: f32,
    status_corporal: String, 
}

impl Pessoa {
    pub fn new(peso: f32, altura: f32) -> Self {
        let imc = arrendondar_um_número_real(
            peso / (altura * altura),
            1
        );

        let status_corporal: String = if imc > 40.0 {
            String::from("Obesidade Mórbida")
        } else if imc > 30.0 {
            String::from("Obesidade")
        } else if imc > 25.0 {
            String::from("Sobrepeso")
        } else if imc > 18.5 {
            String::from("Peso Ideal")
        } else {
            String::from("Abaixo do peso")
        };
        
        Self {
            peso,
            altura,
            imc,
            status_corporal
        }
    }

    pub fn get_peso(
        &self
    ) -> f32 {
        return self.peso;
    }

    pub fn get_altura(
        &self
    ) -> f32 {
        return self.altura;
    }

    pub fn get_imc(
        &self
    ) -> f32 {
        return self.imc;
    }

    pub fn get_status_corporal(
        &self
    ) -> String {
        return self.status_corporal.clone();
    }
}
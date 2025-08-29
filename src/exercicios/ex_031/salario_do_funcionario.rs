pub struct SalárioDoFuncionário {
    _salário_atual: f32,
    salário_novo: f32,
    taxa_de_aumento: f32
}

impl SalárioDoFuncionário {
    pub fn new(
        salário_atual: f32
    ) -> Self {
        fn calcular_o_aumento_de_x_porcento(
            salário_atual: &f32, 
            taxa_de_aumento: &f32
        ) -> f32 {
            let novo_salário = salário_atual + (
                salário_atual * (taxa_de_aumento / 100.0)
            );

            return novo_salário;
        }

        let mut _taxa_de_aumento: f32 = 0.0;
        let mut _salário_novo: f32 = 0.0;

        if salário_atual <= 1250.0 {
            _taxa_de_aumento = 15.0;

            _salário_novo = calcular_o_aumento_de_x_porcento(
                &salário_atual, 
                &_taxa_de_aumento
            );

        } else {
            _taxa_de_aumento = 10.0;
            
            _salário_novo = calcular_o_aumento_de_x_porcento(
                &salário_atual, 
                &_taxa_de_aumento
            ); 
        }

        Self {
            _salário_atual: salário_atual,
            taxa_de_aumento: _taxa_de_aumento,
            salário_novo: _salário_novo
        }
    }

    pub fn get_taxa_de_aumento(
        &self
    ) -> f32 {
        return self.taxa_de_aumento;
    }

    pub fn get_salário_novo(
        &self
    ) -> f32 {
        return self.salário_novo;
    }
}
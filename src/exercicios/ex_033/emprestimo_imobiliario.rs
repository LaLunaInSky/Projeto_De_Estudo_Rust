pub struct EmpréstimoImobiliário {
    _valor_da_casa: f32,
    _salário_do_comprador: f32,
    _anos_à_pagar: u8,
    empréstimo_liberado: bool
}

impl EmpréstimoImobiliário {
    pub fn new(
        valor_da_casa: f32,
        salário_do_comprador: f32,
        anos_à_pagar: u8
    ) -> Self {
        let mut empréstimo_liberado = false;

        let quantidade_de_anos_para_pagar = anos_à_pagar as f32;

        let trinta_porcento_do_salário: f32 = salário_do_comprador * (30.0 / 100.0);

        let prestação_mensal_da_casa: f32 = valor_da_casa / (quantidade_de_anos_para_pagar * 12.0);

        if prestação_mensal_da_casa < trinta_porcento_do_salário {
            empréstimo_liberado = true;
        }

        Self {
            _valor_da_casa: valor_da_casa,
            _salário_do_comprador: salário_do_comprador,
            _anos_à_pagar: anos_à_pagar,
            empréstimo_liberado
        }
    }

    pub fn get_empréstimo_liberado(
        &self
    ) -> bool {
        return self.empréstimo_liberado;
    }
}
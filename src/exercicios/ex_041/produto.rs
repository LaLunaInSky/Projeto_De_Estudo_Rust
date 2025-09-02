use crate::exercicios::ex_041::arredondador_de_numeros_reais::arrendondar_um_número_real;

pub struct Produto {
    preço_do_produto: f32,
    preço_no_pix: f32,
    preço_no_débito: f32,
    preço_no_crédito_2_vezes: f32,
    preço_no_crédito_x_vezes: [f32; 2],
}

impl Produto {
    pub fn new(
        preço_do_produto: f32
    ) -> Self {

        fn calcular_preço_no_pix(
            preço_do_produto: &f32
        ) -> f32 {
            let valor_com_10_de_desconto = preço_do_produto - (preço_do_produto * (10.0 / 100.0));

            let valor_com_10_de_desconto = arrendondar_um_número_real(
                valor_com_10_de_desconto,
                2
            );

            return valor_com_10_de_desconto;
        }

        fn calcular_preço_no_débito(
            preço_do_produto: &f32
        ) -> f32 {
            let valor_com_5_porcento_de_desconto = preço_do_produto - (
                preço_do_produto * (5.0 / 100.0)
            );

            let valor_com_5_porcento_de_desconto = arrendondar_um_número_real(
                valor_com_5_porcento_de_desconto, 
                2
            );

            return valor_com_5_porcento_de_desconto;
        }

        fn calcular_preço_no_crédito_2_vezes(
            preço_do_produto: &f32
        ) -> f32 {
            let valor_de_cada_parcela = preço_do_produto / 2.0;

            let valor_de_cada_parcela = arrendondar_um_número_real(
                valor_de_cada_parcela,
                2
            );

            return valor_de_cada_parcela;
        }

        let preço_no_pix: f32 = calcular_preço_no_pix(
            &preço_do_produto
        );
        
        let preço_no_débito: f32 = calcular_preço_no_débito(
            &preço_do_produto
        );

        let preço_no_crédito_2_vezes: f32 = calcular_preço_no_crédito_2_vezes(
            &preço_do_produto
        );

        Self {
            preço_do_produto,
            preço_no_pix,
            preço_no_débito,
            preço_no_crédito_2_vezes,
            preço_no_crédito_x_vezes: [0.0, 0.0],
        }
    }

    pub fn calcular_preço_no_credito_x_vezes(
        &mut self,
        preço_do_produto: &f32, 
        quantidade_de_parcelas: u8
    ) {
        let valor_final_com_20_de_juros = preço_do_produto + (
            preço_do_produto * (20.0 / 100.0)
        );

        let valor_de_cada_parcela = valor_final_com_20_de_juros / (quantidade_de_parcelas as f32);

        let valor_final_com_20_de_juros = arrendondar_um_número_real(
            valor_final_com_20_de_juros,
            2
        );

        let valor_de_cada_parcela = arrendondar_um_número_real(
            valor_de_cada_parcela,
            2
        );

        self.preço_no_crédito_x_vezes = [
            valor_final_com_20_de_juros,
            valor_de_cada_parcela
        ];
    }

    pub fn get_preço_do_produto(
        &self
    ) -> f32 {
        return self.preço_do_produto;
    }

    pub fn get_preço_no_pix(
        &self
    ) -> f32 {
        return self.preço_no_pix;
    }

    pub fn get_preço_no_débito(
        &self
    ) -> f32 {
        return self.preço_no_débito;
    }

    pub fn get_preço_no_crédito_2_vezes(
        &self
    ) -> f32 {
        return self.preço_no_crédito_2_vezes;
    }

    pub fn get_preço_no_crédito_x_vezes(
        &self
    ) -> [f32; 2] {
        return self.preço_no_crédito_x_vezes;
    }
}
pub struct NúmeroDigitados {
    lista_de_números_digitados: [u32; 4],
    quantidade_de_vezes_que_o_nove_apareceu: u8,
    posição_que_o_valor_3_apareceu_pela_primeira_vez: i8,
    lista_de_números_pares_digitados: Vec<u32>
}

impl NúmeroDigitados {
    pub fn new() -> Self {
        Self {
            lista_de_números_digitados: [0,0,0,0],
            quantidade_de_vezes_que_o_nove_apareceu: 0,
            posição_que_o_valor_3_apareceu_pela_primeira_vez: -1,
            lista_de_números_pares_digitados: vec![]
        }
    }

    pub fn adicionar_um_número_a_lista(
        &mut self,
        número_digitado: u32,
        posição_do_número_na_lista: usize
    ) {
        self.lista_de_números_digitados[
            posição_do_número_na_lista
        ] = número_digitado;

        if número_digitado == 3 
            &&
           self.posição_que_o_valor_3_apareceu_pela_primeira_vez == -1 
        {
            self.posição_que_o_valor_3_apareceu_pela_primeira_vez = posição_do_número_na_lista as i8;
        }

        if número_digitado == 9 {
            self.quantidade_de_vezes_que_o_nove_apareceu += 1;
        }

        if número_digitado % 2 == 0 {
            self.lista_de_números_pares_digitados.push(
                número_digitado
            );
        }
    }

    pub fn get_lista_de_números_digitados(
        &self
    ) -> [u32; 4] {
        return self.lista_de_números_digitados.clone();
    }

    pub fn get_quantidade_de_vezes_que_o_nove_apareceu(
        &self
    ) -> u8 {
        return self.quantidade_de_vezes_que_o_nove_apareceu;
    }

    pub fn get_posição_que_o_valor_3_apareceu_pela_primeira_vez(
        &self
    ) -> String {
        if self.posição_que_o_valor_3_apareceu_pela_primeira_vez != -1 {
            return format!(
                "na {}ª posição",
                self.posição_que_o_valor_3_apareceu_pela_primeira_vez + 1
            );
        } else {
            return String::from(
                "em nenhuma posição"
            );
        }
    }

    pub fn get_lista_de_números_pares_digitados(
        &self
    ) -> Vec<u32> {
        return self.lista_de_números_pares_digitados.clone();
    }
}
use rand::random_range;

pub struct SorteadorDeNúmeros {
    lista_de_números_sorteados: [u8; 5],
    menor_número_sorteado: u8,
    maior_número_sorteado: u8
}

impl SorteadorDeNúmeros {
    pub fn new() -> Self {
        let mut lista_de_números_sorteados: [u8; 5] = [
            0,0,0,0,0
        ];

        let mut menor_número_sorteado: u8 = 0;
        let mut maior_número_sorteado: u8 = 0;

        for index in 0..5 {
            lista_de_números_sorteados[
                index
            ] = random_range(
                1..11
            );
        }

        for (
            index,
            número
        ) in lista_de_números_sorteados.iter().enumerate() {
            if index == 0 {
                menor_número_sorteado = *número;
                maior_número_sorteado = *número;
            } else {
                if menor_número_sorteado > *número {
                    menor_número_sorteado = *número;
                }

                if maior_número_sorteado < *número {
                    maior_número_sorteado = *número;
                }
            }
        }

        Self {
            lista_de_números_sorteados,
            menor_número_sorteado,
            maior_número_sorteado
        }
    }

    pub fn get_lista_de_números_sorteados(
        &self
    ) -> [u8; 5] {
        return self.lista_de_números_sorteados.clone();
    }

    pub fn get_menor_número_sorteado(
        &self
    ) -> u8 {
        return self.menor_número_sorteado;
    }

    pub fn get_maior_número_sorteado(
        &self
    ) -> u8 {
        return self.maior_número_sorteado;
    }
}
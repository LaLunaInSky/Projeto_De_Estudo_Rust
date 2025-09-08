pub struct Números {
    números: [u32; 2],
    soma: u32,
    multiplicação: u32,
    número_maior: u32,
}

impl Números {
    pub fn new(
        números: [u32; 2]
    ) -> Self {
        fn somar_números(
            números: &[u32; 2]
        ) -> u32 {
            let mut soma: u32 = 0;

            for número in números {
                soma += número
            };

            return soma;
        }

        fn multiplicar_números(
            números: &[u32; 2]
        ) -> u32 {
            let mut mulplicação: u32 = 0;

            for (index, número) in números.iter().enumerate() {
                if index == 0 {
                    mulplicação += número;
                } else {
                    mulplicação *= número;
                }
            }

            return mulplicação;
        }

        fn obter_o_número_maior(
            números: &[u32; 2]
        ) -> u32 {
            let mut o_número_maior: u32 = 0;

            for (index, número) in números.iter().enumerate() {
                if index == 0 {
                    o_número_maior = *número;
                } else {
                    if *número > o_número_maior {
                        o_número_maior = *número;
                    }
                }
            }

            return o_número_maior;
        }

        let soma: u32 = somar_números(
            &números
        );

        let multiplicação: u32 = multiplicar_números(
            &números
        );

        let número_maior: u32 = obter_o_número_maior(
            &números
        );

        Self {
            números,
            soma,
            multiplicação,
            número_maior
        }
    }

    pub fn get_números(
        &self
    ) -> [u32; 2] {
        return self.números.clone();
    }

    pub fn get_soma(
        &self
    ) -> u32 {
        return self.soma;
    }

    pub fn get_multiplicação(
        &self
    ) -> u32 {
        return self.multiplicação;
    }

    pub fn get_número_maior(
        &self
    ) -> u32 {
        return self.número_maior;
    }
}
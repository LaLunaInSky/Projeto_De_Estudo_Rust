pub struct PA {
    primeiro_termo: u32,
    razão: u32,
    dez_primeiros_termos: Vec<u32>,
}

impl PA {
    pub fn new(
        primeiro_termo: u32,
        razão: u32
    ) -> Self {
        fn obter_os_dez_primeiros_termos(
            primeiro_termo: &u32,
            razão: &u32
        ) -> Vec<u32> {
            let mut dez_primeiros_termos: Vec<u32> = vec![];

            for count in 1..11 {
                if count == 1 {
                    dez_primeiros_termos.push(
                        *primeiro_termo
                    );
                } else {
                    let próximo_termo = dez_primeiros_termos[
                        count - 2
                    ] + *razão;
    
                    dez_primeiros_termos.push(
                        próximo_termo
                    );
                }
            }

            return dez_primeiros_termos;
        }

        let dez_primeiros_termos = obter_os_dez_primeiros_termos(
            &primeiro_termo, 
            &razão
        );

        Self {
            primeiro_termo,
            razão,
            dez_primeiros_termos
        }
    }

    pub fn get_primeiro_termo(
        &self
    ) -> u32 {
        return self.primeiro_termo;
    }

    pub fn get_razão(
        &self
    ) -> u32 {
        return self.razão;
    }

    pub fn get_dez_primeiros_termos(
        &self
    ) -> Vec<u32> {
        return self.dez_primeiros_termos.clone();
    }
}
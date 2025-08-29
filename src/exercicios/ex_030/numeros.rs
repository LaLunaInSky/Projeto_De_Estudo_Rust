pub struct Números {
    lista_de_números: Vec<u32>,
    número_maior: u32,
    número_menor: u32
}

impl Números {
    pub fn new() -> Self {
        Self {
            lista_de_números: vec![],
            número_maior: 0,
            número_menor: 0
        }
    }

    pub fn adicionar_um_número_na_lista(
        &mut self,
        número: u32
    ) {
        if self.lista_de_números.len() == 0 {
            self.número_menor = número;
        } else {
            if número < self.número_menor {
                self.número_menor = número;
            }
        }

        if número > self.número_maior {
            self.número_maior = número;
        }

        self.lista_de_números.push(
            número
        );
    }

    pub fn get_lista_de_números(
        &self
    ) -> Vec<u32> {
        return self.lista_de_números.clone();
    }

    pub fn get_número_maior(
        &self
    ) -> u32 {
        return self.número_maior
    }

    pub fn get_número_menor(
        &self
    ) -> u32 {
        return self.número_menor;
    }
}

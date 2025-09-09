pub struct Números {
    lista_de_números: Vec<u32>,
    média_dos_números: u32,
    número_maior: u32,
    número_menor: u32
}

impl Números {
    pub fn new() -> Self {
        Self {
            lista_de_números: vec![],
            média_dos_números: 0,
            número_maior: 0,
            número_menor: 0
        }
    }

    pub fn adicionar_um_novo_número(
        &mut self,
        número: u32
    ) {
        if self.lista_de_números.len() == 0 {
            self.número_maior = número;

            self.número_menor = número;
            
        } else {
            if self.número_menor > número {
                self.número_menor = número
            }
            
            if self.número_maior < número {
                self.número_maior = número
            }
        }

        self.lista_de_números.push(número);
    }

    pub fn calcular_a_média_dos_números(
        &mut self
    ) {
        let mut soma_dos_números: u32 = 0;

        let lista_de_números  = self.lista_de_números.clone();
        
        let total_de_números: u32 = lista_de_números.len() as u32;

        for número in lista_de_números {
            soma_dos_números += número;
        }

        self.média_dos_números = soma_dos_números / total_de_números;
    }

    pub fn get_lista_de_números(
        &self
    ) -> Vec<u32> {
        return self.lista_de_números.clone();
    }

    pub fn get_média_dos_números(
        &self
    ) -> u32 {
        return self.média_dos_números;
    }

    pub fn get_número_maior(
        &self
    ) -> u32 {
        return self.número_maior;
    }

    pub fn get_número_menor(
        &self
    ) -> u32 {
        return self.número_menor;
    }
}
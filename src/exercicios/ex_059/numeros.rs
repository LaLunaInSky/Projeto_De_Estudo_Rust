pub struct Números {
    números: Vec<u32>,
    soma_dos_números: u32
}

impl Números {
    pub fn new() -> Self {
        Self {
            números: vec![],
            soma_dos_números: 0,
        }
    }

    pub fn adicionar_número(
        &mut self,
        número: u32
    ) {
        self.números.push(
            número
        );
    }

    pub fn somar_números(
        &mut self
    ) {
        let mut soma_dos_números: u32 = 0;
        let números = self.números.clone();

        for número in números {
            soma_dos_números += número;
        }

        self.soma_dos_números = soma_dos_números;
    }

    pub fn retornar_quantidade_de_números_armazenados(
        &self
    ) -> u32 {
        return self.números.len() as u32;
    }

    pub fn get_soma_dos_números(
        &self
    ) -> u32 {
        return self.soma_dos_números;
    }

    pub fn get_números(
        &self
    ) -> Vec<u32> {
        return self.números.clone();
    }
}
pub struct NúmerosÍmpares {
    lista_de_números_ímpares: Vec<u32>,
    soma_dos_números_ímpares: u32
}

impl NúmerosÍmpares {
    pub fn new(
        números_ímpares: Vec<u32>
    ) -> Self {
        let mut soma_dos_números: u32 = 0;

        for número in &números_ímpares {
            soma_dos_números += número;
        }

        Self {
            lista_de_números_ímpares: números_ímpares,
            soma_dos_números_ímpares: soma_dos_números
        }
    }

    pub fn get_lista_de_números_ímpares(
        &self
    ) -> Vec<u32> {
        return self.lista_de_números_ímpares.clone();
    }

    pub fn get_soma_dos_números_ímpares(
        &self
    ) -> u32 {
        return self.soma_dos_números_ímpares;
    }
}
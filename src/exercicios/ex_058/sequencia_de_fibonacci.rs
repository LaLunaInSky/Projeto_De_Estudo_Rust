pub struct SequênciaDeFibonacci {
    penúltimo_elemento: u32,
    último_elemento: u32,
    próximo_elemento: u32,
}

impl SequênciaDeFibonacci{
    pub fn new() -> Self {
        let penúltimo_elemento = 0;
        let último_elemento = 1;
        let próximo_elemento = penúltimo_elemento + último_elemento;

        Self {
            penúltimo_elemento,
            último_elemento,
            próximo_elemento
        }
    }

    pub fn obter_o_próximo_elemento(
        &mut self
    ) {
        self.penúltimo_elemento = self.último_elemento;
        self.último_elemento = self.próximo_elemento;
        self.próximo_elemento = self.penúltimo_elemento + self.último_elemento;
    }

    pub fn get_penúltimo_elemento(
        &self
    ) -> u32 {
        return self.penúltimo_elemento;
    }
}
use rand::random_range;

pub struct Jogadas {
    número_do_computador: u8,
    tentativas: u8,
}

impl Jogadas {
    pub fn new() -> Self {
        let número_do_computador: u8 = random_range(0..11);
        
        Self {
            número_do_computador,
            tentativas: 0
        }
    }

    pub fn adicionar_mais_uma_tentativa(
        &mut self
    ) {
        self.tentativas += 1;
    }

    pub fn get_número_do_computador(
        &self
    ) -> u8 {
        return self.número_do_computador;
    }

    pub fn get_tentativas(
        &self
    ) -> u8 {
        return self.tentativas;
    }
}
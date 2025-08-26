use rand::random_range;

pub struct Jogo {
    pub computador_número: u8,
    pub usuário_número: u8,
    pub usuário_acertou: bool
}

impl Jogo {
    pub fn new(
        usuário_número: u8
    ) -> Self {
        /* Sortear o número do computador */
        let computador_número: u8 = random_range(0..6);

        /* Analisar se o usuário acertou */
        let mut usuário_acertou = false;

        if usuário_número == computador_número {
            usuário_acertou = true;
        }        

        Self {
            computador_número,
            usuário_número,
            usuário_acertou
        }
    }
}
pub struct Número {
    _número: u16,
    pub unidade: String,
    pub dezena: String,
    pub centena: String,
    pub milhar: String
}

impl Número {
    pub fn new(
        número: u16
    ) -> Self {
        let número_string = format!(
            "{}",
            número
        );

        let mut número_separado: Vec<char> = vec![];

        for char in número_string.chars() {
            número_separado.push(
                char
            );
        }

        let mut unidade = String::from("-");
        let mut dezena = String::from("-");
        let mut centena = String::from("-");
        let mut milhar = String::from("-");

        if número_separado.len() == 4 {
            unidade = número_separado[3].to_string();
            dezena = número_separado[2].to_string();
            centena = número_separado[1].to_string();
            milhar = número_separado[0].to_string();

        } else if número_separado.len() == 3 {
            unidade = número_separado[2].to_string();
            dezena = número_separado[1].to_string();
            centena = número_separado[0].to_string();

        } else if número_separado.len() == 2 {
            unidade = número_separado[1].to_string();
            dezena = número_separado[0].to_string();

        } else if número_separado.len() == 1 {
            unidade = número_separado[0].to_string();
        }

        Self {
            _número: número,
            unidade,
            dezena,
            centena,
            milhar
        }
    }
}
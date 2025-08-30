use crate::exercicios::ex_036::ano_atual::AnoAtual;

pub struct Pessoa {
    _idade: u8,
    situação_do_alistamento_militar: String
}

impl Pessoa {
    pub fn new(
        ano_de_nascimento: u32
    ) -> Self {
        let ano_atual = AnoAtual::new();
        let ano_atual = ano_atual.get_ano();

        let idade = (ano_atual as u32 - ano_de_nascimento) as u8;
        let mut _situação_do_alistamento_militar = String::new();

        if idade > 18 {
            _situação_do_alistamento_militar = format!(
                "Você já passou do ano do seu alistamento!\nJá se passou {} ano{}.",
                idade - 18,
                if idade - 18 > 1 {"s"} else {""}
            );
        } else if idade < 18 {
            _situação_do_alistamento_militar = format!(
                "Você ainda não chegou no ano do seu\nalistamento! Falta{} {} ano{}!",
                if 18 - idade != 1 {"m"} else {""},
                18 - idade,
                if 18 - idade != 1 {"s"} else {""}
            );
        } else {
            _situação_do_alistamento_militar = format!(
                "Você está no ano do seu alistamento!"
            );
        }

        Self {
            _idade: idade,
            situação_do_alistamento_militar: _situação_do_alistamento_militar
        }
    }

    pub fn get_situação_do_alistamento_militar(
        &self
    ) -> String {
        return self.situação_do_alistamento_militar.clone();
    }
}
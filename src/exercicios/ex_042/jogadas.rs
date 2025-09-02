use rand::random_range;

use crate::exercicios::ex_042::{
    opcoes_de_jogadas::OpçõesDeJogadas,
    possiveis_ganhadores::PossíveisGanhadores
};

pub struct Jogadas {
    jogador: OpçõesDeJogadas,
    computador: OpçõesDeJogadas,
    ganhador: PossíveisGanhadores,
}

impl Jogadas {
    pub fn new(
        jogador: OpçõesDeJogadas,
    ) -> Self {
        fn sortear_a_jogado_do_computador() -> OpçõesDeJogadas {
            let sortear_número = random_range(0..3);

            match sortear_número {
                0 => OpçõesDeJogadas::PAPEL,
                1 => OpçõesDeJogadas::PEDRA,
                2 => OpçõesDeJogadas::TESOURA,
                _ => OpçõesDeJogadas::PAPEL,
            }
        }

        let computador = sortear_a_jogado_do_computador();

        let ganhador = match jogador {
            OpçõesDeJogadas::PEDRA => {
                match computador {
                    OpçõesDeJogadas::PEDRA => PossíveisGanhadores::EMPATE,
                    OpçõesDeJogadas::PAPEL => PossíveisGanhadores::COMPUTADOR,
                    OpçõesDeJogadas::TESOURA => PossíveisGanhadores::JOGADOR
                }
            }
            OpçõesDeJogadas::PAPEL => {
                match computador {
                    OpçõesDeJogadas::PEDRA => PossíveisGanhadores::JOGADOR,
                    OpçõesDeJogadas::PAPEL => PossíveisGanhadores::EMPATE,
                    OpçõesDeJogadas::TESOURA => PossíveisGanhadores::COMPUTADOR
                }
            },
            OpçõesDeJogadas::TESOURA => {
                match computador {
                    OpçõesDeJogadas::PEDRA => PossíveisGanhadores::COMPUTADOR,
                    OpçõesDeJogadas::PAPEL => PossíveisGanhadores::JOGADOR,
                    OpçõesDeJogadas::TESOURA => PossíveisGanhadores::EMPATE
                }
            }
        };

        Self {
            jogador,
            computador,
            ganhador
        }
    }

    pub fn get_jogador(
        &self
    ) -> String {
        match self.jogador {
            OpçõesDeJogadas::PAPEL => String::from("papel"),
            OpçõesDeJogadas::PEDRA => String::from("pedra"),
            OpçõesDeJogadas::TESOURA => String::from("tesoura")
        }
    }

    pub fn get_computador(
        &self
    ) -> String {
        match self.computador {
            OpçõesDeJogadas::PAPEL => String::from("papel"),
            OpçõesDeJogadas::PEDRA => String::from("pedra"),
            OpçõesDeJogadas::TESOURA => String::from("tesoura")
        }
    }

    pub fn get_ganhador(
        &self
    ) -> String {
        match self.ganhador {
            PossíveisGanhadores::COMPUTADOR => String::from("computador"),
            PossíveisGanhadores::JOGADOR => String::from("usuário"),
            PossíveisGanhadores::EMPATE => String::from("empate")
        }
    }
}
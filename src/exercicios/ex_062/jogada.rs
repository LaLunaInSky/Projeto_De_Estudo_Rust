use rand::random_range;

use crate::exercicios::ex_062::{
    escolhas::Escolhas,
    jogadores::Jogadores
};

#[derive(Debug)]
pub struct Jogada {
    jogada_computador: u8,
    escolha_computador: Escolhas,
    jogada_usuário: u8,
    escolha_usuário: Escolhas,
    ganhador: Jogadores
}

impl Jogada {
    pub fn new(
        jogada_usuário: u8,
        escolha_usuário: Escolhas
    ) -> Self {
        let jogada_computador: u8 = random_range(0..11);
        let mut escolha_computador = Escolhas::IMPAR;

        if escolha_usuário == Escolhas::IMPAR {
            escolha_computador = Escolhas::PAR;
        }
        
        let soma_das_jogadas = jogada_computador + jogada_usuário;

        let mut _ganhador = Jogadores::COMPUTADOR;

        if soma_das_jogadas % 2 == 0 {
            if escolha_usuário == Escolhas::PAR {
                _ganhador = Jogadores::USUARIO;
            } else {
                _ganhador = Jogadores::COMPUTADOR;
            }
        } else {
            if escolha_usuário == Escolhas::IMPAR {
                _ganhador = Jogadores::USUARIO;
            } else {
                _ganhador = Jogadores::COMPUTADOR;
            }
        }

        Self {
            jogada_computador,
            escolha_computador,
            jogada_usuário,
            escolha_usuário,
            ganhador: _ganhador
        }
    }

    pub fn get_jogada_computador(
        &self
    ) -> u8 {
        return self.jogada_computador;
    }

    pub fn get_escolha_computador(
        &self
    ) -> Escolhas {
        return self.escolha_computador.clone();
    }

    pub fn get_jogada_usuário(
        &self
    ) -> u8 {
        return self.jogada_usuário;
    }

    pub fn get_escolha_usuário(
        &self
    ) -> Escolhas {
        return self.escolha_usuário.clone();
    }

    pub fn get_ganhador(
        &self
    ) -> Jogadores {
        return self.ganhador.clone();
    }
}
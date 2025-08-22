use crate::recursos::{
    exercicio_informacoes::Exercício_Informações,
    limpar_terminal::limpar_terminal
};

use std::io::stdin;

pub fn perguntar_se_quer_iniciar_novamente_o_exercício(
    exercício_informações: &Exercício_Informações
) -> bool {
    loop {
        println!(
            "Quer iniciar novamente o exercício? [S/N]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                let resposta_da_pergunta = resposta_da_pergunta.as_str();

                match resposta_da_pergunta {
                    "s" => {
                        limpar_terminal();
                        
                        return true;
                    }
                    "n" => return false,
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
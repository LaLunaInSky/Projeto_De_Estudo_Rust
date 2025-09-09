use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod jogadores;
mod escolhas;
mod jogada;

use jogadores::Jogadores;
use escolhas::Escolhas;
use jogada::Jogada;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("062")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut quantidade_de_vezes_que_o_usuário_ganhou: u32 = 0;
        
        loop {
            let jogada = Jogada::new(
                obter_um_número_de_zero_à_dez(
                    &exercício_informações
                ),
                obter_a_escolha_de_par_ou_impar(
                    &exercício_informações
                )
            );
            
            analisar_jogada(
                &jogada
            );
        
            if jogada.get_ganhador() == Jogadores::USUARIO {
                quantidade_de_vezes_que_o_usuário_ganhou += 1;
            } else {
                break;
            }
        }

        println!(
            "Você ganhou {} vez{}!\n",
            quantidade_de_vezes_que_o_usuário_ganhou,
            if quantidade_de_vezes_que_o_usuário_ganhou != 1 {"es"} else {""}
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn analisar_jogada(
    jogada: &Jogada
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Você {}!\n",
        if jogada.get_ganhador() == Jogadores::USUARIO {
            "ganhou".to_uppercase()
        } else {
            "perdeu".to_uppercase()
        }
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Computador..Jogada.: {}
Computador.Escolha.: {}
Usuário.....Jogada.: {}
Usuário....Escolha.: {}
",
        jogada.get_jogada_computador(),
        if jogada.get_escolha_computador() == Escolhas::PAR {"Par"} else {"Ímpar"},
        jogada.get_jogada_usuário(),
        if jogada.get_escolha_usuário() == Escolhas::PAR {"Par"} else {"Ímpar"}
    );

    sleep(Duration::from_millis(1100));
}

fn obter_a_escolha_de_par_ou_impar(
    exercício_informações: &ExercícioInformações
) -> Escolhas {
    loop {
        println!(
            "Você quer Par [p] ou Ímpar [i]?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta = input.trim().to_lowercase();

                let resposta = resposta.as_str();

                match resposta {
                    "p" | "i" => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "A escolha de {},\nfoi adicionada com sucesso!\n",
                            if resposta == "p" {"Par"} else {"Ímpar"}
                        );

                        if resposta == "p" {
                            return Escolhas::PAR;
                        } else {
                            return Escolhas::IMPAR;
                        }
                    }
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas é aceito p ou i!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn obter_um_número_de_zero_à_dez(
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "Qual número você escolhe? [0 à 10]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(jogada) => {
                        if jogada <= 10 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "A jogada de {},\nfoi adicionada com sucesso!\n",
                                jogada
                            );

                            return jogada;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Apenas é aceito 0 à 10!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
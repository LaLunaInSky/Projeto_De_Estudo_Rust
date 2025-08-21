use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command,
};

use rand::random_range;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

// ex_068!
fn descrição_do_exercício() -> String {format!(
        "Descrição do exercício 062:
 Um programa que joga par ou ímpar com o\nusuário. O jogo só será interrompido\nquando o usuário PERDER, mostrando o\ntotal de n vitórias consecutivas que o\nmesmo conquistou no final do jogo.
"
    )
}

#[derive(Debug, PartialEq)]
enum Jogadores {
    computador,
    usuário
}

#[derive(Debug, PartialEq)]
enum Escolhas {
    par,
    ímpar
}

#[derive(Debug)]
struct Jogada {
    jogada_computador: u8,
    escolha_computador: Escolhas,
    jogada_usuário: u8,
    escolha_usuário: Escolhas,
    ganhador: Jogadores
}

impl Jogada {
    fn new(
        jogada_usuário: u8,
        escolha_usuário: Escolhas
    ) -> Self {
        let jogada_computador: u8 = random_range(0..11);
        let mut escolha_computador = Escolhas::ímpar;

        if escolha_usuário == Escolhas::ímpar {
            escolha_computador = Escolhas::par;
        }
        
        let soma_das_jogadas = jogada_computador + jogada_usuário;

        let mut ganhador = Jogadores::computador;

        if soma_das_jogadas % 2 == 0 {
            if escolha_usuário == Escolhas::par {
                ganhador = Jogadores::usuário;
            } else {
                ganhador = Jogadores::computador;
            }
        } else {
            if escolha_usuário == Escolhas::ímpar {
                ganhador = Jogadores::usuário;
            } else {
                ganhador = Jogadores::computador;
            }
        }

        Self {
            jogada_computador,
            escolha_computador,
            jogada_usuário,
            escolha_usuário,
            ganhador
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!(
            "{}\n{}",
            cabeçalho_do_programa,
            descrição_do_exercício()
        );

        /* Corpo do Exercício */
        let mut quantidade_de_vezes_que_o_usuário_ganhou = 0;
        
        loop {
            let jogada = Jogada::new(
                obter_um_número_de_zero_à_dez(
                    &cabeçalho_do_programa
                ),
                obter_a_escolha_de_par_ou_impar(
                    &cabeçalho_do_programa
                )
            );
            
            analisar_jogada(
                &jogada
            );
        
            if jogada.ganhador == Jogadores::usuário {
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

        let resposta_sobre_continuar = perguntar_se_quer_jogar_novamente(
            &cabeçalho_do_programa
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_jogada(
    jogada: &Jogada
) {
    sleep(Duration::from_millis(1100));

    println!(
        "Você {}!\n",
        if jogada.ganhador == Jogadores::usuário {
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
        jogada.jogada_computador,
        if jogada.escolha_computador == Escolhas::par {"Par"} else {"Ímpar"},
        jogada.jogada_usuário,
        if jogada.escolha_usuário == Escolhas::par {"Par"} else {"Ímpar"}
    );

    sleep(Duration::from_millis(1700));
}

fn perguntar_se_quer_jogar_novamente(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer jogar novamente? [S/N]"
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
                        clean_terminal_linux();

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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

fn obter_a_escolha_de_par_ou_impar(
    cabeçalho_do_programa: &String
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
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "A escolha de {},\nfoi adicionada com sucesso!\n",
                            if resposta == "p" {"Par"} else {"Ímpar"}
                        );

                        if resposta == "p" {
                            return Escolhas::par;
                        } else {
                            return Escolhas::ímpar;
                        }
                    }
                    _ => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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
    cabeçalho_do_programa: &String
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
                        if jogada >= 0 && jogada <= 10 {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "A jogada de {},\nfoi adicionada com sucesso!\n",
                                jogada
                            );

                            return jogada;
                        } else {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "Erro! Apenas é aceito 0 à 10!\n"
                            );
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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
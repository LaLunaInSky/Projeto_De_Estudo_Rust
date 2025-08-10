use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

use rand::random_range;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 054:
 Melhore o jogo do Ex_025 onde o\ncomputador vai \"pensar\" em um número\nentre 0 e 10. Só que agora o jogador vai\ntentar adivinhar até acertar, mostrando\nno final quantos palpites foram\nnecessários para vencer.
"
    )
}

#[derive(Debug)]
struct Jogadas {
    número_do_computador: u8,
    tentativas: u8,
}

impl Jogadas {
    fn new() -> Self {
        let número_do_computador: u8 = random_range(0..11);
        
        Self {
            número_do_computador,
            tentativas: 0
        }
    }

    fn adicionar_mais_uma_tentativa(&mut self) {
        self.tentativas += 1;
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do exercício */
        loop {
        println!(
            "{}\n{}",
            cabeçalho_do_programa,
            descrição_do_exercício()
        );

        /* Corpo do Exercício */
        let mut jogadas = Jogadas::new();

        loop {
            let palpite_do_jogador: u8 = obter_o_palpite(
                &cabeçalho_do_programa
            );

            if palpite_do_jogador != jogadas.número_do_computador {
                if palpite_do_jogador < jogadas.número_do_computador {
                    println!(
                        "O número {} é MENOR!\n",
                        palpite_do_jogador
                    );
                } else {
                    println!(
                        "O número {} é MAIOR!\n",
                        palpite_do_jogador
                    );
                }

                jogadas.adicionar_mais_uma_tentativa();
            } else {
                break;
            }
        }

        analisar_as_tentativas(
            jogadas
        );

        let resposta_sobre_continuar = perguntar_se_quer_jogar_de_novo(
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

fn analisar_as_tentativas(
    jogadas: Jogadas
) {
    println!(
        "Você ACERTOU! É o número {}.\n",
        jogadas.número_do_computador
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Foram {} tentivas até acertar!\n",
        jogadas.tentativas
    );

    sleep(Duration::from_millis(1500));
}

fn obter_o_palpite(
    cabeçalho_do_programa: &String
) -> u8 {
    loop {
        println!(
            "Qual o seu palpite? [0 até 10]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(palpite) => {
                        match palpite {
                            0..11 => {
                                clean_terminal_linux();

                                println!(
                                    "{}\n{}",
                                    cabeçalho_do_programa,
                                    descrição_do_exercício()
                                );

                                return palpite;
                            }
                            _ => {
                                clean_terminal_linux();

                                println!(
                                    "{}\n{}",
                                    cabeçalho_do_programa,
                                    descrição_do_exercício()
                                );

                                println!(
                                    "Erro! Apenas é aceito de 0 à 10!\n"
                                );
                            }
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

fn perguntar_se_quer_jogar_de_novo(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer jogar de novo? [S/N]"
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
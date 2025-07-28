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

fn descrição_do_exercício() {
    println!("Descrição do exercício 042:");
    println!(
        " Um programa que faça o usuário jogar\nJOKENPÔ com o computador."
    );
}

#[derive(Debug)]
struct Jogadas {
    jogador: String,
    computador: String,
    ganhador: String,
}

impl Jogadas {
    fn new(
        jogador: String, 
        computador: String,
    ) -> Self {
        let ganhador = match jogador.as_str() {
            "pedra" => {
                match computador.as_str() {
                    "pedra" => {
                        String::from("empate")
                    }
                    "papel" => {
                        String::from("computador")
                    }
                    "tesoura" => {
                        String::from("usuário")
                    }
                    _ => String::new(),
                }
            }
            "papel" => {
                match computador.as_str() {
                    "pedra" => {
                        String::from("usuário")
                    }
                    "papel" => {
                        String::from("empate")
                    }
                    "tesoura" => {
                        String::from("computador")
                    }
                    _ => String::new(),
                }
            },
            "tesoura" => {
                match computador.as_str() {
                    "pedra" => {
                        String::from("computador")
                    }
                    "papel" => {
                        String::from("usuário")
                    }
                    "tesoura" => {
                        String::from("empate")
                    }
                    _ => String::new(),
                }
            }
            _ => String::new(),
        };

        Self {
            jogador,
            computador,
            ganhador
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let opções_de_jogadas = vec![
            "pedra", "papel", "tesoura"
        ];

        let jogadas = Jogadas::new(
            obter_a_opção_da_escolha(
                &cabeçalho_do_programa,
                &opções_de_jogadas
            ),
            obter_a_escolha_do_computador(
                &opções_de_jogadas
            )
        );

        analisar_quem_ganhou(&jogadas);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novo_valore(
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

fn obter_a_escolha_do_computador(
    opções_de_jogadas: &Vec<&str>
) -> String {
    let número_sortado = random_range(0..=2);

    let jogada_escolhida = format!(
        "{}", opções_de_jogadas[número_sortado]
    );
    
    return jogada_escolhida;
}

fn perguntar_se_quer_adicionar_novo_valore(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!("Quer adicionar uma nova jogada?\n[S/N]");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
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
                            "{}",
                            cabeçalho_do_programa
                        );

                        descrição_do_exercício();

                        println!(
                            "\nErro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn analisar_quem_ganhou(jogadas: &Jogadas) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando quem ganhou...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "O usuário jogu {}\nE o computador jogou {}\nlogo...",
        jogadas.jogador.to_uppercase(),
        jogadas.computador.to_uppercase()
    );

    println!(
        "\n{}\n",
        match jogadas.ganhador.as_str() {
            "empate" => {
                format!(
                    "Ouve {}!",
                    jogadas.ganhador.to_uppercase()
                )
            }
            _ => {
                format!(
                    "O ganhador foi o {}!",
                    jogadas.ganhador.to_uppercase()
                )
            }
        }
    );

    sleep(Duration::from_millis(1200));
} 

fn obter_a_opção_da_escolha(
    cabeçalho_do_programa: &String,
    opções_de_jogadas: &Vec<&str>
) -> String {
    mostrador_de_escolha();
    
    loop {
        println!("Qual você escolhe?");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(opção_escolhida) => {
                        match opção_escolhida {
                            1..=3 => {
                                let jogada_escolhida = format!(
                                    "{}",
                                    opções_de_jogadas[opção_escolhida - 1]
                                );

                                clean_terminal_linux();

                                println!(
                                    "{}",
                                    cabeçalho_do_programa
                                );

                                descrição_do_exercício();

                                println!(
                                    "\nA opção de {},\nfoi selecionada com sucesso!\n",
                                    jogada_escolhida
                                );

                                return jogada_escolhida;
                            }
                            _ => {
                                clean_terminal_linux();

                                println!(
                                    "{}",
                                    cabeçalho_do_programa
                                );

                                descrição_do_exercício();

                                println!();

                                mostrador_de_escolha();

                                println!(
                                    "Erro! Apenas é aceito de 1 à 3!\n"
                                );
                            }
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}",
                            cabeçalho_do_programa
                        );

                        descrição_do_exercício();

                        println!();

                        mostrador_de_escolha();

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

fn mostrador_de_escolha() {
    println!(
        " [ 1 ] Pedra
 [ 2 ] Papel
 [ 3 ] Tesoura
"
    )
}
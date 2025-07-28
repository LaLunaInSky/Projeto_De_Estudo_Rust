use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

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
        computador: String
    ) -> Self {
        let ganhador = String::new();

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
            String::from("pedra")
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
        "{:?}\n",
        jogadas
    );

    sleep(Duration::from_millis(2500));
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
use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 049:
 Um programa que lê um número inteiro e\nretorna se ele é ou não um número primo.
"
    )
}

#[derive(Debug)]
struct Número_Primo {
    número: u32,
    é_primo: bool
}

impl Número_Primo {
    fn new(
        número: u32
    ) -> Self {
        let é_primo = identificar_se_é_primo(
            &número
        );

        Self {
            número,
            é_primo
        }
    }

    
}

fn identificar_se_é_primo(
    número: &u32
) -> bool {
    if *número > 1 {    
        if número % 2 == 0 {
            if *número == 2 {
                return true;
            } else {
                return false;
            }
        } else if número % 3 == 0 {
            if *número == 3 {
                return true;
            } else {
                return false;
            }
        } else if número % 5 == 0 {
            if *número == 5 {
                return true;
            } else {
                return false
            }
        } else if número % 7 == 0 {
            if *número == 7 {
                return true;
            } else {
                return false;
            }
        }

        return true;
    } else {
        return false;
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
        let número_digitado = Número_Primo::new(
            obter_um_número_inteiro(
                &cabeçalho_do_programa
            )
        );

        analisar_o_número(
            &número_digitado
        );
        
        let resposta_sobre_continuar = perguntar_se_quer_adicionar_um_novo_número(
            &cabeçalho_do_programa
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_o_número(
    número: &Número_Primo
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o número...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "O número {}{} é PRIMO!\n",
        número.número,
        if !número.é_primo {" NÃO"} else {""}
    );

    sleep(Duration::from_millis(1200));
}

fn obter_um_número_inteiro(
    cabeçalho_do_programa: &String
) -> u32 {
    loop {
        println!(
            "Digite um número inteiro:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "O número inteiro {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número
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

fn perguntar_se_quer_adicionar_um_novo_número(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer digitar um novo número? [S/N]"
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
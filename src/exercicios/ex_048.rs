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
        "Descrição do exercício 048:
 Um programa que lê o primeiro termo e a\nrazão de uma PA. No final, mostra os 10\nprimeiros termos dessa progressão.
"
    )
}

#[derive(Debug)]
struct PA {
    primeiro_termo: u32,
    razão: u32,
    dez_primeiros_termos: Vec<u32>,
}

impl PA {
    fn new(
        primeiro_termo: u32,
        razão: u32
    ) -> Self {
        let mut dez_primeiros_termos: Vec<u32> = vec![];

        for count in 1..11 {
            if count == 1 {
                dez_primeiros_termos.push(primeiro_termo);
            } else {
                let próximo_termo = (
                    dez_primeiros_termos[
                        count - 2
                    ] + razão);

                dez_primeiros_termos.push(próximo_termo);
            }
        }

        Self {
            primeiro_termo,
            razão,
            dez_primeiros_termos
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
        let pa = PA::new(
            obter_um_número(
                &cabeçalho_do_programa,
                "primeiro_termo"
            ),
            obter_um_número(
                &cabeçalho_do_programa,
                "razão"
            )
        );

        analisar_a_pa(&pa);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_valores(
            &cabeçalho_do_programa
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de execícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn perguntar_se_quer_adicionar_novos_valores(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar novos valores? [S/N]"
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

fn analisar_a_pa(
    pa: &PA
) {
    sleep(Duration::from_millis(1000));
    
    println!(
        "Analisando a PA...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "Primeiro Termo: {}\nA Razão: {}\n",
        pa.primeiro_termo,
        pa.razão
    );

    sleep(Duration::from_millis(1000));

    println!(
        "Logo os 10 termos são...\n"
    );

    sleep(Duration::from_millis(1500));

    for (index, número) in pa.dez_primeiros_termos.iter().enumerate() {
        if index == 9 {
            print!(
                "{}.\n\n", 
                número
            );
        }else if index == 4 && index > 0 {
            print!(
                "{}\n",
                número
            )
        } else {
            print!(
                "{}, ",
                número
            )
        }
    }

    sleep(Duration::from_millis(1500));
}

fn obter_um_número(
    cabeçalho_do_programa: &String,
    argumento_de_chamada: &str
) -> u32 {
    loop {
        println!(
            "Digite {}",
            if argumento_de_chamada == "razão" {
                "a Razão:"
            } else {
                "o primeiro termo:"
            }
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

                        if argumento_de_chamada == "razão" {
                            match número {
                                0 => {
                                    println!(
                                        "A razão de 1,\nfoi adicionado com sucesso!\n"
                                    );

                                    return 1;
                                }
                                _ => {
                                    println!(
                                        "A razão de {},\nfoi adicionado com sucesso!\n",
                                        número
                                    );

                                    return número;
                                }
                            }
                        } else {
                            println!(
                                "O primeiro termo {},\nfoi adicionado com sucesso!\n",
                                número
                            );

                            return número;
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
                            "Erro! Digite apenas número!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
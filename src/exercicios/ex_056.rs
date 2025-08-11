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
        "Descrição do exercício 056:
 Um programa que lê um número inteiro e\nmostre o seu fatorial.

 Exemplo:
5! = 5 x 4 x 3 x 2 x 1 = 120
"
    )
}

#[derive(Debug)]
struct Fatorial {
    número: u32,
    fatorial: String
}

impl Fatorial {
    fn new(
        número: u32
    ) -> Self {

        fn obter_o_fatorial(
            número: &u32
        ) -> String {
            let mut fatorial = String::new();

            fatorial = format!(
                "{}! = ",
                *número
            );

            let mut multiplicação: u32 = 1;
            let mut número_do_fatorial = *número;

            while número_do_fatorial > 0 {
                let mut número_formatado = String::new();

                multiplicação *= número_do_fatorial;

                if número_do_fatorial > 1 {
                    número_formatado = format!(
                        "{} x ",
                        número_do_fatorial
                    );
                } else {
                    número_formatado = format!(
                        "{} = {}",
                        número_do_fatorial,
                        multiplicação
                    )
                }

                fatorial.push_str(&número_formatado);

                número_do_fatorial -= 1;
            }

            return fatorial;
        }

        let fatorial = obter_o_fatorial(
            &número
        );

        Self {
            número,
            fatorial
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
        let fatorial = Fatorial::new(
            obter_o_número_inteiro(
                &cabeçalho_do_programa
            )
        );

        analisar_o_número(
            &fatorial
        );

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_um_novo_fatorial(
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

fn analisar_o_número(
    fatorial: &Fatorial
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o fatorial...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "{}\n",
        fatorial.fatorial
    );

    sleep(Duration::from_millis(1500));
}

fn obter_o_número_inteiro(
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
                        if número >= 1 {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "O número {},\nfoi adicionado com sucesso!\n",
                                número
                            );

                            return número;
                        } else {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "Erro! Digite um número maior que zero!\n"
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

fn perguntar_se_quer_adicionar_um_novo_fatorial(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar um novo fatorial? [S/N]"
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
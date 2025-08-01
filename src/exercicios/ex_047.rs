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
        "Descrição do exercício 047:
 Um programa que lê seis números inteiros\ne mostre a soma apenas daqueles que forem\npares. Se o valor digitado for ímpar,\ndesconsidere-o.
"
    )
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
        let mut números_inteiros_digitados: Vec<u32> = vec![];

        for quantidade in 1..7 {
            números_inteiros_digitados.push(
                obter_um_número_inteiro(
                    &cabeçalho_do_programa, 
                    quantidade
                )
            );
        }

        analisar_os_números(
            &números_inteiros_digitados
        );

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_números(
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

fn perguntar_se_quer_adicionar_novos_números(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar novos números? [S/N]"
        );

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

fn analisar_os_números(
    números: &Vec<u32>
) {
    let mut números_pares: Vec<u32> = vec![];
    let mut números_ímpares: Vec<u32> = vec![];
    let mut soma_dos_números_pares: u32 = 0;

    for número in números {
        if número % 2 == 0 {
            números_pares.push(*número);

            soma_dos_números_pares += número;
        } else {
            números_ímpares.push(*número);
        }
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "Os números pares: {:?} \nOs números ímpares: {:?}\n",
        números_pares, números_ímpares
    );

    sleep(Duration::from_millis(1000));

    println!(
        "A soma dos números pares é {}.\n",
        soma_dos_números_pares
    );

    sleep(Duration::from_millis(2000));
}

fn obter_um_número_inteiro(
    cabeçalho_do_programa: &String,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o {}º Número:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
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
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "Erro! Digite apenas números.\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
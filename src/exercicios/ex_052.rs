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
        "Descrição do exercício 052:
 Um programa que lê o peso de cinco\npessoas. No final, mostra qual foi o maior\ne o menor peso lido.
"
    )
}

#[derive(Debug)]
struct Pesos {
    pesos: Vec<f32>,
    maior_peso: f32,
    menor_peso: f32,
}

impl Pesos {
    fn new(
        pesos: Vec<f32>
    ) -> Self {
        let mut maior_peso: f32 = 0.0;
        let mut menor_peso: f32 = 0.0;

        for (indice, peso) in pesos.iter().enumerate() {
            if indice == 0 {
                maior_peso = *peso;
                menor_peso = *peso;
            } else {
                if menor_peso > *peso {
                    menor_peso = *peso;
                }

                if maior_peso < *peso {
                    maior_peso = *peso
                }
            }
        }

        Self {
            pesos,
            maior_peso,
            menor_peso
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
        let mut pesos: Vec<f32> = vec![];

        for contagem in 1..6 {
            pesos.push(
                obter_o_peso(
                    &cabeçalho_do_programa,
                    contagem
                )
            );
        }

        let pesos = Pesos::new(pesos);

        analisar_os_pesos(
            &pesos
        );

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
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_os_pesos(
    pesos: &Pesos 
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os pesos...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "Os pesos:\n{:?}\n\nO menor peso é: {:.2}Kg\nO maior peso é: {:.2}Kg\n",
        pesos.pesos,
        pesos.menor_peso,
        pesos.maior_peso
    );

    sleep(Duration::from_millis(1500));
}

fn obter_o_peso(
    cabeçalho_do_programa: &String,
    indice_da_chamada: u8
) -> f32 {
    loop {
        println!(
            "Digite o {indice_da_chamada}º peso:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(peso) => {
                        let formatação_do_peso = format!(
                            "{:.2}",
                            peso
                        );

                        match formatação_do_peso.parse::<f32>() {
                            Ok(peso_final) => {
                                clean_terminal_linux();

                                println!(
                                    "{}\n{}",
                                    cabeçalho_do_programa,
                                    descrição_do_exercício()
                                );

                                println!(
                                    "O peso de {:.2},\nfoi adicionado com sucesso!\n",
                                    peso_final
                                );

                                return peso_final;
                            }
                            Err(_) => (),
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
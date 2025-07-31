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
        "Descrição do exercício 046:
 Refaça o Ex_007, mostrando a tabuada de\num número que o usuário escolher, só que\nagora utilizando uma respetição com\npergunta sobre continuar."
    )
}

struct Tabuada {
    número: u32
}

impl Tabuada {
    fn mostrar_tabuada(
        número: u32
    ) -> Self {
        for count in 1..11 {
            println!(
                "{} X {:>2} = {}",
                número, 
                count,
                (número * count)
            );
        }

        println!();

        Self {
            número
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercíco */
    loop {
        println!(
            "{}\n{}\n", 
            cabeçalho_do_programa,
            descrição_do_exercício()
        );

        /* Corpo do Exercício */
        let número_digitado = obter_o_número_inteiro(
            &cabeçalho_do_programa
        );

        Tabuada::mostrar_tabuada(
            número_digitado
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
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn  perguntar_se_quer_adicionar_um_novo_número(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar um novo número? [S/N]"
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
                        clean_terminal_linux();

                        println!(
                            "{}\n{}\n",
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

fn obter_o_número_inteiro(
    cabeçalho_do_programa: &String,
) -> u32 {
    loop {
        println!(
            "Digite um número:"
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) =>{
                        clean_terminal_linux();

                        println!(
                            "{}\n{}\n",
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
                            "{}\n{}\n",
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
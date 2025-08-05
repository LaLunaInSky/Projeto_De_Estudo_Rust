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
        "Descrição do exercício 050:
 Um programa que lê uma frase qualquer e\nretorna se ela é um palíndromo,\ndesconsiderando os espaços.

Exemplos:
- apos a sopa
- a sacada da casa
- a torre da derrota
- o lobo ama o bolo
- anotaram a data da maratona
"
    )
}

#[derive(Debug)]
struct Frase {
    frase: String,
    frase_sem_espaços: String,
    é_palíndromo: bool,
}

impl Frase {
    fn new(
        frase: String
    ) -> Self {
        let mut frase_sem_espaços = String::new();

        for char in frase.chars() {
            if char != ' ' {
                let char = format!(
                    "{}",
                    char
                );

                let char = char.as_str();

                frase_sem_espaços.push_str(char);
            }
        }

        let é_palíndromo = if frase_sem_espaços == frase_sem_espaços.chars().rev().collect::<String>() {
            true
        } else {
            false
        };

        Self {
            frase,
            frase_sem_espaços,
            é_palíndromo
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
        let frase_digita = Frase::new(
            obter_uma_frase(
                &cabeçalho_do_programa
            )
        );

        analisar_a_frase(
            &frase_digita
        );

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_uma_nova_frase(
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

fn analisar_a_frase(
    frase: &Frase
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a frase...\n"
    );

    sleep(Duration::from_millis(2500));

    println!(
        "A frase sem espaços fica:\n{}\n\nLogo...\n",
        frase.frase_sem_espaços
    );

    sleep(Duration::from_millis(1500));

    println!(
        "A frase {}é PALÍNDROMO!\n",
        if !frase.é_palíndromo {
            "NÃO "
        } else {
            ""
        }
    );

    sleep(Duration::from_millis(1200));
}

fn obter_uma_frase(
    cabeçalho_do_programa: &String
) -> String {
    loop {
        println!(
            "Digite uma frase:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let frase = input.trim().to_lowercase();

                clean_terminal_linux();

                println!(
                    "{}\n{}",
                    cabeçalho_do_programa,
                    descrição_do_exercício()
                );

                println!(
                    "A frase {},\nfoi adicionada com sucesso!\n",
                    frase
                );

                return frase;
            }
            Err(_) => (),
        }
    }
}

fn perguntar_se_quer_adicionar_uma_nova_frase(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer digitar outra frase? [S/N]"
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
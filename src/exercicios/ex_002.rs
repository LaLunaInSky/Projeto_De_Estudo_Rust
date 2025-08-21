use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::limpar_terminal::limpar_terminal;

fn descrição_do_exercícios() -> String {
    format!(
        "Descrição do exercício 002:
 Um programa que lê a entrada do teclado\ne mostra no terminal o seu tipo primitivo,\ne outras as informação possíveis sobre o\nque foi digitado.

Exemplo:

* Seu Tipo Primitivo
* Se possui espaços
* Se é apenas um número
* Se é alfabético
* Se é alfanumérico
* Se está em maiúscula
* Se está em minúscula
* Se está capitalizada
"
    )
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    loop {
        println!(
            "{}\n{}",
            cabeçalho_do_programa,
            descrição_do_exercícios()
        );

        /* Corpo do Exercício */
        let frase_digitada = obter_uma_frase(
            &cabeçalho_do_programa
        );

        analisar_a_frase(
            &frase_digitada
        );
    
        let resposta_da_pergunta = perguntar_se_quer_adicionar_outra_frase(
            &cabeçalho_do_programa
        );

        if !resposta_da_pergunta {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...");

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn perguntar_se_quer_adicionar_outra_frase(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Gostaria de digitar outra frase? [S/N]"
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
                        limpar_terminal();

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        limpar_terminal();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercícios()
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

fn obter_uma_frase(
    cabeçalho_do_programa: &String
) -> String {
    loop {
        println!(
            "Digite algo, pode conter números:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let frase = input.trim().to_string();

                limpar_terminal();

                println!(
                    "{}\n{}", 
                    cabeçalho_do_programa,
                    descrição_do_exercícios()
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

fn analisar_a_frase(
    entrada_digitada: &String
) {
    let entrada_digitada = entrada_digitada.as_str();
    
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o que foi digitado:\n\'{}\'",
        entrada_digitada
    );
    
    let mut entrada_sem_espaços = String::new();
    let mut chars: Vec<char> = vec![];

    if entrada_digitada.contains(" ") {
        for char in entrada_digitada.chars() {
            if char != ' ' {
                chars.push(char);
            }
        }

        for char in chars {
            entrada_sem_espaços.push(char);
        }

    } else {
        entrada_sem_espaços.push_str(
            entrada_digitada
        );
    }

    sleep(Duration::from_millis(1000));

    match entrada_sem_espaços.parse::<i64>() {
        Ok(number) => {
            println!(
                "\nO seu Tipo Primitivo é: {}", 
                retornar_o_tipo_primitivo(
                    &number
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "É apenas número?......: SIM"
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Possui espaços?.......: {}", 
                retornar_se_possui_espaços(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "É alfabético?.........: NÃO"
            );
            sleep(Duration::from_millis(1000));

            println!(
                "É alfanumérico?.......: NÃO"
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Está em maiúscula?....: NÃO"
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Está em minúscula?....: NÃO"
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Está capitalizada?....: NÃO"
            );
            sleep(Duration::from_millis(1000));
        }
        Err(_) => {
            println!(
                "\nO seu Tipo Primitivo é: {}", 
                retornar_o_tipo_primitivo(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "É apenas número?......: NÃO"
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Possui espaços?.......: {}", 
                retornar_se_possui_espaços(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "É alfabético?.........: {}", 
                if retorna_se_a_string_possui_números(
                    &entrada_digitada
                ) == "SIM" {"NÃO"} else {"SIM"}
            );
            sleep(Duration::from_millis(1000));

            println!(
                "É alfanumérico?.......: {}", 
                retorna_se_a_string_possui_números(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Está em maiúscula?....: {}", 
                verificar_se_a_string_é_maiúscula(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Está em minúscula?....: {}", 
                verificar_se_a_string_é_minúscula(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));

            println!(
                "Está capitalizada?....: {}", 
                verificar_se_a_string_está_capitalizada(
                    &entrada_digitada
                )
            );
            sleep(Duration::from_millis(1000));
        }
    }    

    println!();

    sleep(Duration::from_millis(1500));
}

fn retornar_o_tipo_primitivo<T>(
    _: &T
) -> String {
    std::any::type_name::<T>().to_string()
}

fn retornar_se_possui_espaços(
    entrada_digitada: &str
) -> String {
    if entrada_digitada.contains(" ") {
        return String::from("SIM");
    } else {
        return String::from("NÃO");
    }
}

fn retorna_se_a_string_possui_números(
    entrada_digitada: &str
) -> String {
    let lista_de_número_primários = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    for número in lista_de_número_primários {
        if entrada_digitada.contains(número) {
            return String::from("SIM");
        }
    }

    String::from("NÃO")
}

fn verificar_se_a_string_é_maiúscula(
    entrada_digitada: &str
) -> String {
    if entrada_digitada == entrada_digitada.to_uppercase() {
        return String::from("SIM");
    } else {
        return String::from("NÃO");
    }
}

fn verificar_se_a_string_é_minúscula(
    entrada_digitada: &str
) -> String {
    if entrada_digitada == entrada_digitada.to_lowercase() {
        return String::from("SIM");
    } else {
        return String::from("NÃO");
    }
}

fn verificar_se_a_string_está_capitalizada(
    entrada_digitada: &str
) -> String {
    if verificar_se_a_string_é_maiúscula(&entrada_digitada) == "NÃO" && verificar_se_a_string_é_minúscula(&entrada_digitada) == "NÃO" && entrada_digitada.len() > 1 {
        let palavras_digitadas: Vec<&str> = entrada_digitada.split(" ").collect();

        let mut primeiras_letras_upper: Vec<bool> = vec![];

        for palavra in &palavras_digitadas {
            for (index, char) in palavra.chars().enumerate() {
                if index == 0 {
                    if char.is_ascii_uppercase() {
                        primeiras_letras_upper.push(true);
                    }
                }
            }
        }
        
        if palavras_digitadas.len() == primeiras_letras_upper.len() {
            return String::from("SIM");
        } else {
            return String::from("NÃO");
        }

    } else if entrada_digitada.len() == 1 && verificar_se_a_string_é_minúscula(&entrada_digitada) == "NÃO" {
        return String::from("SIM");

    } else {
        return String::from("NÃO");
    }
}
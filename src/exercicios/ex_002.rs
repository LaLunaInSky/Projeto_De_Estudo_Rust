use std::{
    process::Command,
    io,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercícios() {
    println!("Descrição do exercício 002:");
    println!(
        " Um programa que lê a entrada do teclado\ne mostra no terminal o seu tipo primitivo,\ne outras as informação possíveis sobre o\nque foi digitado.

Exemplo:

* Seu Tipo Primitivo
* Se possui espaços
* Se é apenas um número
* Se é alfabético
* Se é alfanumérico
* Se está em maiúscula
* Se está em minúscula
* Se está capitalizada"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        println!("{}", cabeçalho_do_programa);
    
        descrição_do_exercícios();
    
        println!();
    
        let resposta_e_input = obter_a_entrada_do_teclado(&cabeçalho_do_programa);

        if resposta_e_input == "n" {
            break;
        }
    }

    println!("{}", cabeçalho_do_programa);

    println!("\nVoltando para o menu de exercício...");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn obter_a_entrada_do_teclado(cabeçalho_do_programa: &String) -> String {
    println!(
        "Digite algo, pode conter números ou não:"
    );

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            loop {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                analise_da_entrada_digitada(&input.trim());

                println!("\nGostaria de digitar outra frase (S/N)? ");

                let mut input_02 = String::new();

                match io::stdin().read_line(&mut input_02) {
                    Ok(_) => {
                        if input_02.trim().to_lowercase() == "s" || input_02.trim().to_lowercase() == "n" {
                            clean_terminal_linux();
                            
                            return input_02.trim().to_lowercase();
                        }
                    }
                    Err(error) => println!("Error: {}", error),
                }
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    input
}

fn analise_da_entrada_digitada(entrada_digitada: &str) {
    println!("\nAnalisando o que foi digitado:\n'{}'", entrada_digitada);
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
        entrada_sem_espaços.push_str(entrada_digitada);
    }

    thread::sleep(Duration::from_millis(1000));

    match entrada_sem_espaços.parse::<i64>() {
        Ok(number) => {
            println!("\nO seu Tipo Primitivo é: {}", retornar_o_tipo_primitivo(&number));
            thread::sleep(Duration::from_millis(1000));

            println!("É apenas número?......: SIM");
            thread::sleep(Duration::from_millis(1000));

            println!("Possui espaços?.......: {}", retornar_se_possui_espaços(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));

            println!("É alfabético?.........: NÃO");
            thread::sleep(Duration::from_millis(1000));

            println!("É alfanumérico?.......: NÃO");
            thread::sleep(Duration::from_millis(1000));

            println!("Está em maiúscula?....: NÃO");
            thread::sleep(Duration::from_millis(1000));

            println!("Está em minúscula?....: NÃO");
            thread::sleep(Duration::from_millis(1000));

            println!("Está capitalizada?....: NÃO");
            thread::sleep(Duration::from_millis(1000));
        }
        Err(_) => {
            println!("\nO seu Tipo Primitivo é: {}", retornar_o_tipo_primitivo(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));

            println!("É apenas número?......: NÃO");
            thread::sleep(Duration::from_millis(1000));

            println!("Possui espaços?.......: {}", retornar_se_possui_espaços(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));

            println!("É alfabético?.........: {}", if retorna_se_a_string_possui_números(&entrada_digitada) == "SIM" {"NÃO"} else {"SIM"});
            thread::sleep(Duration::from_millis(1000));

            println!("É alfanumérico?.......: {}", retorna_se_a_string_possui_números(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));

            println!("Está em maiúscula?....: {}", verificar_se_a_string_é_maiúscula(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));

            println!("Está em minúscula?....: {}", verificar_se_a_string_é_minúscula(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));

            println!("Está capitalizada?....: {}", verificar_se_a_string_está_capitalizada(&entrada_digitada));
            thread::sleep(Duration::from_millis(1000));
        }
    }    
}

fn retornar_o_tipo_primitivo<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn retornar_se_possui_espaços(entrada_digitada: &str) -> String {
    if entrada_digitada.contains(" ") {
        return String::from("SIM");
    } else {
        return String::from("NÃO");
    }
}

fn retorna_se_a_string_possui_números(entrada_digitada: &str) -> String {
    let lista_de_número_primários = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for número in lista_de_número_primários {
        if entrada_digitada.contains(número) {
            return String::from("SIM");
        }
    }

    String::from("NÃO")
}

fn verificar_se_a_string_é_maiúscula(entrada_digitada: &str) -> String {
    if entrada_digitada == entrada_digitada.to_uppercase() {
        return String::from("SIM");
    } else {
        return String::from("NÃO");
    }
}

fn verificar_se_a_string_é_minúscula(entrada_digitada: &str) -> String {
    if entrada_digitada == entrada_digitada.to_lowercase() {
        return String::from("SIM");
    } else {
        return String::from("NÃO");
    }
}

fn verificar_se_a_string_está_capitalizada(entrada_digitada: &str) -> String {
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
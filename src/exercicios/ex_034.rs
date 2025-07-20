use std:: {
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 034:");
    println!(
        " Um programa que lê um número inteiro e\npergunta ao usuário qual base quer\nconverter:
- 1 para binário
- 2 para octal
- 3 para hexadecimal"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let número_inteiro_digitado = obter_um_número_inteiro(&cabeçalho_do_programa);

        mostrar_o_menu_de_opções();

        let resposta_sobre_continuar = obter_a_opção_digitada(&cabeçalho_do_programa, &número_inteiro_digitado);

        if resposta_sobre_continuar == false {
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

fn obter_a_opção_digitada(cabeçalho_do_programa: &String, número_digitado: &u32) -> bool {
    loop {
        let mut converção_na_base = String::new();

        println!("\nQual opção você escolhe?");
    
        let mut input = String::new();
        
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção_do_menu) => {
                        if opção_do_menu < 1 || opção_do_menu > 6 {
                            clean_terminal_linux();
    
                            println!("{}", cabeçalho_do_programa);
                            
                            descrição_do_exercício();
                            
                            println!("\nVocê digitou o número {}\n", número_digitado);

                            mostrar_o_menu_de_opções();

                            println!("\nErro! Apenas é aceito o número 1 à 5!");
                        } else {
                            if opção_do_menu == 5 {
                                return false;

                            } else if opção_do_menu == 4 {
                                clean_terminal_linux();

                                return  true;

                            } else if opção_do_menu == 3 {
                                converção_na_base = format!(
                                    "{:x}",
                                    número_digitado
                                );

                                clean_terminal_linux();
    
                                println!("{}", cabeçalho_do_programa);
                                
                                descrição_do_exercício();
                                
                                println!(
                                    "\nO número {} em HEXA é {}\n", 
                                    número_digitado,
                                    converção_na_base
                                );

                                mostrar_o_menu_de_opções();

                            } else if opção_do_menu == 2 {
                                converção_na_base = format!(
                                    "{:o}",
                                    número_digitado
                                );

                                clean_terminal_linux();
    
                                println!("{}", cabeçalho_do_programa);
                                
                                descrição_do_exercício();
                                
                                println!(
                                    "\nO número {} em OCTAL é {}\n", 
                                    número_digitado,
                                    converção_na_base
                                );

                                mostrar_o_menu_de_opções();
                            } else {
                                converção_na_base = format!(
                                    "{:b}",
                                    número_digitado
                                );

                                clean_terminal_linux();
    
                                println!("{}", cabeçalho_do_programa);
                                
                                descrição_do_exercício();
                                
                                println!(
                                    "\nO número {} em BINÁRIO é {}\n", 
                                    número_digitado,
                                    converção_na_base
                                );

                                mostrar_o_menu_de_opções();
                            }
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();
    
                        println!("{}", cabeçalho_do_programa);
                        
                        descrição_do_exercício();
                        
                        println!("\nVocê digitou o número {}\n", número_digitado);

                        mostrar_o_menu_de_opções();
    
                        println!("\nErro! Digite apenas números!");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn mostrar_o_menu_de_opções() {
    println!(
        "[ 1 ] Base Binária
[ 2 ] Base Octal
[ 3 ] Base Hexadecimal
[ 4 ] Adicionar novo número
[ 5 ] Sair do Exercício"
    );
}

fn obter_um_número_inteiro(cabeçalho_do_programa: &String) -> u32 {
    loop {
        println!("Digite um número inteiro:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nO número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas números inteiros!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}
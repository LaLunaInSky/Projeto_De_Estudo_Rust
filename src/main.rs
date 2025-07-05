use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

mod exercicios;

fn link_dos_exercícios(número_do_exercício: u32, cabeçalho_do_programa: &String) {
    if número_do_exercício == 9 {
        exercicios::ex_009::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 8 {
        exercicios::ex_008::rodar_o_exercício(&cabeçalho_do_programa);
    } else if  número_do_exercício == 7 {
        exercicios::ex_007::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 6 {
        exercicios::ex_006::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 5 {
        exercicios::ex_005::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 4 {
        exercicios::ex_004::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 3 {
        exercicios::ex_003::rodar_o_exercício(&cabeçalho_do_programa);
    } else if número_do_exercício == 2 {
        exercicios::ex_002::rodar_o_exercício(&cabeçalho_do_programa);
    } else {
        exercicios::ex_001::rodar_o_exercício(&cabeçalho_do_programa);
    }
}

fn menu_de_opções_de_exercícios(cabeçalho_do_programa: &String) {
    loop {
        let nome_de_todos_os_exercícios = vec![
            String::from("ex_001"),
            String::from("ex_002"),
            String::from("ex_003"),
            String::from("ex_004"),
            String::from("ex_005"),
            String::from("ex_006"),
            String::from("ex_007"),
            String::from("ex_008"),
            String::from("ex_009")
        ];

        let tamanho_da_lista_de_exercícios = nome_de_todos_os_exercícios.len().to_string();
        let tamanho_da_lista_de_exercícios: u32 = tamanho_da_lista_de_exercícios.parse().unwrap();

        println!("{}", cabeçalho_do_programa);
        
        println!("          Lista de Exercícios\n");
        
        for (index, exercicio) in nome_de_todos_os_exercícios.into_iter().enumerate() {
            if index % 4 == 0 && index != 0 {
                print!("{exercicio}\n");
            } else {
                print!("{exercicio}   ");
            }
        }
    
        println!("\n\n(Escreva SAIR para fechar o programa)\n'Coloque APENAS o número do exercício'\n\nQual exercício você escolhe?");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().to_lowercase() == "sair" {
                    break;
                }

                match input.trim().parse::<u32>() {
                    Ok(number) => {
                        if number > 0 && number <= tamanho_da_lista_de_exercícios {
                            clean_terminal_linux();
                            
                            println!("{}", cabeçalho_do_programa);
                            
                            println!("\nAbrindo o exercício {}...\n", number);

                            thread::sleep(Duration::from_millis(2000));

                            clean_terminal_linux();

                            // Ficar de olho neste trecho
                            link_dos_exercícios(number, cabeçalho_do_programa);
                            
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);
                            
                            println!("\nErro! Exercício {} não encontrado!\n", number);

                            thread::sleep(Duration::from_millis(1200));

                            clean_terminal_linux();
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);
                            
                        println!("\nErro! Valor inválido digitado!\n");

                        thread::sleep(Duration::from_millis(2000));

                        clean_terminal_linux();
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }

}

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();

    let cabeçalho_do_programa: String = String::from("- Gerenciador De Projetos De Estudo Rust -\n             Por LaLunaInSky               \n");

    // Para o programa final
    menu_de_opções_de_exercícios(&cabeçalho_do_programa);

    // Para Desenolvimento do exercício
    // exercicios::ex_009::rodar_o_exercício(&cabeçalho_do_programa);
}
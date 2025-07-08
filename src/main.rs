use std::{
    io,
    thread,
    time::Duration,
    process::Command,
};

mod exercicios;

fn menu_de_opções_de_exercícios(cabeçalho_do_programa: &String) {
    loop {
        let mut nome_de_todos_os_exercícios = vec![];

        for quantidade_de_exercícios in 1..16 {
            let mut número_formatado = String::new();
            
            if quantidade_de_exercícios < 10 {
                número_formatado = format!("00{}", quantidade_de_exercícios);
            } else if quantidade_de_exercícios < 100 {
                número_formatado = format!("0{}", quantidade_de_exercícios);
            } else {
                número_formatado = format!("{}", quantidade_de_exercícios);
            }

            let nome = format!(
                "ex_{}", número_formatado
            );

            nome_de_todos_os_exercícios.push(nome);
        }

        let tamanho_da_lista_de_exercícios = nome_de_todos_os_exercícios.len().to_string();
        let tamanho_da_lista_de_exercícios: u32 = tamanho_da_lista_de_exercícios.parse().unwrap();

        println!("{}", cabeçalho_do_programa);
        
        println!("          Lista de Exercícios\n");
        
        for (index, exercicio) in nome_de_todos_os_exercícios.into_iter().enumerate() {
            if index > 0 && index % 5 == 4{
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
                            // link_dos_exercícios(number, cabeçalho_do_programa);
                            exercicios::executar_o_exercício_x(number, &cabeçalho_do_programa);
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

    // Rodar o menu de exercícios
    menu_de_opções_de_exercícios(&cabeçalho_do_programa);

    // Rodar apenaso exercício X
    // exercicios::executar_o_exercício_x(15, &cabeçalho_do_programa);
}
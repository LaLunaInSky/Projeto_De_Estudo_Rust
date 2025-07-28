use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command,
};

mod exercicios;

fn construir_a_lista_de_exercícios(cabeçalho_do_programa: &String, total_de_exercícios: &u32) {
    let mut nome_de_todos_os_exercícios = vec![];

    for quantidade_de_exercícios in 1..(total_de_exercícios + 1) {
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

    println!("{}", cabeçalho_do_programa);
    
    println!("          Lista de Exercícios\n");
    
    for (index, exercicio) in nome_de_todos_os_exercícios.into_iter().enumerate() {
        if index > 0 && index % 5 == 4{
            print!("{exercicio}\n");
        } else {
            print!("{exercicio}   ");
        }
    }

    println!(
        "\n\n(Escreva SAIR para fechar o programa)\n\"Coloque APENAS o número do exercício\"\n"
    );
}

fn menu_de_opções_de_exercícios(cabeçalho_do_programa: &String, total_de_exercícios: u32) {
    let mut complemento_da_pergunta = String::new();
    
    loop {

        construir_a_lista_de_exercícios(&cabeçalho_do_programa, &total_de_exercícios);    

        println!(
            "{}Qual exercício você escolhe?",
            complemento_da_pergunta
        );
    
        let mut input = String::new();
    
        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().to_lowercase() == "sair" {
                    break;
                }

                match input.trim().parse::<u32>() {
                    Ok(number) => {
                        if number >= 1 && number <= total_de_exercícios {
                            clean_terminal_linux();
                            
                            println!("{}", cabeçalho_do_programa);
                            
                            println!("\nAbrindo o exercício {}...\n", number);
    
                            sleep(Duration::from_millis(2000));
    
                            clean_terminal_linux();
    
                            exercicios::executar_o_exercício_x(number, &cabeçalho_do_programa);

                            complemento_da_pergunta = String::new();

                            clean_terminal_linux();
                        } else {
                            clean_terminal_linux();

                            complemento_da_pergunta = format!(
                                "Erro! Exercício {} não encontrado!\n\n",
                                number
                            )
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        complemento_da_pergunta = String::from("Erro! Valor inválido digitado!\n\n");
                    }
                }
            }
            Err(_) => println!("Error!"),
        }
    }

}

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();

    /* Alterar para o número do último exercício em modelagem */
    let total_de_exercícios: u32 = 42;

    /* Não precisa mexer */
    let último_exercício = &total_de_exercícios;

    let cabeçalho_do_programa = String::from("- Gerenciador De Projetos De Estudo Rust -\n             Por LaLunaInSky               \n");

    // Rodar o menu de exercícios
    // menu_de_opções_de_exercícios(&cabeçalho_do_programa, total_de_exercícios);

    // Rodar apenas o exercício X
    exercicios::executar_o_exercício_x(*último_exercício, &cabeçalho_do_programa);
}
use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};

pub mod exercicios;
pub mod recursos;
pub mod lista_de_exercicios;

use recursos::{
    limpar_terminal::limpar_terminal
};

fn construir_a_lista_de_exercícios(
    cabeçalho_do_programa: &String, 
    total_de_exercícios: &u32
) {
    let mut nome_de_todos_os_exercícios = vec![];

    for quantidade_de_exercícios in 1..(total_de_exercícios + 1) {
        let mut _número_formatado = String::new();
        
        if quantidade_de_exercícios < 10 {
            _número_formatado = format!("00{}", quantidade_de_exercícios);
        } else if quantidade_de_exercícios < 100 {
            _número_formatado = format!("0{}", quantidade_de_exercícios);
        } else {
            _número_formatado = format!("{}", quantidade_de_exercícios);
        }

        let nome = format!(
            "ex_{}", _número_formatado
        );

        nome_de_todos_os_exercícios.push(nome);
    }

    println!(
        "{}",
        cabeçalho_do_programa
    );
    
    println!(
        "{:^42}\n",
        "Lista de Exercícios"
    );
    
    for (
        index, 
        exercicio
    ) in nome_de_todos_os_exercícios.into_iter().enumerate() {
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

fn menu_de_opções_de_exercícios(
    cabeçalho_do_programa: &String,
    total_de_exercícios: u32
) {
    let mut complemento_da_pergunta = String::new();
    
    loop {
        construir_a_lista_de_exercícios(
            &cabeçalho_do_programa, 
            &total_de_exercícios);    

        println!(
            "{}Qual exercício você escolhe?",
            complemento_da_pergunta
        );
    
        let mut input = String::new();
    
        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                if input.trim().to_lowercase() == "sair" {
                    break;
                }

                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        if número >= 1 && número <= total_de_exercícios {
                            limpar_terminal();
                            
                            println!(
                                "{}",
                                cabeçalho_do_programa
                            );
                            
                            println!(
                                "\nAbrindo o exercício {}...\n",
                                número
                            );
    
                            sleep(Duration::from_millis(2000));
    
                            limpar_terminal();
    
                            exercicios::executar_o_exercício_x(
                                número, 
                                &cabeçalho_do_programa
                            );

                            complemento_da_pergunta = String::new();

                            limpar_terminal();
                        } else {
                            limpar_terminal();

                            complemento_da_pergunta = format!(
                                "Erro! Exercício {} não encontrado!\n\n",
                                número
                            )
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        complemento_da_pergunta = String::from(
                            "Erro! Valor inválido digitado!\n\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }

}

fn main() {
    limpar_terminal();

    /* Alterar para o número do último exercício em modelagem */
    let total_de_exercícios: u32 = 63;

    /* Não precisa mexer */
    let _último_exercício = &total_de_exercícios;

    let cabeçalho_do_programa = format!(
        "- Gerenciador De Projetos De Estudo Rust -\n{:^42}\n",
        "Por LaLunaInSky"
    );

    // Rodar o menu de exercícios
    menu_de_opções_de_exercícios(&cabeçalho_do_programa, total_de_exercícios);

    // Rodar apenas o exercício X
    // exercicios::executar_o_exercício_x(*_último_exercício, &cabeçalho_do_programa);

    
}
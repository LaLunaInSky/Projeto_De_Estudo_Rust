use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 011:");
    println!(
        " Um programa que lê o salário de um funcionário e mostra o seu novo salário com 15% de aumento."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do exercício main */
    let salário_do_funcionário = obter_o_salario(&cabeçalho_do_programa);

    calcular_o_aumento_do_salario(&salário_do_funcionário);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando ao menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));  

    clean_terminal_linux();  
}

fn calcular_o_aumento_do_salario(salário: &f32) {
    println!("Calculando o aumento de 15%...\n");

    thread::sleep(Duration::from_millis(2000));

    let novo_salário = salário + (salário * (15.0 / 100.0));

    println!(
        "O novo salário é de R${:.2}.",
        novo_salário
    );
}

fn obter_o_salario(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite o salário do funcionário:");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(salário) => {
                        if salário > 0.0 {
                            clean_terminal_linux();
    
                            println!("{}", cabeçalho_do_programa);
    
                            descrição_do_exercício();

                            let salário_formatado = format!(
                                "{:.2}", salário
                            );

                            match salário_formatado.parse::<f32>() {
                                Ok(salário_final) => {
                                    println!("\nO salário do funcionário é R${:.2}\n", salário_final);
                                
                                    return salário_final;
                                }
                                Err(_) => println!("Erro!"),
                            }
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um valor maior que zero!\n");
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um valor válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}
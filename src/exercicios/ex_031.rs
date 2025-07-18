use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 031:");
    println!(
        " Um programa que pergunta o salário de\num funcinário e calcule o valor do seu\naumento. Para salários superiores a\nR$1.250,00, calcule um aumento de 10%.\n Para os inferiores ou iguais, o aumento\né de 15%."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* corpo do Exercício - fn main */
    let salário_do_funcionário = obter_o_salario_de_um_functionário(&cabeçalho_do_programa);

    calcular_o_aumento_do_salário_do_funcionário(&salário_do_funcionário);

    sleep(Duration::from_millis(1500));

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn calcular_o_aumento_de_x_porcento(salário_do_funcionário: &f32, taxa_do_aumento: &f32) -> f32 {
    let novo_salário = salário_do_funcionário + (salário_do_funcionário * (taxa_do_aumento / 100.0));

    return novo_salário;
}

fn calcular_o_aumento_do_salário_do_funcionário(salário_do_funcionário: &f32) {
    let mut novo_salário: f32 = 0.0;
    let mut taxa_de_aumento: f32 = 0.0;

    if *salário_do_funcionário <= 1250.0 {
        taxa_de_aumento = 15.0;

        novo_salário = calcular_o_aumento_de_x_porcento(&salário_do_funcionário, &taxa_de_aumento);

    } else {
        taxa_de_aumento = 10.0;
        
        novo_salário = calcular_o_aumento_de_x_porcento(&salário_do_funcionário, &taxa_de_aumento);
        
    }

    sleep(Duration::from_millis(1000));

    println!("Calculando o aumento...\n");

    sleep(Duration::from_millis(2500));

    println!(
        "O novo salário com {}% de aumento,\nfica R${:.2}.",
        taxa_de_aumento,
        novo_salário
    );
}

fn obter_o_salario_de_um_functionário(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite o salário de um funcionário:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(salário) => {
                        let salário_formatado = format!(
                            "{:.2}",
                            salário
                        );

                        match salário_formatado.parse::<f32>() {
                            Ok(salário_final) => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nO salário de R${:.2},\nadicionado com sucesso!\n",
                                    salário_final
                                );

                                return salário_final
                            }
                            Err(_) => println!("Erro!"),
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas números!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}
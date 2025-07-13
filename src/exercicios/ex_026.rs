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
    println!("Descrição do exercício 026:");
    println!(
        " Um programa que lê a velocidade de\num carro, se o mesmo ultrapassar 80Km/h,\nmostre uma mensagem dizendo que ele foi\nmultado.
 A multa vai custar R$7,00 por cada\nquilometro acima do limite."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let velocidade_do_carro = obter_a_velocidade_do_carro(&cabeçalho_do_programa);

    verificar_se_a_velocidade_passou_do_limite(&velocidade_do_carro);

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn calcular_o_valor_da_multa(velocidade_do_carro: &f32) {
    let valor_da_multa = (velocidade_do_carro - 80.0) * 7.0;

    sleep(Duration::from_millis(1200));

    println!("Calculando o valor da multa...\n");
        
    sleep(Duration::from_millis(2200));

    println!(
        "O valor da multa é de R${:.2}",
        valor_da_multa
    );
}

fn verificar_se_a_velocidade_passou_do_limite(velocidade_do_carro: &f32) {
    sleep(Duration::from_millis(2000));

    if *velocidade_do_carro > 80.0 {
        println!("O Carro foi multado!\n");

        calcular_o_valor_da_multa(&velocidade_do_carro);

    } else {
        println!("Não se esqueca do sinto de segurança!");
    }

    sleep(Duration::from_millis(2500));
}

fn obter_a_velocidade_do_carro(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite a velocidade do carro:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(velocidade) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        let velocidade_formatada = format!(
                            "{:.1}",
                            velocidade
                        );

                        match velocidade_formatada.parse::<f32>() {
                            Ok(velocidade_final) => {
                                println!(
                                    "\nVelocidade de {:.1}km/h,\nadiconada com sucesso!\n",
                                    velocidade_final
                                );

                                return velocidade_final;
                            }
                            Err(_) => println!("Erro!"),
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um número válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}
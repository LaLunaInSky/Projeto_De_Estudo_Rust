use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 012:");
    println!(
        " Um programa que converte uma temperatura\nem °C para °F."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do exercício main */
    let temperatura_em_celsius = obter_a_temperatura(&cabeçalho_do_programa);

    thread::sleep(Duration::from_millis(1500));

    converter_celsius_em_fahrenheit(&temperatura_em_celsius);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando ao menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn converter_celsius_em_fahrenheit(temperatura_em_celsius: &f32) {
    println!("Convertendo para Farenheit...\n");

    thread::sleep(Duration::from_millis(2000));

    let temperatura_em_farenheit: f32 = (*temperatura_em_celsius * (9.0 / 5.0)) + 32.0;

    println!(
        "A temperatura é de {:.1}°F",
        temperatura_em_farenheit
    );
}

fn obter_a_temperatura(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite a temperatuda em °C:");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(temperatura) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        let temperatura_formatada = format!("{:.1}", temperatura);
                        
                        match temperatura_formatada.parse::<f32>() {
                            Ok(temperatura_final) => {
                                println!("\nTemperatura de {:.1}°C,\nAdicionada com sucesso!\n", temperatura_final);

                                return temperatura_final
                            }
                            Err(_) => println!("Erro!"),
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
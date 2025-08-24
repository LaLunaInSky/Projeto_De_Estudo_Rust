use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::descrição_de_exercício,
    exercicio_informacoes::Exercício_Informações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("012"),
            String::from("Um programa que converte uma temperatura\nem °C para °F.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let temperatura_em_celsius = obter_a_temperatura(
            &exercício_informações
        );

        converter_celsius_em_fahrenheit(
            &temperatura_em_celsius
        );
    
        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn converter_celsius_em_fahrenheit(
    temperatura_em_celsius: &f32
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Convertendo para Farenheit...\n"
    );

    sleep(Duration::from_millis(1500));

    let temperatura_em_farenheit: f32 = (
        *temperatura_em_celsius * (9.0 / 5.0)
    ) + 32.0;

    println!(
        "A temperatura é de {:.1}°F\n",
        temperatura_em_farenheit
    );

    sleep(Duration::from_millis(1100));
}

fn obter_a_temperatura(
    exercício_informações: &Exercício_Informações
) -> f32 {
    loop {
        println!(
            "Digite a temperatuda em °C:"
        );
    
        let mut input = String::new();
    
        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(temperatura) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        let temperatura_formatada = format!(
                            "{:.1}", 
                            temperatura
                        );
                        
                        match temperatura_formatada.parse::<f32>() {
                            Ok(temperatura_final) => {
                                println!(
                                    "Temperatura de {:.1}°C,\nfoi adicionada com sucesso!\n", 
                                    temperatura_final
                                );

                                return temperatura_final
                            }
                            Err(_) => (),
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um número real!\n"
                        );
                    }
                 }
            }
            Err(_) => (),
        }
    }
}
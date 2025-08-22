use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    exercicio_informacoes::Exercício_Informações,
    descricao_de_exercicio::descrição_de_exercício,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("006"),
            String::from("Um programa que lê um valor em metros e o\nexibe convertido em todos os tipos a\nseguir:

km <- hm <- dam <- m -> dm -> cm -> mm")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número_input_em_metros = obter_o_número_float(
            &exercício_informações
        );

        converter_o_valor_de_metros(
            &número_input_em_metros
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
        "\nVoltando para o menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn converter_o_valor_de_metros(
    valor_em_metros: &f32
) {
    println!(
        "Convertendo o valor...\n"
    );

    sleep(Duration::from_millis(2000));

    println!(
        "km...: {}", 
        (valor_em_metros / 1000.0)
    );

    sleep(Duration::from_millis(500));

    println!(
        "hm...: {}", 
        (valor_em_metros / 100.0)
    );

    sleep(Duration::from_millis(500));

    println!(
        "dam..: {}", 
        (valor_em_metros / 10.0)
    );

    sleep(Duration::from_millis(500));

    println!(
        "m....: {}", 
        valor_em_metros
    );

    sleep(Duration::from_millis(500));

    println!(
        "dm...: {}", 
        (valor_em_metros * 10.0)
    );

    sleep(Duration::from_millis(500));

    println!(
        "cm...: {}", 
        (valor_em_metros * 100.0)
    );

    sleep(Duration::from_millis(500));

    println!(
        "mm...: {}\n", 
        (valor_em_metros * 1000.0)
    );

    sleep(Duration::from_millis(1500));
}

fn obter_o_número_float(
    exercício_informações: &Exercício_Informações
) -> f32 {
    loop {
        println!(
            "Digite um valor em metros:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas número reais!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    exercicio_informacoes::ExercícioInformações,
    descricao_de_exercicio::descrição_de_exercício,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

mod comprimentos;

use comprimentos::Comprimentos;

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
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
        let comprimentos = Comprimentos::new(
            obter_o_número_float(
                &exercício_informações
            )
        );

        analisar_os_comprimentos(
            &comprimentos
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

fn analisar_os_comprimentos(
    comprimentos: &Comprimentos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Convertendo o valor...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "km...: {}", 
        comprimentos.km
    );

    sleep(Duration::from_millis(500));

    println!(
        "hm...: {}", 
        comprimentos.hm
    );

    sleep(Duration::from_millis(500));

    println!(
        "dam..: {}", 
        comprimentos.dam
    );

    sleep(Duration::from_millis(500));

    println!(
        "m....: {}", 
        comprimentos.m
    );

    sleep(Duration::from_millis(500));

    println!(
        "dm...: {}", 
        comprimentos.dm
    );

    sleep(Duration::from_millis(500));

    println!(
        "cm...: {}", 
        comprimentos.cm
    );

    sleep(Duration::from_millis(500));

    println!(
        "mm...: {}\n", 
        comprimentos.mm
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_número_float(
    exercício_informações: &ExercícioInformações
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
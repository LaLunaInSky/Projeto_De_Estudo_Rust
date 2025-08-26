use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod numero;

use numero::Número;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("020"),
            String::from("Um programa que lê um número de 0 a 9999\ne mostra no terminal cada um dos dígitos\nseparados.

Exemplo: 
 1834
 unidade.: 4
 dezena..: 3
 centena.: 8
 milhar..: 1")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número = Número::new(
            obter_o_número_inteiro(
                &exercício_informações
            )   
        );

        analisar_o_número(
            &número
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn analisar_o_número(
    número: &Número
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o número...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Unidade.: {}", 
        número.unidade
    );

    sleep(Duration::from_millis(500));

    println!(
        "Dezena..: {}",
        número.dezena
    );

    sleep(Duration::from_millis(500));

    println!(
        "Centena.: {}",
        número.centena
    );

    sleep(Duration::from_millis(500));

    println!(
        "Milhar..: {}\n", 
        número.milhar
    );

    sleep(Duration::from_millis(1500));
}

fn obter_o_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u16 {
    loop {
        println!(
            "Digite um número inteiro [0 à 9999]:"
        );

        let mut input = String::new();
    
        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u16>() {
                    Ok(número) => {
                        if número <= 9999 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O número {},\nfoi adicionado com sucesso!\n",
                                número
                            );

                            return número;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um número de até 9999!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();
                    
                        println!(
                            "Erro! Digite um número inteiro!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
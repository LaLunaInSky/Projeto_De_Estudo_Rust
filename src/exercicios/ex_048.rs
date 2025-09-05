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

mod progressao_aritmetica;

use progressao_aritmetica::PA;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("048"),
            String::from("Um programa que lê o primeiro termo e a\nrazão de uma PA. No final, mostra os 10\nprimeiros termos dessa progressão.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let pa = PA::new(
            obter_um_número(
                &exercício_informações,
                "primeiro_termo"
            ),
            obter_um_número(
                &exercício_informações,
                "razão"
            )
        );

        analisar_a_pa(
            &pa
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

fn analisar_a_pa(
    pa: &PA
) {
    sleep(Duration::from_millis(1000));
    
    println!(
        "Analisando a PA..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Primeiro Termo: {}
A Razão.......: {}
",
        pa.get_primeiro_termo(),
        pa.get_razão()
    );

    sleep(Duration::from_millis(800));

    println!(
        "Logo os 10 primeiros termos são...\n"
    );

    sleep(Duration::from_millis(800));

    for (
        index, número
    ) in pa.get_dez_primeiros_termos().iter().enumerate() {
        if index == 9 {
            print!(
                "{}.\n\n", 
                número
            );
        }else if index == 4 && index > 0 {
            print!(
                "{}\n",
                número
            )
        } else {
            print!(
                "{}, ",
                número
            )
        }
    }

    sleep(Duration::from_millis(1100));
}

fn obter_um_número(
    exercício_informações: &ExercícioInformações,
    argumento_de_chamada: &str
) -> u32 {
    loop {
        println!(
            "Digite {}",
            if argumento_de_chamada == "razão" {
                "a Razão:"
            } else {
                "o primeiro termo:"
            }
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        if argumento_de_chamada == "razão" {
                            match número {
                                0 => {
                                    println!(
                                        "A razão de 1,\nfoi adicionada com sucesso!\n"
                                    );

                                    return 1;
                                }
                                _ => {
                                    println!(
                                        "A razão de {},\nfoi adicionada com sucesso!\n",
                                        número
                                    );

                                    return número;
                                }
                            }
                        } else {
                            println!(
                                "O primeiro termo {},\nfoi adicionada com sucesso!\n",
                                número
                            );

                            return número;
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas número!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
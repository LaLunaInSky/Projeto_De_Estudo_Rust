use std::{
    io::stdin, 
    thread::sleep, 
    time::Duration,
    f64::consts::PI 
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("016")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let angulo_inforamado = obter_o_angulo(
            &exercício_informações
        );

        calcular_o_seno_o_cosseno_e_a_tangente_do_angulo(
            &angulo_inforamado
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

fn calcular_o_seno_o_cosseno_e_a_tangente_do_angulo(
    angulo: &f64
) {
    let x = (angulo * PI) / 180.0;
    let seno: f64 = x.sin();
    let cosseno = x.cos();

    let mut tangente: Option<f64> = None;
    if *angulo != 90.0 {
        tangente = Some(x.tan());
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o ângulo...\n"
    );

    sleep(Duration::from_millis(1500));
    
    println!(
        "O seno......: {:.4}",
        seno
    );

    sleep(Duration::from_millis(500));

    println!(
        "O cosseno...: {:.4}",
        cosseno
    );

    sleep(Duration::from_millis(500));

    match tangente {
        Some(result) => {
            println!(
                "A tangente..: {:.4}\n",
                result
            );
        }
        None => {
            println!(
                "A tangente..: -\n",
                
            );
        }
    }


    sleep(Duration::from_millis(1100));
}

fn obter_o_angulo(
    exercício_informações: &ExercícioInformações
) -> f64 {
    loop {
        println!(
            "Digite um ângulo:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(angulo) => {
                        if angulo <= 90 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Ângulo de {}° graus,\nfoi adicionado com sucesso!\n",
                                angulo
                            );

                            return angulo as f64;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um ângulo de até 90°!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!("\nErro! Digite um número real!\n");
                    }
                }
            }
            Err(_) => (),
        }
    }
}
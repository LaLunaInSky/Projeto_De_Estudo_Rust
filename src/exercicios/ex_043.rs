use std::{
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    descricao_de_exercicio::criar_descrição_do_exercício,
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
        criar_descrição_do_exercício(
            String::from("043"),
            String::from("Um programa que mostra no terminal uma\ncontagem regressiva para o estoura de\nfogos de artifícios, indo de 10 até 0,\ncom uma pausa de 1 segundo entre eles.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        obter_contagem_regressiva();

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

fn obter_contagem_regressiva() {
    let mut count = 10;
    
    while count > 0 {
        sleep(Duration::from_secs(1));

        println!("{}", count);

        count -= 1;
    }

    sleep(Duration::from_secs(1));

    println!("\nBoom boom!\n");

    sleep(Duration::from_millis(1100));
}
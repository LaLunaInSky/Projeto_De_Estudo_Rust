use crate::recursos::{
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
            String::from("044")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let números_pares: Vec<u8> = obter_o_números_pares_até_x();

        println!(
            "Os números pares são:"
        );

        mostrar_resultado(
            &números_pares
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

fn mostrar_resultado(
    números: &Vec<u8>
) {
    for (index, número) in números.iter().enumerate() {
        if index % 11 == 0 && index > 0 {
            print!("{número}, \n");
        } else if index == (números.len() - 1) {
            println!("{número}.\n")
        } else {
            print!("{número}, ");
        }
    }
}

fn obter_o_números_pares_até_x() -> Vec<u8> {
    let mut números_pares: Vec<u8> = vec![];

    for número in 1..51 {
        if número % 2 == 0 {
            números_pares.push(número);
        }
    }

    return números_pares;
}
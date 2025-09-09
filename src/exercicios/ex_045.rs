use std::{
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod numeros_impares;

use numeros_impares::NúmerosÍmpares;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("045")
        )
    ); 

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let números_ímpares = NúmerosÍmpares::new(
            obter_números_ímpares_até_quinhentos()
        );

        analisar_números(
            &números_ímpares
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

fn analisar_números(
    números_ímpares: &NúmerosÍmpares
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números ímpares...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os Números ímpares:\n{:?}\n\nA soma de todos os números é: {}.\n",
        números_ímpares.get_lista_de_números_ímpares(),
        números_ímpares.get_soma_dos_números_ímpares()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_números_ímpares_até_quinhentos() -> Vec<u32> {
    let mut números_ímpares: Vec<u32> = vec![];

    for número in 1..501 {
        if número % 2 != 0 {
            números_ímpares.push(número);
        }
    }

    return números_ímpares;
}
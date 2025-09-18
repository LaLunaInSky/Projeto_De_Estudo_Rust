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

mod sortador_de_numeros;

use sortador_de_numeros::SorteadorDeNúmeros;

// 74!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("068")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let números_sorteados = SorteadorDeNúmeros::new();

        analisar_os_números(
            &números_sorteados
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

fn analisar_os_números(
    números_sorteados: &SorteadorDeNúmeros
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Os números sorteados são:
{:?};

O menor número é o: {};
O maior número é o: {}.
",
        números_sorteados.get_lista_de_números_sorteados(),
        números_sorteados.get_menor_número_sorteado(),
        números_sorteados.get_maior_número_sorteado()
    );

    sleep(Duration::from_millis(1100));
}
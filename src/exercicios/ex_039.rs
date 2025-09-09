use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod tipos_de_triangulos;
mod segmentos;

use segmentos::Segmentos;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("039")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut segmentos = Segmentos::new();

        for indice in 1..4 {
            segmentos.adicionar_um_segmento(
                obter_o_lado_de_um_suposto_triângulo(
                    &exercício_informações, 
                    indice
                ),
                (indice - 1) as usize
            );
        }

        analisar_os_segmentos(
            &segmentos
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

fn analisar_os_segmentos(
    segmentos: &Segmentos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os segmentos...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O segmentos {:?},\n{}formam um triângulo!\n",
        segmentos.get_lista_de_segmentos(),
        if !segmentos.get_forma_um_triângulo() {"NÃO "} else {""}
    );

    if segmentos.get_forma_um_triângulo() {
        println!(
            "Este é um triângulo {}!\n",
            segmentos.get_tipo_do_triângulo()
        )
    }

    sleep(Duration::from_millis(1100));
}

fn obter_o_lado_de_um_suposto_triângulo(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o {}º Segmento:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(segmento) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O segmento de {},\nfoi adicionado com sucesso!\n",
                            segmento
                        );

                        return segmento;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
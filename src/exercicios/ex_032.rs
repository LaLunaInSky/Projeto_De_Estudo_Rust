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

mod segmentos;

use segmentos::Segmentos;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("032")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut segmentos = Segmentos::new();

        for indice in 1..4 {
            segmentos.adicionar_novo_segmento(
                (indice - 1) as usize,
                obter_um_segmento_de_um_suposto_triângulo(
                    &exercício_informações, 
                    indice
                )
            );
        }

        analisar_os_segmentos(
            &mut segmentos
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
    segmentos: &mut Segmentos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os segmentos.."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Os segmentos {:?},
{}podem formar um triângulo!
",
        segmentos.get_lista_de_segmentos(),
        if !segmentos.get_forma_um_triângulo() {"Não "} else {""}
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_segmento_de_um_suposto_triângulo(
    exercício_informações: &ExercícioInformações, 
    indice_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o {}º Segmento:",
            indice_da_chamada
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
                            "O segmento {},\nfoi adicionado com sucesso!\n",
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
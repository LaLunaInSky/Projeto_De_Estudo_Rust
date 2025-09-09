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

mod numeros;

use numeros::Números;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("030")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números = Números::new();

        for index in 1..4 {
            números.adicionar_um_número_na_lista(
                obter_um_número_inteiro(
                    index, 
                    &exercício_informações
                )
            );
        }

        analisar_os_números(
            &números
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
    números: &Números
) {
    sleep(Duration::from_millis(1000));

    println!("Analisando os números...\n");

    sleep(Duration::from_millis(1500));
    
    println!(
        "{:?}", 
        números.get_lista_de_números()
    );

    sleep(Duration::from_millis(500));

    println!(
        "
Maior.: {}
Menor.: {}
", 
    números.get_número_maior(),
    números.get_número_menor()
);

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro(
    index_da_chamada: u8, 
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
        println!(
            "Digite o {}º Número:", 
            index_da_chamada
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

                        println!(
                            "Número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
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
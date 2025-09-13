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

mod valor_a_sacar;

use valor_a_sacar::ValorASacar;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("065")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        mostrar_slogan();

        let valor_a_sacar = ValorASacar::new(
            obter_um_número_inteiro(
                &exercício_informações
            )
        );

        analisar_o_valor_a_sacar(
            &valor_a_sacar
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

fn analisar_o_valor_a_sacar(
    valor_a_sacar: &ValorASacar
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o valor..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
{:^42}
R$ 50.: {}
R$ 20.: {}
R$ 10.: {}
R$ 1..: {}
",
        "Quantidade de Cédulas",
        valor_a_sacar.get_quantidade_de_notas_de_cinquenta(),
        valor_a_sacar.get_quantidade_de_notas_de_vinte(),
        valor_a_sacar.get_quantidade_de_notas_de_dez(),
        valor_a_sacar.get_quantidade_de_notas_de_um()
    );

    sleep(Duration::from_millis(1100));
}

fn mostrar_slogan() {
    println!(
        "{:^42}\n",
        " CAIXA ELETRÔNICO "
    );
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
        println!(
            "Quanto quer sacar?"
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

                        mostrar_slogan();

                        println!(
                            "O valor de R${},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        mostrar_slogan();

                        println!(
                            "Erro! Digite apenas números inteiros.\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}
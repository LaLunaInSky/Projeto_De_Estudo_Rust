use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::descrição_de_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

// fn descrição_do_exercício() {
//     println!("Descrição do exercício 010:");
//     println!(
//         " "
//     );
// }

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("010"),
            String::from("Um programa que lê o preço de um produto\ne mostra seu novo preço com 5% de desconto.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício*/
        let preço_do_produto = obter_o_preco_de_um_produto(
            &exercício_informações
        );

        calcular_o_desconto_do_produto(
            &preço_do_produto
        );

        let resposta_da_pergunta = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_da_pergunta {
            break;
        }
    }

    /* Fim do Programa */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    limpar_terminal();
}

fn calcular_o_desconto_do_produto(
    valor_do_produto: &f32
) {
    sleep(Duration::from_millis(1000));

    let valor_com_o_desconto = valor_do_produto - (
        valor_do_produto * (5.0 / 100.0)
    );

    println!(
        "Calculando o desconto de 5%...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O produto fica R${:.2}\n",
        valor_com_o_desconto
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_preco_de_um_produto(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite o preço do produto:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        if número > 0.0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            let número_formatado = format!(
                                "{:.2}", número
                            );

                            match número_formatado.parse::<f32>() {
                                Ok(número_final) => {
                                    println!(
                                        "Preço de R${:.2},\nfoi adicionado com sucesso!\n",
                                        número_final
                                    );

                                    return número_final;
                                }
                                Err(_) => (),
                            }
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um valor maior que zero!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um número real!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
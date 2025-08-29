use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod salario_do_funcionario;

use salario_do_funcionario::SalárioDoFuncionário;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("031"),
            String::from("Um programa que pergunta o salário de\num funcinário e calcule o valor do seu\naumento. Para salários superiores a\nR$1.250,00, calcule um aumento de 10%.\n Para os inferiores ou iguais, o aumento\né de 15%.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let salário_do_funcionário = SalárioDoFuncionário::new(
            obter_o_salario_de_um_functionário(
                &exercício_informações
            )
        );

        analisar_o_salário(
            &salário_do_funcionário
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

fn analisar_o_salário(
    salário_do_funcionário: &SalárioDoFuncionário
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Calculando o aumento...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O novo salário com {}% de aumento,
fica R${:.2}.
",
        salário_do_funcionário.get_taxa_de_aumento(),
        salário_do_funcionário.get_salário_novo()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_salario_de_um_functionário(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite o salário de um funcionário:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(salário) => {
                        let salário_formatado = format!(
                            "{:.2}",
                            salário
                        );

                        match salário_formatado.parse::<f32>() {
                            Ok(salário_final) => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "O salário de R${:.2},\nfoi adicionado com sucesso!\n",
                                    salário_final
                                );

                                return salário_final
                            }
                            Err(_) => (),
                        }
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
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

mod emprestimo_imobiliario;

use emprestimo_imobiliario::EmpréstimoImobiliário;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("033")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let empréstimo_imobiliário = EmpréstimoImobiliário::new(
            obter_valor_da_casa(
                &exercício_informações
            ),
            obter_o_salário_do_comprador(
                &exercício_informações
            ),
            obter_a_quantidade_de_anos_para_pagar_a_casa(
                &exercício_informações
            )
        );

        analisar_se_é_possivel_o_empréstimo(
            &empréstimo_imobiliário
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

fn analisar_se_é_possivel_o_empréstimo(
    empréstimo_imobiliário: &EmpréstimoImobiliário
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a liberação do Empréstimo...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O empréstimo está {}!\n",
        if empréstimo_imobiliário.get_empréstimo_liberado() {"LIBERADO"} else {"NEGADO"}
    );

    sleep(Duration::from_millis(1100));
}

fn obter_a_quantidade_de_anos_para_pagar_a_casa(
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "Em quantos anos quer pagar a casa?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(anos) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "A quantidade de {} anos,\nfoi adicionada com sucesso!\n",
                            anos
                        );

                        return anos;
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

fn obter_o_salário_do_comprador(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Qual o salário do comprador?"
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

                                return salário_final;
                            }
                            Err(_) => println!("Erro!"),
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

fn obter_valor_da_casa(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Qual o valor da casa?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(valor) => {
                        let valor_formatado = format!(
                            "{:.2}",
                            valor
                        );

                        match valor_formatado.parse::<f32>() {
                            Ok(valor_final) => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "O valor da casa de R${:.2},\nfoi adicionado com sucesso!\n",
                                    valor_final
                                );

                                return valor_final
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
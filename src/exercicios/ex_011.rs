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

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("011")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let salário_do_funcionário = obter_o_salario(
            &exercício_informações
        );

        calcular_o_aumento_do_salario(
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

fn calcular_o_aumento_do_salario(
    salário: &f32
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Calculando o aumento de 15%...\n"
    );

    sleep(Duration::from_millis(1500));

    let novo_salário = salário + (
        salário * (15.0 / 100.0)
    );

    println!(
        "O novo salário é de R${:.2}.\n",
        novo_salário
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_salario(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite o salário do funcionário:"
        );
    
        let mut input = String::new();
    
        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(salário) => {
                        if salário > 0.0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            let salário_formatado = format!(
                                "{:.2}", 
                                salário
                            );

                            match salário_formatado.parse::<f32>() {
                                Ok(salário_final) => {
                                    println!(
                                        "O salário de R${:.2}\nfoi adicionado com sucesso.\n",
                                        salário_final
                                    );
                                
                                    return salário_final;
                                }
                                Err(_) => println!("Erro!"),
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
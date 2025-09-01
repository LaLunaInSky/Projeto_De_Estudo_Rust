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

mod pessoa;
mod arredondador_de_numeros_reais;

use pessoa::Pessoa;
use arredondador_de_numeros_reais::arrendondar_um_número_real;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("040"),
            String::from("Desenvolva uma lógica que lê o peso e a\naltura de uma pessoa, calcula o seu IMC e\nmostra seu status corporal, de acordo com\na tabela abaixo:

- Abaixo de  18.5: Abaixo do peso
- Entre 18.5 e 25: Peso ideal
- Entre 25 e 30: Sobrepeso
- Entre 30 e 40: Obesidade
- Acima de 40: Obesidade mórbida")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let dados_da_pessoa = Pessoa::new(
            obter_um_número_real(
                &exercício_informações, 
                "peso"
            ),
            obter_um_número_real(
                &exercício_informações,
                "altura"
            )
        );

        analisar_o_imc_e_o_status_da_pessoa(
            &dados_da_pessoa
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

fn analisar_o_imc_e_o_status_da_pessoa(
    pessoa: &Pessoa
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o IMC da pessoa...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "A pessoa com o peso de {:.2}kg,\ne a altura de {:.2}m,\nestá com o IMC de {}.\n",
        pessoa.get_peso(), 
        pessoa.get_altura(), 
        pessoa.get_imc()
    );

    sleep(Duration::from_millis(1000));

    println!(
        "Status corporal é de {}!\n",
        pessoa.get_status_corporal()
    );    

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_real(
    exercício_informações: &ExercícioInformações, 
    tipo_do_input: &str
) -> f32 {
    loop {
        println!(
            "Digite {}:",
            if tipo_do_input == "peso" {
                format!(
                    "o {}", 
                    tipo_do_input
                )
            } else {
                format!(
                    "a {}", 
                    tipo_do_input
                )
            }
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        if tipo_do_input == "altura" {
                            if número < 2.25 {
                                let número_final = arrendondar_um_número_real(
                                    número, 2
                                );

                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "A {} de {:.2}m,\nfoi adicionada com sucesso!\n", tipo_do_input,
                                    número_final

                                );

                                return número_final;
                            } else {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Erro! Apenas é aceito altura de até 2.25m!\n"
                                );
                            } 
                        } else {
                            let número_final = arrendondar_um_número_real(
                                número, 
                                2
                            );

                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O {} de {:.2}Kg,\nfoi adicionado com sucesso!\n", tipo_do_input,
                                número_final

                            );

                            return número_final;
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas números!\n"
                        )
                    }
                }
            }
            Err(_) => (),
        }
    }
}
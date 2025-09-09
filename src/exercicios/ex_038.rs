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

mod ano_atual;
mod categorias;
mod atleta;

use ano_atual::AnoAtual;
use atleta::Atleta;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("038")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let dados_do_atleta = Atleta::new(
            obter_o_ano_de_nascimento_do_atleta(
                &exercício_informações
            )
        );

        analisar_a_categoria_do_atleta(
            &dados_do_atleta
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

fn analisar_a_categoria_do_atleta(
    dados_do_atleta: &Atleta
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a categoria do Atleta...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O atleta com {} anos,\nestá na categoria: {}!\n",
        dados_do_atleta.get_idade(),
        dados_do_atleta.get_categoria()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_ano_de_nascimento_do_atleta(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
        println!(
            "Digite o ano de nascimento do atleta:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(ano) => {
                        let ano_atual = AnoAtual::new();
                        let ano_atual = ano_atual.get_ano();

                        let ano_mínimo = ano_atual as u32 - 100;

                        if ano >= ano_mínimo  {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O ano de {},\nfoi adicionado com sucesso!\n",
                                ano
                            );

                            return ano;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Apenas é aceito ano superior que {}!\n",
                                ano_mínimo
                            );
                        }
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
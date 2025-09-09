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
mod pessoa;

use ano_atual::AnoAtual;
use pessoa::Pessoa;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("036")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let pessoa = Pessoa::new(
            obter_o_ano_de_nascimento(
                &exercício_informações,
            )
        );

        analisar_a_idade(
            &pessoa
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

fn analisar_a_idade(
    pessoa: &Pessoa
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o alistamento militar...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "{}\n",
        pessoa.get_situação_do_alistamento_militar()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_ano_de_nascimento(
    exercício_informações: &ExercícioInformações,
) -> u32 {
    loop {
        println!(
            "Digite o ano de nascimento:"
        );
        
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(ano_de_nascimento) => {
                        let ano_atual = AnoAtual::new();
                        let ano_atual = ano_atual.get_ano();

                        if ano_de_nascimento > (ano_atual as u32 - 100) {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O ano de nacimento {},\nfoi adicionado com sucesso!\n",
                                ano_de_nascimento
                            );

                            return ano_de_nascimento;

                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um ano acima de {}!\n",
                                ano_atual - 100
                            );
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
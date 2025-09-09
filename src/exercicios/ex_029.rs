use std:: {
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

use ano_atual::AnoAtual;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("029")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício - fn main */
        let ano_escolhido = obter_um_ano(
            &exercício_informações
        );

        analisar_se_o_ano_é_bissexto(
            &ano_escolhido
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

fn analisar_se_o_ano_é_bissexto(
    ano_digitado: &u16
) {
    let mut ano_é_bissexto = false;

    if *ano_digitado > 1900 {

        if ano_digitado % 4 == 0 {

            if ano_digitado % 100 != 0 {

                ano_é_bissexto = true;
            } else {

                if ano_digitado % 400 == 0{
                    ano_é_bissexto = true;
                }
            }
        }
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o ano...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O ano {}é BISSEXTO!\n", 
        if !ano_é_bissexto {"não "} else {""}
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_ano(
    exercício_informações: &ExercícioInformações
) -> u16 {
    loop {
        println!(
            "[0 para o ano atual]\nDigite um ano:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u16>() {
                    Ok(ano) => {
                        let object_ano = AnoAtual::new();
                        let ano_atual = object_ano.get_ano();

                        if ano >= 1900 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Ano de {},\nescolhido com sucesso!\n",
                                ano
                            );

                            return ano;
                        } else if ano == 0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Ano de {},\nfoi adicionado com sucesso!\n",
                                ano_atual
                            );
                            return ano_atual;
                        }else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um ano que comece em 1900!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um ano válido!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
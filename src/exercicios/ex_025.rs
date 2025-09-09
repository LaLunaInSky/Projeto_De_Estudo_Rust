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

mod jogo;

use jogo::Jogo;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("025")
        )
    );

    /* Corpo do Exercíco - fn main */
    loop {
        exercício_informações.mostrar_informações();

        let jogo = Jogo::new(
            obter_o_palpite(
                &exercício_informações
            )
        );

        analisar_o_palpite(
            &jogo
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(&exercício_informações);

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn analisar_o_palpite(
    jogo: &Jogo
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o palpite..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Computador.: {}
Usuário....: {}
",
        jogo.computador_número,
        jogo.usuário_número
    );

    println!(
        "Você {}!\n",
        if jogo.usuário_acertou {"ACERTOU"} else {"ERROU"}
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_palpite(
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "[Adivinhe o número entre 0 e 5]\nQual o seu palpite?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(número) => {
                        if número <= 5 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O número {},\nfoi adicionado com sucesso!\n",
                                número
                            );

                            return número; 
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um número entre 0 e 5!\n"
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
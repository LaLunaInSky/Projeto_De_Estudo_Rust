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

mod opcoes_de_jogadas;
mod possiveis_ganhadores;
mod jogadas;

use opcoes_de_jogadas::OpçõesDeJogadas;
use jogadas::Jogadas;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("042")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let jogadas = Jogadas::new(
            obter_a_opção_da_escolha(
                &exercício_informações
            )
        );

        analisar_jogada(
            &jogadas
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

fn analisar_jogada(
    jogadas: &Jogadas
) {
    let ganhador = jogadas.get_ganhador();

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando quem ganhou...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Usuário.....: {}
Computador..: {}

Logo...",
        jogadas.get_jogador().to_uppercase(),
        jogadas.get_computador().to_uppercase()
    );

    sleep(Duration::from_millis(1500));

    println!(
        "\n{}\n",
        match ganhador.as_str() {
            "empate" => {
                format!(
                    "Ouve {}!",
                    ganhador.to_uppercase()
                )
            }
            _ => {
                format!(
                    "O ganhador foi o {}!",
                    ganhador.to_uppercase()
                )
            }
        }
    );

    sleep(Duration::from_millis(1100));
} 

fn obter_a_opção_da_escolha(
    exercício_informações: &ExercícioInformações
) -> OpçõesDeJogadas {
    mostrador_de_escolha();
    
    loop {
        println!(
            "Qual você escolhe?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção_escolhida) => {
                        match opção_escolhida {
                            1..=3 => {
                                let jogada_jogador = match opção_escolhida {
                                    1 => OpçõesDeJogadas::PEDRA,
                                    2 => OpçõesDeJogadas::PAPEL,
                                    3 => OpçõesDeJogadas::TESOURA,
                                    _ => OpçõesDeJogadas::PEDRA,
                                };

                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "A opção de {},\nfoi selecionada com sucesso!\n",
                                    match jogada_jogador {
                                        OpçõesDeJogadas::PAPEL => String::from("papel"),
                                        OpçõesDeJogadas::PEDRA => String::from("pedra"),
                                        OpçõesDeJogadas::TESOURA => String::from("tesoura")
                                    }
                                );

                                return jogada_jogador;
                            }
                            _ => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                mostrador_de_escolha();

                                println!(
                                    "Erro! Apenas é aceito de 1 à 3!\n"
                                );
                            }
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        mostrador_de_escolha();

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

fn mostrador_de_escolha() {
    println!(
        " [ 1 ] Pedra
 [ 2 ] Papel
 [ 3 ] Tesoura
"
    )
}
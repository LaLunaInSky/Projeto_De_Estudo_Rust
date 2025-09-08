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

mod jogadas;

use jogadas::Jogadas;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("054"),
            String::from("Melhore o jogo do Ex_025 onde o\ncomputador vai \"pensar\" em um número\nentre 0 e 10. Só que agora o jogador vai\ntentar adivinhar até acertar, mostrando\nno final quantos palpites foram\nnecessários para vencer.")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut jogadas = Jogadas::new();

        loop {
            let palpite_do_jogador: u8 = obter_o_palpite(
                &exercício_informações
            );

            if palpite_do_jogador != jogadas.get_número_do_computador() {
                if palpite_do_jogador < jogadas.get_número_do_computador() {
                    println!(
                        "O número {} é MENOR!\n",
                        palpite_do_jogador
                    );
                } else {
                    println!(
                        "O número {} é MAIOR!\n",
                        palpite_do_jogador
                    );
                }

                jogadas.adicionar_mais_uma_tentativa();
            } else {
                break;
            }
        }

        analisar_as_tentativas(
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

fn analisar_as_tentativas(
    jogadas: &Jogadas
) {
    println!(
        "Você ACERTOU! É o número {}.\n",
        jogadas.get_número_do_computador()
    );

    sleep(Duration::from_millis(800));

    println!(
        "Foram {} tentivas até acertar!\n",
        jogadas.get_tentativas()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_palpite(
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "Qual o seu palpite? [0 até 10]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(palpite) => {
                        match palpite {
                            0..11 => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                return palpite;
                            }
                            _ => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "Erro! Apenas é aceito de 0 à 10!\n"
                                );
                            }
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
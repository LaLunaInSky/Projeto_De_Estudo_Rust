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

mod frase;

use frase::Frase;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("050"),
            String::from("Um programa que lê uma frase qualquer e\nretorna se ela é um palíndromo,\ndesconsiderando os espaços.

Exemplos:
- apos a sopa
- a sacada da casa
- a torre da derrota
- o lobo ama o bolo
- anotaram a data da maratona")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let frase_digita = Frase::new(
            obter_uma_frase(
                &exercício_informações
            )
        );

        analisar_a_frase(
            &frase_digita
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

fn analisar_a_frase(
    frase: &Frase
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a frase...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "A frase sem espaços fica:\n{}\n\nLogo...\n",
        frase.get_frase_sem_espaços()
    );

    sleep(Duration::from_millis(800));

    println!(
        "A frase {}é PALÍNDROMO!\n",
        if !frase.get_é_palíndromo() {
            "NÃO "
        } else {
            ""
        }
    );

    sleep(Duration::from_millis(1100));
}

fn obter_uma_frase(
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite uma frase:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let frase = input.trim().to_lowercase();

                limpar_terminal();

                exercício_informações.mostrar_informações();

                println!(
                    "A frase {},\nfoi adicionada com sucesso!\n",
                    frase
                );

                return frase;
            }
            Err(_) => (),
        }
    }
}
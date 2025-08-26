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

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("023"),
            String::from("Um programa que lê uma frase pelo teclado\nmostra:

- Quantas vezes aparece a letra \"A\";
- Em que posição o \"A\" aparece pela\nprimeira vez;
- Em que posição o \"A\" aparece pela última\nvez.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let frase_digitada = obter_uma_frase(
            &exercício_informações
        );

        analisar_a_frase(
            &frase_digitada
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
    frase_digitada: &String
) {
    let mut quantidade_de_as: u8 = 0;
    let mut posição_do_primeiro_a: u8 = 0;
    let mut posição_do_ultimo_a: u8 = 0;

    for (index, char) in frase_digitada.chars().enumerate() {
        if char == 'a' {
            quantidade_de_as += 1;

            if quantidade_de_as == 1 {
                posição_do_primeiro_a = (index as u8) + 1;
            }

            posição_do_ultimo_a = (index as u8) + 1;
        }
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a frase...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Existe {} \"A\" na frase.",
        quantidade_de_as
    );

    sleep(Duration::from_millis(500));

    if quantidade_de_as > 0 {
        println!(
            "O primeiro \"A\" aparece na posição {}.",
            posição_do_primeiro_a
        );

        sleep(Duration::from_millis(500));

        println!(
            "O último \"A\" aparece na posição {}.\n",
            if quantidade_de_as == 1 {posição_do_primeiro_a} else {posição_do_ultimo_a}
        );

        sleep(Duration::from_millis(1100));
    } else {
        println!(
            "Nenhuma posição do \"A\" na frase.\n",
        );

        sleep(Duration::from_millis(1100));
    }

    
}

fn obter_uma_frase(
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite uma frase:"
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                limpar_terminal();

                exercício_informações.mostrar_informações();

                let frase = input.trim().to_lowercase();

                println!(
                    "A frase \"{}\",\nfoi adicionada com sucesso!\n",
                    frase
                );

                return frase;
            }
            Err(_) => (),
        }
    }
}
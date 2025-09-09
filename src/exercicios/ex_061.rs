use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    final_do_exercicio::rodar_final_do_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("061"),
            String::from("Um programa que mostra a tabuada de\nvários números, um de cada vez, para cada\nvalor digitado pelo usuário. O programa\nserá interrompido quando o número\nsolicitado for negativo.")
        )
    );

    exercício_informações.mostrar_informações();

    /* Corpo do Exercício */
    loop {
        let número_digitado = obter_um_número(
            &exercício_informações
        );
    
        if número_digitado < 0 {
            break;
        } else {
            mostrar_tabuada(
                número_digitado
            );
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn mostrar_tabuada(
    número_digitado: i32
) {
    sleep(Duration::from_millis(1000));
    
    println!(
        "Tabuada do {}\n",
        número_digitado
    );

    for count in 1..11 {
        println!(
            "{} x {:>2} = {}",
            número_digitado,
            count,
            número_digitado * count
        );
    }

    println!();
}

fn obter_um_número(
    exercício_informações: &ExercícioInformações
) -> i32 {
    loop {
        println!(
            "[número negativo encerra]\nQual tabuada você quer ver?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas digite número!\n"
                        );
                    }
                }
            } 
            Err(_) => (),
        }
    }
}
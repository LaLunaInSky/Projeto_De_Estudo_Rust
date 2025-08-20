use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 061:
 Um programa que mostra a tabuada de\nvários números, um de cada vez, para cada\nvalor digitado pelo usuário. O programa\nserá interrompido quando o número\nsolicitado for negativo.
"
    )
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!(
        "{}\n{}",
        cabeçalho_do_programa,
        descrição_do_exercício()
    );

    /* Corpo do Exercício */
    loop {
        let número_digitado = obter_um_número(
            &cabeçalho_do_programa
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
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
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
    cabeçalho_do_programa: &String
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
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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
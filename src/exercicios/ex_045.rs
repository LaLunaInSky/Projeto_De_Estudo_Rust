use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!(
        "Descrição do exercício 045:"
    );
    println!(
        " Um programa que calcula a soma entre todos\nos números ímpares que são múltiplos de\ntrês e que se encontram no intervalo de 1\naté 500."
    );
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!(
        "{}", cabeçalho_do_programa
    );

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício */
    let números_ímpares: Vec<u32> = obter_números_ímpares_até_quinhentos_até_quinhentos();

    somar_todos_os_números(
        &números_ímpares
    );

    println!(
        "{:?}", números_ímpares
    );

    /* Fim do Exercício */
    sleep(Duration::from_millis(6000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn somar_todos_os_números(
    números: &Vec<u32>
) {
    let mut soma_dos_números = 0;

    for número in números {
        soma_dos_números += número;
    }

    println!(
        "\nA soma de todos os números é: {}.\n",
        soma_dos_números
    );
}

fn obter_números_ímpares_até_quinhentos_até_quinhentos() -> Vec<u32> {
    let mut números_ímpares: Vec<u32> = vec![];

    for número in 1..501 {
        if número % 2 != 0 {
            números_ímpares.push(número);
        }
    }

    return números_ímpares;
}
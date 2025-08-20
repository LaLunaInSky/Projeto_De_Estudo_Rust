use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command,
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

// ex_068!
fn descrição_do_exercício() -> String {
    let descrição = formatar_descrição(" Um programa que joga par ou ímpar com o usuário. O jogo só será interrompido quando o usuário PERDER, mostrando o total de vitórias consecutivas que o mesmo conquistou no final do jogo.".to_string());

    format!(
        "Descrição do exercício 062:
{}
",
        descrição
    )
}

fn formatar_descrição(
    descrição: String
) -> String {
    let mut descrição = descrição;
    let mut quantidade = 1;

    for (index, char) in descrição.chars().enumerate() {
        println!(
            "{} - {} - {}",
            quantidade,
            char,
            index
        );

        
        
        if quantidade < 42 {
            quantidade += 1; 
        } else {
            quantidade = 1;
        }
    }

    descrição
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


    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}
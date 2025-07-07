use std::{
    io,
    thread,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 013:");
    println!(
        " Um programa que pergunta a quantidade de\nKm percorridos por um carro alugado e a\nquantidade de dias pelos quais ele foi\nalugado. 
O programa irá calcular o preço à ser pago,\nsabendo que o carro custa R$60.00 por dia\ne R$0.15 por Km rodado."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */


    /* Fim do Exercício */
    // thread::sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // thread::sleep(Duration::from_millis(3000));

    // clean_terminal_linux(); 
}
use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

use rand::seq::index;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 023:");
    println!(
        " Um programa que lê uma frase pelo teclado\nmostra:

- Quantas vezes aparece a letra \"A\";
- Em que posição o \"A\" aparece pela\nprimeira vez;
- Em que posição o \"A\" aparece pela última\nvez."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let frase_digitada = obter_uma_frase(&cabeçalho_do_programa);

    analisar_a_frase(&frase_digitada);

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_a_frase(frase_digitada: &String) {
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

    sleep(Duration::from_millis(1500));

    println!("Analisando a frase...\n");

    sleep(Duration::from_millis(3000));

    println!(
        "Existe {} \"A\" na frase.",
        quantidade_de_as
    );

    sleep(Duration::from_millis(900));

    if quantidade_de_as > 0 {
        println!(
            "O primeiro \"A\" aparece na posição {}.",
            posição_do_primeiro_a
        );

        sleep(Duration::from_millis(900));

        println!(
            "O último \"A\" aparece na posição {}.",
            if quantidade_de_as == 1 {posição_do_primeiro_a} else {posição_do_ultimo_a}
        );

        sleep(Duration::from_millis(3000));
    } else {
        println!(
            "Nenhuma posição do \"A\" na frase.",
        );

        sleep(Duration::from_millis(3000));
    }

    
}

fn obter_uma_frase(cabeçalho_do_programa: &String) -> String {
    loop {
        println!("Digite uma frase:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                clean_terminal_linux();

                println!("{}", cabeçalho_do_programa);

                descrição_do_exercício();

                println!(
                    "\nA frase \"{}\",\nfoi adicionada com sucesso!\n",
                    input.trim()
                );

                return (input.trim()).to_string();
            }
            Err(_) => println!("Erro!"),
        }
    }
}
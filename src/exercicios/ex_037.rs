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
    println!("Descrição do exercício 037:");
    println!(
        " Um programa que lê duas notas de um aluno\ne calcula sua média, mostrando uma\nmensagem no final, de acordo com a média\natingida:
- Média abaixo de 5.0: Reprovado
- Média entre 5.0 e 6.9: Recuperação
- Média 7.0 ou superior: Aprovado"
    );
}

struct Aluno {
    nota_1: f32,
    nota_2: f32,
    média: f32
}

impl Aluno {
    fn new(nota_1: f32, nota_2: f32) -> Self {
        Self {
            nota_1,
            nota_2,
            média: (nota_1 + nota_2 / 2.0)
        }
    }
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();

    println!();

    /* Corpo do Exercício - fn main */
    let notas_do_aluno_01 = Aluno::new(
        obter_a_nota(&cabeçalho_do_programa, 1),
        obter_a_nota(&cabeçalho_do_programa, 2)
    );

    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_a_nota(cabeçalho_do_programa: &String, indice_da_nota: u8) -> f32 {
    loop {
        println!("Digite a {indice_da_nota}ª Nota:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {

            }
            Err(_) => println!("Erro!"),
        }
    }
}
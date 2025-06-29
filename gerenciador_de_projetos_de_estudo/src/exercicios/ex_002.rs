use std::process::Command;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercícios() {
    println!("Descrição do exercício 002:");
    println!(
        " Um programa que lê a entrado do teclado\ne mostra no terminal o seu tipo primitivo,\ne outras as informação possíveis sobre o\nque foi digitado.

Exemplo:

* Seu Tipo Primitivo
* Se possui espaços
* Se é apenas um número
* Se é alfabético
* Se é alfanumérico
* Se está em maiúscula
* Se está em minúscula
* Se está capitalizada"
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercícios();

    println!();
}
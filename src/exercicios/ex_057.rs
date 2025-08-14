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
        "Descrição do exercício 057:
 Melhore o EX_048, perguntando para o\nusuário se ele quer mostrar mais alguns\ntermos. O programa encerra quando ele\ndisser que quer mostrar 0 termos.
"
    )
}

#[derive(Debug)]
struct PA {
    número: u32,
    razão: u32,
    próximo_termo: u32,
}

impl PA {
    fn new(
        número: u32,
        razão: u32
    ) -> Self {
        Self {
            número,
            razão,
            próximo_termo: número + razão
        }
    }

    fn buscar_o_próximo_termo(
        &mut self
    ) {
        self.próximo_termo += self.razão;
    }
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
    let pa = PA::new(
        obter_número_inteiro(
            &cabeçalho_do_programa, 
            "pa"
        ),
        obter_número_inteiro(
            &cabeçalho_do_programa,
            "razão"
        )
    );

    println!(
        "{:?}\n",
        pa
    );

    /* Fim do Exercício */
    // sleep(Duration::from_millis(3000));

    // println!(
    //     "\nVoltando ao menu de exercícios...\n"
    // );

    // sleep(Duration::from_millis(3000));

    // clean_terminal_linux();
}

fn obter_número_inteiro(
    cabeçalho_do_programa: &String,
    nome_da_chamada: &str
) -> u32 {
    loop {
        println!(
            "Digite o número da {}:",
            nome_da_chamada.to_uppercase()
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        if número == 0 && nome_da_chamada == "razão" {
                            let número: u32 = 1;

                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "A {} de {},\nfoi adicionada com sucesso!\n",
                                nome_da_chamada.to_uppercase(),
                                número
                            );

                            return número;
                            
                        } else {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "A {} de {},\nfoi adicionada com sucesso!\n",
                                nome_da_chamada.to_uppercase(),
                                número
                            );

                            return número;
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "Erro! Digite número inteiros!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
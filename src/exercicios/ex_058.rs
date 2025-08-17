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
        "Descrição do exercício 058:
 Um programa que lê um número inteiro x e\nmostra no terminal os x primeiros\nelementos da Sequência de Fibonacci.

 Exemplo:
0 -> 1 -> 1 -> 2 -> 3 -> 5 -> 8
"
    )
}

struct SequênciaDeFibonacci {
    penúltimo_elemento: u32,
    último_elemento: u32,
    próximo_elemento: u32,
}

impl SequênciaDeFibonacci{
    fn new() -> Self {
        let penúltimo_elemento = 0;
        let último_elemento = 1;
        let próximo_elemento = penúltimo_elemento + último_elemento;

        Self {
            penúltimo_elemento,
            último_elemento,
            próximo_elemento
        }
    }

    fn obter_o_próximo_elemento(
        &mut self
    ) {
        self.penúltimo_elemento = self.último_elemento;
        self.último_elemento = self.próximo_elemento;
        self.próximo_elemento = self.penúltimo_elemento + self.último_elemento;
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
    let mut sequência_de_fibonacci = SequênciaDeFibonacci::new();

    loop {
        let quantidade_de_termos: u32 = obter_um_número_inteiro(
            &cabeçalho_do_programa
        );

        if quantidade_de_termos == 0 {
            break;
        } else {
            mostrar_x_termos_da_sequência_de_fibonacci(
                &mut sequência_de_fibonacci,
                quantidade_de_termos
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

fn mostrar_x_termos_da_sequência_de_fibonacci(
    sequência_de_fibonacci: &mut SequênciaDeFibonacci,
    quantidade_de_termos: u32
) {
    
    sleep(Duration::from_millis(1500));

    for termo in 0..quantidade_de_termos {
        print!(
            "{}",
            sequência_de_fibonacci.penúltimo_elemento
        );

        if quantidade_de_termos > 1 {
            if termo != (quantidade_de_termos - 1) {
                print!(
                    " -> "
                );
            }
        }

        sequência_de_fibonacci.obter_o_próximo_elemento();
    }

    println!("\n");

    sleep(Duration::from_millis(1500));
}

fn obter_um_número_inteiro(
    cabeçalho_do_programa: &String
) -> u32 {
    loop {
        println!(
            "[0 o programa encerra]\nQuantos termos você quer ver?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(quantidade_de_termos) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "A quantidade de {} termos,\nfoi adicionada com sucesso!\n",
                            quantidade_de_termos
                        );

                        return quantidade_de_termos;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "Erro! Apenas digite números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
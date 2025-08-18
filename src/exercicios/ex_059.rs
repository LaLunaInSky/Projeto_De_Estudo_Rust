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
        "Descrição do exercício 059:
 Um programa que lê vários números\ninteiros digitados. O programa só vai\nparar quando o usuário digitar o valor\n999, que é a condição de parada. No\nfinal, mostra quantos números foram\ndigitados e qual foi a soma entre eles\n(desconsiderando a flag.)
"
    )
}

#[derive(Debug)]
struct Números {
    números: Vec<u32>,
    soma_dos_números: u32
}

impl Números {
    fn new() -> Self {
        Self {
            números: vec![],
            soma_dos_números: 0,
        }
    }

    fn adicionar_número(
        &mut self,
        número: u32
    ) {
        self.números.push(
            número
        );
    }

    fn somar_números(
        &mut self
    ) {
        let mut soma_dos_números: u32 = 0;
        let números = self.números.clone();

        for número in números {
            soma_dos_números += número;
        }

        self.soma_dos_números = soma_dos_números;
    }

    fn retornar_quantidade_de_números_armazenados(
        &self
    ) -> u32 {
        return self.números.len() as u32;
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
    let mut números_digitados = Números::new();

    loop {
        let número_digitado = obter_um_número_inteiro(
            &cabeçalho_do_programa
        );

        if número_digitado != 999 {
            números_digitados.adicionar_número(
                número_digitado
            );
        } else {
            números_digitados.somar_números();

            break;
        }
    }

    analisar_os_números(
        números_digitados
    );

    /* Fim do Exercício */
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_os_números(
    números_digitados: Números
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números digitados...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os números:\n{:?}\n\nNo total foram digitados {} números.\n\nA soma deles é igual à {}.\n",
        números_digitados.números,
        números_digitados.retornar_quantidade_de_números_armazenados(),
        números_digitados.soma_dos_números
    );

    sleep(Duration::from_millis(2500));
}

fn obter_um_número_inteiro(
    cabeçalho_do_programa: &String
) -> u32 {
    loop {
        println!(
            "[999 encerra o programa]\nDigite um número inteiro:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
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
                            "Erro! Digite apenas número!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
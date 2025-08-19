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
        "Descrição do exercício 060:
 Um programa que lê vários números\ninteiros digitados. No final da execução,\nmostra a média entre todos os valores e\nqual foi o maior e o menor valor\nregistrado. O programa deve perguntar ao\nusuário se ele quer ou não continuar a\ndigitar valores.
"
    )
}

#[derive(Debug)]
struct Números {
    lista_de_números: Vec<u32>,
    média_dos_números: u32,
    número_maior: u32,
    número_menor: u32
}

impl Números {
    fn new() -> Self {
        Self {
            lista_de_números: vec![],
            média_dos_números: 0,
            número_maior: 0,
            número_menor: 0
        }
    }

    fn adicionar_um_novo_número(
        &mut self,
        número: u32
    ) {
        if self.lista_de_números.len() == 0 {
            self.número_maior = número;

            self.número_menor = número;
            
        } else {
            if self.número_menor > número {
                self.número_menor = número
            }
            
            if self.número_maior < número {
                self.número_maior = número
            }
        }

        self.lista_de_números.push(número);
    }

    fn calcular_a_média_dos_números(
        &mut self
    ) {
        let mut soma_dos_números: u32 = 0;

        let lista_de_números  = self.lista_de_números.clone();
        
        let total_de_números: u32 = lista_de_números.len() as u32;

        for número in lista_de_números {
            soma_dos_números += número;
        }

        self.média_dos_números = soma_dos_números / total_de_números;
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!(
            "{}\n{}",
            cabeçalho_do_programa,
            descrição_do_exercício()
        );

        /* Corpo do Exercício */
        let mut números_digitados = Números::new();

        loop {
            números_digitados.adicionar_um_novo_número(
                obter_um_número_inteiro(
                    &cabeçalho_do_programa
                )
            );

            let resposta_sobre_continuar = perguntar_se_quer_digitar_um_novo_número(
                &cabeçalho_do_programa
            );

            if !resposta_sobre_continuar {
                números_digitados.calcular_a_média_dos_números();

                break;
            }
        }

        analisar_os_números_digitados(
            números_digitados
        );

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_valores(
            &cabeçalho_do_programa
        );

        if !resposta_sobre_continuar {
            break;
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

fn perguntar_se_quer_adicionar_novos_valores(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar novos valores? [S/N]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                let resposta_da_pergunta = resposta_da_pergunta.as_str();

                match resposta_da_pergunta {
                    "s" => {
                        clean_terminal_linux();

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "Erro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn analisar_os_números_digitados(
    números_digitados: Números
) {
    sleep(Duration::from_millis(1000));

    println!(
        "\nAnalisando os números...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os números:\n{:?}\n\nA média é {}.\nO maior número é o {},\ne o menor número é o {}.\n",
        números_digitados.lista_de_números,
        números_digitados.média_dos_números,
        números_digitados.número_maior,
        números_digitados.número_menor
    );

    sleep(Duration::from_millis(1500));
}

fn obter_um_número_inteiro(
    cabeçalho_do_programa: &String,
) -> u32 {
    loop {
        println!(
            "Digite um número inteiro:"
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
                            "Erro! Apenas é aceito números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn perguntar_se_quer_digitar_um_novo_número(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar mais um número? [S/N]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                let resposta_da_pergunta = resposta_da_pergunta.as_str();

                match resposta_da_pergunta {
                    "s" => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "Erro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
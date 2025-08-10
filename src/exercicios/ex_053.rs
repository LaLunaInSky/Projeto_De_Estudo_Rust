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
        "Descrição do exercício 052:
 Um programa que lê o nome, idade e\ngênero de 4 pessoas. No final do\nprograma, motra:

- A média de idade do grupo.
- Qual é o nome do homem mais velho.
- Quantas mulheres têm menos de 20 anos.
"
    )
}

#[derive(Debug)]
struct Pessoa {
    nome: String,
    idade: u8,
    gênero: char,
}

impl Pessoa {
    fn new(
        index_da_chamada: u8,
        cabeçalho_do_programa: &String
    ) -> Self {
        fn obter_o_nome(
            index_da_chamada: &u8,
            cabeçalho_do_programa: &String
        ) -> String {
            loop {
                println!(
                    "Qual o nome da {}ª Pessoa:",
                    index_da_chamada
                );

                let mut input = String::new();

                match stdin().read_line(
                    &mut input
                ) {
                    Ok(_) => {
                        let nome = (input.trim()).to_string();

                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        println!(
                            "O nome {},\nfoi adicionado com sucesso!\n",
                            nome
                        );
                        
                        return nome;
                    } 
                    Err(_) => (),
                }
            }
        }

        fn obter_a_idade(
            index_da_chamada: &u8,
            cabeçalho_do_programa: &String
        ) -> u8 {
            loop {
                println!(
                    "Digite a idade da {}ª Pessoa:",
                    index_da_chamada
                );

                let mut input = String::new();

                match stdin().read_line(
                    &mut input
                ) {
                    Ok(_) => {
                        match input.trim().parse::<u8>() {
                            Ok(idade) => {
                                if idade <= 100 {
                                    clean_terminal_linux();

                                    println!(
                                        "{}\n{}",
                                        cabeçalho_do_programa,
                                        descrição_do_exercício()
                                    );

                                    println!(
                                        "A idade de {} anos,\nfoi adicionado com sucesso!\n",
                                        idade
                                    );

                                    return idade;
                                } else {
                                    clean_terminal_linux();

                                    println!(
                                        "{}\n{}",
                                        cabeçalho_do_programa,
                                        descrição_do_exercício()
                                    );

                                    println!(
                                        "Erro! Apenas idade até 100!\n"
                                    );
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
                                    "Erro! Digite apenas número!\n"
                                );
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
        }

        fn obter_o_gênero(
            index_da_chamada: &u8,
            cabeçalho_do_programa: &String
        ) -> char {
            loop {
                println!(
                    "Digite o gênero: [F/M]"
                );

                let mut input = String::new();

                match stdin().read_line(
                    &mut input
                ) {
                    Ok(_) => {
                        let resposta = input.trim().to_lowercase();

                        let resposta = resposta.as_str();

                        match resposta {
                            "f" | "m" => {
                                clean_terminal_linux();

                                println!(
                                    "{}\n{}",
                                    cabeçalho_do_programa,
                                    descrição_do_exercício()
                                );

                                let gênero: char = resposta.parse().unwrap();

                                println!(
                                    "O Gênero {},\nfoi adicionado com sucesso!\n",
                                    gênero
                                );

                                return gênero;
                            }
                            _ => {
                                clean_terminal_linux();

                                println!(
                                    "{}\n{}",
                                    cabeçalho_do_programa,
                                    descrição_do_exercício()
                                );

                                println!(
                                    "Erro! Apenas é aceito F ou M!\n"
                                );
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
        }

        let nome: String = obter_o_nome(
            &index_da_chamada, 
            &cabeçalho_do_programa
        );

        let idade: u8 = obter_a_idade(
            &index_da_chamada, 
            &cabeçalho_do_programa
        );

        let gênero: char = obter_o_gênero(
            &index_da_chamada, 
            &cabeçalho_do_programa
        );
        
        Self {
            nome,
            idade,
            gênero
        }
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
        let mut pessoas: Vec<Pessoa> = vec![];

        for quantidade in 1..5 {
            pessoas.push(
                Pessoa::new(
                    quantidade,
                    &cabeçalho_do_programa
                )
            );
        }

        println!(
            "{:?}\n",
            pessoas
        );

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_dados(
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

fn perguntar_se_quer_adicionar_novos_dados(
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
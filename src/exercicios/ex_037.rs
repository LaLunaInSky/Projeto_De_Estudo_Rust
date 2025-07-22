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
    média: f32,
    situação_final: String
}

impl Aluno {
    fn new(nota_1: f32, nota_2: f32) -> Self {
        Self {
            nota_1,
            nota_2,
            média: (nota_1 + nota_2) / 2.0,
            situação_final: String::from(if (nota_1 + nota_2) / 2.0 > 7.0 {"APROVADO"} else if (nota_1 + nota_2) / 2.0 < 5.0 {"REPROVADO"} else {"RECUPERAÇÃO"}) 
        }
    }
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let notas_do_aluno = Aluno::new(
            obter_a_nota(&cabeçalho_do_programa, 1),
            obter_a_nota(&cabeçalho_do_programa, 2)
        );

        mostrando_a_média(&notas_do_aluno);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novas_notas(&cabeçalho_do_programa);

        if resposta_sobre_continuar == false {
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

fn perguntar_se_quer_adicionar_novas_notas(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer adicionar novas notas? [S/N]");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
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

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nErro! Apenas é aceito S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn mostrando_a_média(dados_aluno: &Aluno) {
    sleep(Duration::from_millis(1000));

    println!(
        "Calculando a média das notas {:.1} e {:.1}...\n",
        dados_aluno.nota_1, dados_aluno.nota_2
    );

    sleep(Duration::from_millis(3000));

    println!(
        "A média do aluno é de {:.1}!\nO aluno está {}!\n",
        dados_aluno.média,
        if dados_aluno.média > 5.0 || dados_aluno.média < 7.0 {
            format!(
                "em {}",
                dados_aluno.situação_final
            )
        } else {
            format!(
                "{}",
                dados_aluno.situação_final
            )
        }
    );
}

fn obter_a_nota(cabeçalho_do_programa: &String, indice_da_nota: u8) -> f32 {
    loop {
        println!("Digite a {indice_da_nota}ª Nota:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(nota) => {
                        match nota {
                            0.0..=10.0 => {
                                let nota_formatada = format!(
                                    "{:.1}",
                                    nota
                                );

                                match nota_formatada.parse::<f32>() {
                                    Ok(nota_final) => {
                                        clean_terminal_linux();

                                        println!("{}", cabeçalho_do_programa);

                                        descrição_do_exercício();

                                        println!(
                                            "\nA nota {:.1},\nfoi adicionada com sucesso!\n",
                                            nota_final
                                        );

                                        return nota_final
                                    }
                                    Err(_) => println!("Erro!"),
                                }
                            }
                            _ => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!("\nErro! Apenas é aceito notas de 0.0 à 10.0!\n");
                            }
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();
                    
                        println!(
                            "\nErro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}
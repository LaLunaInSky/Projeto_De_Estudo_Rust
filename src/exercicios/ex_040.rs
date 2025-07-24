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
    println!("Descrição do exercício 040:");
    println!(
        " Desenvolva uma lógica que lê o peso e a\naltura de uma pessoa, calcula o seu IMC e\nmostra seu status corporal, de acordo com\na tabela abaixo:

- Abaixo de  18.5: Abaixo do peso
- Entre 18.5 e 25: Peso ideal
- Entre 25 e 30: Sobrepeso
- Entre 30 e 40: Obesidade
- Acima de 40: Obesidade mórbida"
    );
}

#[derive(Debug)]
struct Pessoa {
    peso: f32,
    altura: f32,
    imc: f32,
    status_corporal: String, 
}

impl Pessoa {
    fn new(peso: f32, altura: f32) -> Self {
        let imc = arrendondar_um_número_real(peso / (altura * altura), 1);

        let status_corporal: String = if imc > 40.0 {
            String::from("Obesidade Mórbida")
        } else if imc > 30.0 {
            String::from("Obesidade")
        } else if imc > 25.0 {
            String::from("Sobrepeso")
        } else if imc > 18.5 {
            String::from("Peso Ideal")
        } else {
            String::from("Abaixo do peso")
        };
        
        Self {
            peso,
            altura,
            imc,
            status_corporal
        }
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    loop {
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let dados_da_pessoa = Pessoa::new(
            obter_um_número_real(cabeçalho_do_programa, "peso"),
            obter_um_número_real(cabeçalho_do_programa, "altura")
        );

        analisar_o_imc_e_o_status_da_pessoa(&dados_da_pessoa);

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
        "\nVoltando ao menu de exercícios,,,\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn analisar_o_imc_e_o_status_da_pessoa(pessoa: &Pessoa) {
    sleep(Duration::from_millis(1000));

    println!("Analisando o IMC da pessoa...\n");

    sleep(Duration::from_millis(3000));

    println!(
        "A pessoa com o peso de {:.2}kg,\ne a altura de {:.2}m,\nestá com o IMC de {}.\n",
        pessoa.peso, pessoa.altura, pessoa.imc
    );

    sleep(Duration::from_millis(1000));

    println!(
        "Status corporal é de {}!\n",
        pessoa.status_corporal
    );    

    sleep(Duration::from_millis(1200));
}

fn perguntar_se_quer_adicionar_novos_dados(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!(
            "Quer adicionar novos valores? [S/N]"
        );

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

                        println!("\nErro! Apenas é aceito S [sim] ou N [não]!\n");
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn obter_um_número_real(
    cabeçalho_do_programa: &String, 
    tipo_do_input: &str
) -> f32 {
    loop {
        println!(
            "Digite {}:",
            if tipo_do_input == "peso" {
                format!(
                    "o {}", tipo_do_input
                )
            } else {
                format!(
                    "a {}", tipo_do_input
                )
            }
        );

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número) => {
                        if tipo_do_input == "altura" {
                            if número < 2.25 {
                                let número_final = arrendondar_um_número_real(número, 2);

                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nA {} de {:.2}m,\nfoi adicionada com sucesso!\n", tipo_do_input,
                                    número_final

                                );

                                return número_final;
                            } else {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nErro! Apenas é aceito altura de até 2.25m!\n"
                                );
                            } 
                        } else {
                            let número_final = arrendondar_um_número_real(número, 2);

                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!(
                                "\nO {} de {:.2}Kg,\nfoi adicionado com sucesso!\n", tipo_do_input,
                                número_final

                            );

                            return número_final;
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite apenas números!\n")
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn arrendondar_um_número_real(número: f32, quantidade_depois_do_ponto: u8) -> f32 {
    let mut número_formatado = String::new();
    
    match quantidade_depois_do_ponto {
        1 => {
            número_formatado = format!(
                "{:.1}",
                número
            );
        }
        2 => {
            número_formatado = format!(
                "{:.2}",
                número
            );
        }
        _ => (),
    }

    match número_formatado.parse::<f32>() {
        Ok(número_final) => número_final,
        Err(_) => 0.0,
    }
}
use std::{
    io,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 028:");
    println!(
        " Um programa que pergunta a distância de\numa viagem em Km. Calcula o preço da\npassagem, cobrando  R$0,50 por Km para\nviagens de até 200Km e R$0,45 para\nviagens mais longas."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);
    
        descrição_do_exercício();
    
        println!();
    
        /* Corpo do Exercício - fn main */
        let distância_da_viagem = obter_a_distância_da_viagem(&cabeçalho_do_programa);
    
        calcular_o_preço_da_viagem(&distância_da_viagem);
    
        let resposta_sobre_continuar_o_exercício = perguntar_se_quer_recomeçar_o_exercício(&cabeçalho_do_programa);

        if resposta_sobre_continuar_o_exercício == false {
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

fn perguntar_se_quer_recomeçar_o_exercício(cabeçalho_do_programa: &String) -> bool {
    loop {
        println!("Quer calcular o preço de outra viagem?\n[S/N]");

        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let resposta_da_pergunta = input.trim().to_lowercase();

                if resposta_da_pergunta == "s" || resposta_da_pergunta == "n" {
                    if resposta_da_pergunta == "s" {
                        clean_terminal_linux();

                        return true;
                    } else {
                        return false;
                    }
                } else {
                    clean_terminal_linux();

                    println!("{}", cabeçalho_do_programa);

                    descrição_do_exercício();

                    println!(
                        "\nErro! Digite apenas S [Sim] ou N [Não]!\n"
                    );
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}

fn calcular_o_preço_da_viagem(distância_da_viagem: &f32) {
    let mut valor_da_viagem: f32 = 0.0;

    if *distância_da_viagem <= 200.0 {
        valor_da_viagem = *distância_da_viagem * 0.50;
    } else {
        valor_da_viagem = *distância_da_viagem * 0.45;
    }

    sleep(Duration::from_millis(1500));

    println!("Calculando o preço da viagem...\n");

    sleep(Duration::from_millis(2500));

    println!(
        "O preço final é de R${:.2}.\n",
        valor_da_viagem
    );

    sleep(Duration::from_millis(1500));
}

fn obter_a_distância_da_viagem(cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Digite a distância da viagem em kms:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {

            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(distância) => {

                        let distância_formatada = format!(
                            "{:.2}", distância
                        );

                        match distância_formatada.parse::<f32>() {
                            Ok(distância_final) => {
                                clean_terminal_linux();

                                println!("{}", cabeçalho_do_programa);

                                descrição_do_exercício();

                                println!(
                                    "\nA distância de {:.2}km,\nfoi adicionada com sucesso!\n",
                                    distância_final
                                );

                                return distância_final;
                            }
                            Err(_) => println!("Erro!"),
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!("\nErro! Digite um valor válido!\n");
                    }
                }
            }
            Err(_) => println!("Erro!"),
        }
    }
}
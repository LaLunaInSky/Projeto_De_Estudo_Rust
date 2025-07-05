use std::{
    io,
    process::Command,
    thread,
    time::Duration
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() {
    println!("Descrição do exercício 009:");
    println!(
        " Um programa que lê a largura e a altura\n de uma parade em metros, calcula a sua\nárea e a quantidade de tinta necessária\npara pintá-la, sabendo que cada litro de\ntinta pinta uma área de 2m²."
    );
}

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    println!("{}", cabeçalho_do_programa);

    descrição_do_exercício();
    
    println!();

    /* Corpo do exercício main */
    let mut tamanho_da_parede: Vec<f32> = vec![];

    for tamanho in 1..3 {
        if tamanho == 1 {
            tamanho_da_parede.push(obter_o_tamanho_de_uma_parede_em_metros("altura", &cabeçalho_do_programa));
        } else {
            tamanho_da_parede.push(obter_o_tamanho_de_uma_parede_em_metros("largura", &cabeçalho_do_programa));
        }
    }

    let area_da_parede = cacular_a_area_da_parede(&tamanho_da_parede[0], &tamanho_da_parede[1]);

    println!(
        "Altura: {:.1}m\nLargura: {:.1}m\n",
        tamanho_da_parede[0],
        tamanho_da_parede[1]
    );

    println!(
        "A área da parade é de {}m²\n",
        area_da_parede
    );

    calcular_quantidade_de_tinta(&area_da_parede);

    /* Fim do Exercício */
    thread::sleep(Duration::from_millis(3000));

    println!("\nVoltando para o menu de exercícios...\n");

    thread::sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn calcular_quantidade_de_tinta(area_da_parade: &f32) {
    let um_litro_de_tinta_pinta_x_area = 2.0;

    let quantidade_de_litros_de_tinta_que_precisará = area_da_parade / um_litro_de_tinta_pinta_x_area;

    println!(
        "Logo precisará de {} litros de tinta!",
        quantidade_de_litros_de_tinta_que_precisará
    );
}

fn cacular_a_area_da_parede(altura_da_parede: &f32, largura_da_parede: &f32) -> f32 {
    let area = altura_da_parede * largura_da_parede;

    let area_formatada = format!(
        "{:.1}", area
    );

    match area_formatada.parse::<f32>() {
        Ok(number) => return number,
        Err(_) => println!("Erro!"),
    }

    return 0.0;
}

fn obter_o_tamanho_de_uma_parede_em_metros(comprimento: &str, cabeçalho_do_programa: &String) -> f32 {
    loop {
        println!("Qual o {} da parede? ( em metros )", comprimento.to_uppercase());

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(number) => {
                        if number > 0.0 {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            let número_formatado = format!(
                                "{:.1}", number
                            );

                            match número_formatado.parse::<f32>() {
                                Ok(número) => {
                                    println!(
                                        "\nA {} de {}m foi adicionado com\nsucesso!\n", 
                                        comprimento.to_uppercase(),
                                        número
                                    );
            
                                    return número;
                                }
                                Err(_) => println!("Erro!"),
                            }
                        } else {
                            clean_terminal_linux();

                            println!("{}", cabeçalho_do_programa);

                            descrição_do_exercício();

                            println!("\nErro! Digite um valor maior que zero!\n");
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
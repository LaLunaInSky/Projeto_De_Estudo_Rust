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
    println!("Descrição do exercício 039:");
    println!(
        " Refaça o EX_032 dos triângulos\nacrescentando recurso de mostrar que tipo\nde triângulo será formado:
        
- Equilátero: todos os lados iguais
- Isósceles: dois lados iguais
- Escaleno: todos os lados diferentes"
    );
}

#[derive(Debug)]
struct Segmentos {
    lados: Vec<u32>,
    é_um_triangulo: bool,
    tipo_do_triângulo: String,
}

impl Segmentos {
    fn new(lados: Vec<u32>) -> Self {
        let é_um_triangulo = verificar_se_formar_um_triângulo(&lados);

        let mut tipo_do_triângulo = String::new();

        if é_um_triangulo {
            tipo_do_triângulo = verificar_o_tipo_do_triângulo(&lados);
        }

        Self {
            lados,
            é_um_triangulo,
            tipo_do_triângulo
        }
    }
}

fn verificar_o_tipo_do_triângulo(
    lados: &Vec<u32>
) -> String {
    if lados[0] == lados[1] && lados[1] == lados[2] && lados[0] == lados[2] {
        return String::from(
            "EQUILÁTERO"
        );
    } else if lados[0] == lados[1] || lados[0] == lados[2] || lados[1] == lados[2] {
        return String::from(
            "ISÓSCELES"
        );
    } else {
        return String::from(
            "ESCALENO"
        );
    }
}

fn verificar_se_formar_um_triângulo(
    lados: &Vec<u32>
) -> bool {
    if (lados[0] + lados[1]) > lados[2] && (lados[0] + lados[2]) > lados[1] && (lados[1] + lados[2]) > lados[0] {
        return true;
    } else {
        return false;
    }
} 

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    loop {
        /* Começo do Exercício */
        println!("{}", cabeçalho_do_programa);

        descrição_do_exercício();

        println!();

        /* Corpo do Exercício - fn main */
        let mut lados_digitados: Vec<u32> = vec![];

        for lado in 1..4 {
            lados_digitados.push(
                obter_o_lado_de_um_suposto_triângulo(
                    &cabeçalho_do_programa, 
                    lado
                )
            );
        }

        let suposto_triângulo = Segmentos::new(
            lados_digitados
        );

        analisar_os_segmentos(&suposto_triângulo);

        let resposta_sobre_continuar = perguntar_se_quer_adicionar_novos_segmentos(&cabeçalho_do_programa);

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

fn analisar_os_segmentos(segmentos: &Segmentos) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os segmentos...\n"
    );

    sleep(Duration::from_millis(3000));

    println!(
        "O segmentos {:?},\n{}formam um triângulo!",
        segmentos.lados,
        if !segmentos.é_um_triangulo {"NÃO "} else {""}
    );

    if segmentos.é_um_triangulo {
        println!(
            "\nEste é um triângulo {}!\n",
            segmentos.tipo_do_triângulo
        )
    } else {
        println!();
    }

    sleep(Duration::from_millis(1500));
}

fn perguntar_se_quer_adicionar_novos_segmentos(
    cabeçalho_do_programa: &String
) -> bool {
    loop {
        println!("Quer adicionar novos segmentos? [S/N]");

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

fn obter_o_lado_de_um_suposto_triângulo(
    cabeçalho_do_programa: &String,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!("Digite o {index_da_chamada}º Segmento:");

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(segmento) => {
                        clean_terminal_linux();

                        println!("{}", cabeçalho_do_programa);

                        descrição_do_exercício();

                        println!(
                            "\nO segmento de {segmento},\nfoi adicionado com sucesso!\n"
                        );

                        return segmento;
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
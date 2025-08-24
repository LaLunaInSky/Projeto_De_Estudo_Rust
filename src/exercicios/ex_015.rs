use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::descrição_de_exercício,
    exercicio_informacoes::Exercício_Informações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício
};

mod tipos_de_cateto;
mod triangulos;

use tipos_de_cateto::TiposDeCateto;
use triangulos::Triângulo;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = Exercício_Informações::new(
        &cabeçalho_do_programa,
        descrição_de_exercício(
            String::from("015"),
            String::from("Um programa que lê o comprimento do\ncateto oposto e do cateto adjacente de um\ntriângulo retângulo, e depois calcula o\ncomprimento da hipotenusa.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let triângulo = Triângulo::new(
            obter_o_tamanho_do_cateto(
                TiposDeCateto::Oposto, 
                &exercício_informações
            ),
            obter_o_tamanho_do_cateto(
                TiposDeCateto::Adjacente, 
                &exercício_informações
            )
        );

        calcular_a_hipotenusa(
            &triângulo
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
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

    limpar_terminal();
}

fn calcular_a_hipotenusa(
    triângulo: &Triângulo
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Calculando a hipotenusa...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O triângulo retângulo com os catetos,
Oposto..........: {}
Adjacente.......: {}
A Hipotenusa é..: {:.2}
",
        triângulo.cateto_oposto,
        triângulo.cateto_adjacente,
        triângulo.hipotenusa
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_tamanho_do_cateto(
    tipo_do_cateto: TiposDeCateto, 
    exercício_informações: &Exercício_Informações
) -> u32 {
    loop {
        println!(
            "Digite o tamanho do cateto {}:", 
            if tipo_do_cateto == TiposDeCateto::Adjacente {"Adjacente"} else {"Oposto"}
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(cateto) => {
                        if cateto > 0 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "O cateto {} de {},\nfoi adicionado com sucesso!\n",
                                if tipo_do_cateto == TiposDeCateto::Adjacente {"Adjacente"} else {"Oposto"},
                                cateto
                            );

                            return cateto;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Digite um valor maior que zero!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um número inteiro!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
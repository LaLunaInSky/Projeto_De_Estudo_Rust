use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("027"),
            String::from("Um programa que lê um número inteiro e\nmostra na tela se ele é PAR ou ÍMPAR.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let número_informado = obter_um_número(
            &exercício_informações
        );

        verificar_se_é_par_ou_ímpar(
            &número_informado
        );
    
        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if resposta_sobre_continuar == false {
            break;
        }
    }
    
    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn verificar_se_é_par_ou_ímpar(
    número: &u32
) {
    let mut resultado = String::from("ÍMPAR");

    if número % 2 == 0 {
        resultado = String::from("PAR");
    }

    sleep(Duration::from_millis(1000));
    
    println!("Analisando...\n");

    sleep(Duration::from_millis(1500));

    println!(
        "O número é {}!\n",
        resultado
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número(
    exercício_informações: &ExercícioInformações
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
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O número {},\nfoi adicionado com sucesso!\n",
                            número
                        );

                        return número;
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
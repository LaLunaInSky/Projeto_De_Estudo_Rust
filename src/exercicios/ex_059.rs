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

mod numeros;

use numeros::Números;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("059"),
            String::from("Um programa que lê vários números\ninteiros digitados. O programa só vai\nparar quando o usuário digitar o valor\n999, que é a condição de parada. No\nfinal, mostra quantos números foram\ndigitados e qual foi a soma entre eles\n(desconsiderando a flag.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_digitados = Números::new();

        loop {
            let número_digitado = obter_um_número_inteiro(
                &exercício_informações
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
            &mut números_digitados
        );

        let resposta_sobre_continuar = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_sobre_continuar {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn analisar_os_números(
    números_digitados: &mut Números
) {
    números_digitados.somar_números();

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os números digitados...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "Os números:\n{:?}\n\nNo total foram digitados {} números.\n\nA soma deles é igual à {}.\n",
        números_digitados.get_números(),
        números_digitados.retornar_quantidade_de_números_armazenados(),
        números_digitados.get_soma_dos_números()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações
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
                            "Erro! Digite apenas número!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
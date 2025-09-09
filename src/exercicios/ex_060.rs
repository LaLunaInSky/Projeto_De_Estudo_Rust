use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
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
        buscar_descrição_do_exercício(
            String::from("060")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_digitados = Números::new();

        loop {
            números_digitados.adicionar_um_novo_número(
                obter_um_número_inteiro(
                    &exercício_informações
                )
            );

            let resposta_sobre_adicionar_mais_um_número = perguntar_se_quer_digitar_um_novo_número(
                &exercício_informações
            );

            if !resposta_sobre_adicionar_mais_um_número {
                números_digitados.calcular_a_média_dos_números();

                break;
            }
        }

        analisar_os_números_digitados(
            números_digitados
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

fn analisar_os_números_digitados(
    números_digitados: Números
) {
    sleep(Duration::from_millis(1000));

    println!(
        "\nAnalisando os números..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Os números:
{:?}

O Número maior é o {},
O Número menor é o {}.
A média dos números é {}.
",
        números_digitados.get_lista_de_números(),
        números_digitados.get_número_maior(),
        números_digitados.get_número_menor(),
        números_digitados.get_média_dos_números()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações,
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
    exercício_informações: &ExercícioInformações
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
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        return true;
                    }
                    "n" => return false,
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

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
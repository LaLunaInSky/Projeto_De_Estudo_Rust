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

mod numeros_unicos_em_ordem;

use numeros_unicos_em_ordem::NúmerosÚnicosEmOrdem;

// 79!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("072")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut números_únicos_em_ordem = NúmerosÚnicosEmOrdem::new();

        loop {
            números_únicos_em_ordem.adicionar_um_número_novo(
                obter_um_número_inteiro(
                    &exercício_informações
                )
            );
        
            let resposta_sobre_adicionar_mais_um_número = perguntar_se_quer_adicionar_um_novo_número(
                &exercício_informações
            );

            if !resposta_sobre_adicionar_mais_um_número {
                break;
            }
        }

        analisar_os_números_digitados(
            &números_únicos_em_ordem
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

fn perguntar_se_quer_adicionar_um_novo_número(
    exercício_informações: &ExercícioInformações
) -> bool {
    loop {
        println!(
            "Quer adicionar um novo número? [S/N]"
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
                            "Erro! Apenas S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}

fn obter_um_número_inteiro(
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
                            "O número {},\nfoi digitado com sucesso!\n",
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
            Err(_) => ()
        }
    }
}

fn analisar_os_números_digitados(
    números_únicos_em_ordem: &NúmerosÚnicosEmOrdem
) {
    sleep(Duration::from_millis(1000));

    println!(
        "\nAnalisando os números..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Os números únicos digitados em ordem são:
{:?}
",
        números_únicos_em_ordem.get_lista_de_números_únicos_em_ordem()
    );

    sleep(Duration::from_millis(1100));
}
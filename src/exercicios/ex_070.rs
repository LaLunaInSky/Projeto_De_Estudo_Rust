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

mod produtos;
mod arredondador_de_numeros_reais;

use produtos::Produtos;
use arredondador_de_numeros_reais::arredondador_de_número_real;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("070")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut produtos = Produtos::new();
        let mut quantidade_de_produtos_cadastrados: usize = 1;

        loop {
            produtos.adicionar_um_novo_produto(
                obter_o_nome_do_produto(
                    &exercício_informações,
                    quantidade_de_produtos_cadastrados
                ),
                obter_um_número_real(
                    &exercício_informações,
                    quantidade_de_produtos_cadastrados
                )
            );

            let resposta_sobre_adicionar_mais_um_produto = perguntar_sobre_adicionar_mais_um_produto(
                &exercício_informações
            );

            if !resposta_sobre_adicionar_mais_um_produto {
                break;
            } else {
                quantidade_de_produtos_cadastrados += 1;
            }
        }

        analisar_os_produtos(
            &produtos
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

fn obter_o_nome_do_produto(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: usize
) -> String {
    loop {
        println!(
            "Qual o nome do {}º Produto?",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                limpar_terminal();

                exercício_informações.mostrar_informações();

                println!(
                    "O produto {},\nfoi adicionado com sucesso!\n",
                    input.trim().to_lowercase()
                );

                return input.trim().to_lowercase();
            }
            Err(_) => ()
        }
    }
}

fn obter_um_número_real(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: usize
) -> f32 {
    loop {
        println!(
            "Qual o preço do {}º produto?",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(número_real) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O preço de R${:.2},\nfoi adicionado com sucesso!\n",
                            número_real
                        );

                        return arredondador_de_número_real(número_real);
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}

fn perguntar_sobre_adicionar_mais_um_produto(
    exercício_informações: &ExercícioInformações
) -> bool {
    loop {
        println!(
            "Quer adicionar mais um produto? [S/N]"
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
                            "Erro! Digite S [sim] ou N [não]!\n"
                        )
                    }
                }
            }
            Err(_) => ()
        }
    }
}

fn analisar_os_produtos(
    produtos: &Produtos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "\nAnalisando os produtos...\n"
    );

    sleep(Duration::from_millis(1500));

    for produto in produtos.get_lista_de_produtos() {
        println!(
            "{:.<18}R$ {:>6}",
            produto.0,
            format!(
                "{:.2}",
                produto.1
            ).replace(".", ",")
        )
    }

    println!();

    sleep(Duration::from_millis(1100));
}
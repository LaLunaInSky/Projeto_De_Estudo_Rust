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

mod produto;
mod arredondador_de_precos;
mod produtos;

use produto::Produto;
use arredondador_de_precos::arredondadar_preço;
use produtos::Produtos;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("064")
        )
    );

    loop {
        exercício_informações.mostrar_informações();
        
        /* Corpo do Exercício */
        let mut produtos = Produtos::new();
        
        loop {
            let quantidade_de_produtos: u8 = produtos.get_quantidade_de_produtos_adicionados() + 1;
            
            produtos.adicionar_um_novo_produto(
                Produto::new(
                obter_o_nome_do_produto(
                        &exercício_informações, 
                        quantidade_de_produtos
                    ),
                    obter_o_preço_do_produto(
                        &exercício_informações, 
                        quantidade_de_produtos
                    )
                )
            );

            let resposta_sobre_adicionar_outro_produto = perguntar_se_quer_adicionar_mais_um_produto(
                &exercício_informações
            );

            if !resposta_sobre_adicionar_outro_produto {
                break
            };
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

fn analisar_os_produtos(
    produtos: &Produtos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os produtos..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
O total de gasto é de R${:.2};

Possui {} produto{} que custa mais de\nR$1000.00;

O produto mais barato é {} com\no preço de R${:.2}.
",
        produtos.get_preço_total_dos_produtos(),
        produtos.get_quantidade_de_produtos_com_o_preço_superior_a_mil(),
        if produtos.get_quantidade_de_produtos_com_o_preço_superior_a_mil() != 1 {
            "s"
        } else {
            ""
        },
        produtos.get_dado_do_produto_com_menor_preço().get_nome(),
        produtos.get_dado_do_produto_com_menor_preço().get_preço()
    );

    sleep(Duration::from_millis(1100));
}

fn perguntar_se_quer_adicionar_mais_um_produto(
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
                            "Erro! Apenas digite S [sim] ou N [não]!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}

fn obter_o_preço_do_produto(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: u8
) -> f32 {
    loop {
        println!(
            "Qual o preço do {}º Produto?",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(preço) => {
                        let preço_arredondado = arredondadar_preço(
                            preço
                        );

                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O preço de R${},\nfoi adicionado com sucesso!\n",
                            preço_arredondado
                        );

                        return preço_arredondado;
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

fn obter_o_nome_do_produto(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: u8
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
                let resposta_do_input = input.trim().to_lowercase();

                limpar_terminal();

                exercício_informações.mostrar_informações();

                println!(
                    "O produto {},\nfoi adicionado com sucesso!\n",
                    resposta_do_input
                );

                return resposta_do_input;
            }
            Err(_) => ()
        }
    }
}
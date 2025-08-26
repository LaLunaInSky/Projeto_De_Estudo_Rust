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

mod nome_completo;

use nome_completo::NomeCompleto;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("024"),
            String::from("Um programa que lê o nome completo de\numa pessoa, mostrando em seguida o\nprimeiro e o último nome separadamente.

Exemplo: 
\"Ana Maria de Souza\"
- primeiro.: Ana
- último...: Souza")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let nome_completo = NomeCompleto::new(
            obter_nome_completo_da_pessoa(
                &exercício_informações
            )
        );

        analisar_o_nome_informado(
            &nome_completo
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

fn analisar_o_nome_informado(
    nome_completo: &NomeCompleto
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o nome..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
Primeiro..Nome: {}
Último....Nome: {}
",
        nome_completo.primeiro_nome.to_uppercase(),
        nome_completo.último_nome.to_uppercase()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_nome_completo_da_pessoa(
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite o nome completo de uma pessoa:"
        );

        let mut input = String::new();
    
        match stdin().read_line(&mut input) {
            Ok(_) => {
                limpar_terminal();

                exercício_informações.mostrar_informações();

                let nome = input.trim().to_lowercase();

                println!(
                    "O nome {},\nfoi adicionado com sucesso!\n",
                    nome
                );

                return nome;
            }
            Err(_) => (),
        }
    }
}
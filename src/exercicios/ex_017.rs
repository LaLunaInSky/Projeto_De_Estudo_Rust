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

mod alunos;

use alunos::Alunos;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("017"),
            String::from("Contexto:
 Um professor quer sortear um dos seus\nquatro alunos para apagar o quadro.

Exercício:
 Um programa que ajude ele, lendo o nome\ndos quatro alunos e retornando o nome do\nescolhido.")      
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut alunos = Alunos::new();

        for quantidade in 1..5 {
            alunos.adicionar_novo_aluno(
                obter_o_nome_de_um_aluno(
                    quantidade, 
                    &exercício_informações
                )
            );
        }

        sortear_um_nome(
            &alunos
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

fn sortear_um_nome(
    alunos: &Alunos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Sorteando o nome entre os alunos:"
    );

    let index_da_última_ocorrência = alunos.lista_de_nomes.len();

    for (
        index, 
        aluno
    ) in alunos.lista_de_nomes.iter().enumerate() {
        if index == (index_da_última_ocorrência - 1) {
            print!(
                "{}.\n\n",
                aluno
            )
        } else {
            print!(
                "{}, ",
                aluno
            );
        }
    }

    sleep(Duration::from_millis(1500));

    println!(
        "O Aluno sorteado é {}.\n",
        alunos.sortear_nome_de_um_aluno()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_nome_de_um_aluno(
    indice_da_chamada: u8,
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite o nome do {}º Aluno:",
            indice_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                limpar_terminal();

                exercício_informações.mostrar_informações();

                let nome_do_aluno = input.trim().to_lowercase();

                println!(
                    "O Aluno {},\nfoi adicionado com sucesso!\n",
                    nome_do_aluno
                );

                return nome_do_aluno;
            }
            Err(_) => (),
        }
    }
}
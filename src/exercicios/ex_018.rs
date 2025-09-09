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

mod alunos;

use alunos::Alunos;

pub fn rodar_o_exercício(cabeçalho_do_programa: &String) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("018")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut alunos = Alunos::new();

        for quantidade in 1..5 {
            alunos.adicionar_um_novo_aluno(
                obter_o_nome_do_aluno(
                    quantidade, 
                    &exercício_informações
                )
            );
        }

        sorteando_a_ordem_dos_nome(
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

fn sorteando_a_ordem_dos_nome(
    alunos: &Alunos
) {
    let lista_em_ordem = alunos.sortear_ordem_de_apresentação();

    sleep(Duration::from_millis(1000));

    println!(
        "Sorteando a ordem de apresentação...\n"
    );

    sleep(Duration::from_millis(1500));

    println!("A ordem de apresentação é:");

    sleep(Duration::from_millis(500));

    for (index, aluno) in lista_em_ordem.iter().enumerate() {
        println!(
            "{}º - {}",
            index + 1,
            aluno
        )
    }

    println!();

    sleep(Duration::from_millis(1100));
}

fn obter_o_nome_do_aluno(
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
                    "O Aluno {},\nfoi adiconado com sucesso!\n",
                    nome_do_aluno
                );

                return nome_do_aluno;
            }
            Err(_) => (),
        }
    }
}
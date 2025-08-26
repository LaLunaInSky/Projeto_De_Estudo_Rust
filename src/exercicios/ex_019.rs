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

mod nome;

use nome::Nome;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("019"),
            String::from("Um programa que lê o nome completo de\numa pessoa e mostra:
        
- O nome com todas as letras maiúsculas e\nminúsculas.
- Quantas letras o nome todo possui (sem\nconsiderar espaços).
- Quantas letras tem o primeiro nome.")    
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício - fn main */
        let nome = Nome::new(
            obter_o_nome_completo(
                &exercício_informações
            )
        );

        analisar_o_nome(
            &nome
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

fn analisar_o_nome(
    nome: &Nome
) { 
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando o nome...\n"
    );

    sleep(Duration::from_millis(1500));
    
    println!(
        "O seu nome em minúsculo é:\n{}\n",
        nome.obter_o_nome_em_minúscula()
    );

    sleep(Duration::from_millis(1000));

    println!(
        "O seu nome em maiúscula é:\n{}\n",
        nome.obter_o_nome_em_maiúscula()
    );

    sleep(Duration::from_millis(1000));

    println!(
        "O seu nome tem {} letras, sem contar os\nespaços!\n",
        nome.obter_o_total_de_letras_no_nome()
    );

    sleep(Duration::from_millis(1000));

    println!(
        "O seu primeiro nome tem {} letras.\n",
        nome.obter_o_total_de_letras_do_primeiro_nome()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_nome_completo(
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite o seu nome completo:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                limpar_terminal();

                exercício_informações.mostrar_informações();

                let nome_digitado = input.trim().to_lowercase();

                println!(
                    "O nome {},\nfoi adicionado com sucesso!\n",
                    nome_digitado
                );

                return nome_digitado;
            }
            Err(_) => (),
        }
    }
}
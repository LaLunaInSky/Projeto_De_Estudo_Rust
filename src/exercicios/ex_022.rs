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

mod nome;

use nome::Nome;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("022")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício - fn main */
        let nome = Nome::new(
            obter_o_nome_da_pessoa(
                &exercício_informações
            )
        );

        analisar_o_nome_digitado(
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

fn analisar_o_nome_digitado(
    nome: &Nome
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando do nome...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "No nome {},\n{}existe \"SILVA\" no nome!\n",
        nome.nome.to_uppercase(),
        if !nome.possui_silva {"Não "} else {""}
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_nome_da_pessoa(
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Digite o um nome e um sobrenome:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
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
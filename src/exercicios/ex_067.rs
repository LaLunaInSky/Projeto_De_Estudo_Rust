use std::{
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

mod tabela_do_brasileirao_2025;
mod orgnizacao_alfabeticamente;

use tabela_do_brasileirao_2025::TabelaDoBrasileirão2025;

// 73!
pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("067")
        )
    );
    
    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let tabela_do_brasileirão_2025 = TabelaDoBrasileirão2025::new();

        analisar_a_tabela(
            &tabela_do_brasileirão_2025
        );

        let resposta_da_pergunta = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if !resposta_da_pergunta {
            break;
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn analisar_a_tabela(
    tabela_do_brasileirão_2025: &TabelaDoBrasileirão2025
) {
    let analise_se_tem_o_time_chapecoense = if tabela_do_brasileirão_2025.get_posição_do_time_chapecoense() >= 0 {
        format!(
            "O time \"Chapecoense\" está na posição {}",
            tabela_do_brasileirão_2025.get_posição_do_time_chapecoense()
        )
    } else {
        format!(
            "O time \"Chapecoense\" não está em nenhuma posição!"
        )
    };

    sleep(Duration::from_millis(1000));

    println!(
        "Analisando a tabela..."
    );

    sleep(Duration::from_millis(1500));

    println!(
        "
 {}

1º - {}

2º - {}

3º - {}

4º - {}
",
        tabela_do_brasileirão_2025.get_tabela_em_ordem_de_colocação(),
        tabela_do_brasileirão_2025.get_cinco_primeiros_colocados(),
        tabela_do_brasileirão_2025.get_quatro_últimos_colocados(),
        tabela_do_brasileirão_2025.get_tabela_em_ordem_alfabética(),
        analise_se_tem_o_time_chapecoense
    );

    sleep(Duration::from_millis(1100));
}
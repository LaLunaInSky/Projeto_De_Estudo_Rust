// use std::{
//     io::stdin,
//     thread::sleep,
//     time::Duration
// };

use crate::recursos::{
    // limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    perguntar_se_quer_iniciar_novamento_o_exercicio::perguntar_se_quer_iniciar_novamente_o_exercício,
    final_do_exercicio::rodar_final_do_exercício
};

struct TabelaDoBrasileirão2025 {
    tabela_em_ordem_de_colocação: [String; 20],
    cinco_primeiros_colocados: [String; 5],
    quatro_últimos_colocados: [String; 4],
    tabela_em_ordem_alfabética: [String; 20],
    posição_do_time_chapecoense: i8
}

impl TabelaDoBrasileirão2025 {
    fn new() -> Self {
        let tabela_em_ordem_de_colocação: [String; 20] = [
            String::from("flamengo"), String::from("cruzeiro"), String::from("palmeiras"), String::from("mirassol"),  String::from("bahia"), String::from("botafogo"), String::from("são paulo"), String::from("bragantino"), String::from("corinthians"), String::from("fluminense"), String::from("ceará"), String::from("internacional"), String::from("atlético-MG"), String::from("grêmio"), String::from("vasco"), String::from("santos"), String::from("vitória"), String::from("juventude"), String::from("fortaleza"), String::from("sport")
        ];

        let mut cinco_primeiros_colocados: [String; 5] = [
            String::new(), String::new(), String::new(), String::new(), String::new()
        ];

        let mut quatro_últimos_colocados: [String; 4] = [
            String::new(), String::new(), String::new(), String::new()
        ];

        let mut tabela_em_ordem_alfabética: [String; 20] = [
            String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()
        ];

        let mut posição_do_time_chapecoense: i8 = -1;

        for (
            index, 
            time
        ) in tabela_em_ordem_de_colocação.iter().enumerate() {
            if index <= 4 {
                cinco_primeiros_colocados[index as usize] = time.clone();
            } else if index >= 16 {
                quatro_últimos_colocados[
                    (index - 16) as usize
                ] = time.clone()
            }

            if time == "chapecoense" {
                posição_do_time_chapecoense = (index + 1) as i8;
            }
        }
        
        Self {
            tabela_em_ordem_de_colocação,
            cinco_primeiros_colocados,
            quatro_últimos_colocados,
            tabela_em_ordem_alfabética,
            posição_do_time_chapecoense
        }
    }

    fn get_tabela_em_ordem_de_colocação(
        &self
    ) -> [String; 20] {
        return self.tabela_em_ordem_de_colocação.clone();
    }

    fn get_cinco_primeiros_colocados(
        &self
    ) -> [String; 5] {
        return self.cinco_primeiros_colocados.clone();
    }

    fn get_quatro_últimos_colocados(
        &self
    ) -> [String; 4] {
        return self.quatro_últimos_colocados.clone();
    }

    fn get_posição_do_time_chapecoense(
        &self
    ) -> i8 {
        return self.posição_do_time_chapecoense;
    }
}

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
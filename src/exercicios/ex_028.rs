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

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("028")
        )
    );

    loop {
        exercício_informações.mostrar_informações();
    
        /* Corpo do Exercício - fn main */
        let distância_da_viagem = obter_a_distância_da_viagem(
            &exercício_informações
        );
    
        calcular_o_preço_da_viagem(
            &distância_da_viagem
        );
    
        let resposta_sobre_continuar_o_exercício = perguntar_se_quer_iniciar_novamente_o_exercício(
            &exercício_informações
        );

        if resposta_sobre_continuar_o_exercício == false {
            break;
        }
    }    

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn calcular_o_preço_da_viagem(
    distância_da_viagem: &f32
) {
    let mut _valor_da_viagem: f32 = 0.0;

    if *distância_da_viagem <= 200.0 {
        _valor_da_viagem = *distância_da_viagem * 0.50;
    } else {
        _valor_da_viagem = *distância_da_viagem * 0.45;
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Calculando o preço da viagem...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O preço final é de R${:.2}.\n",
        _valor_da_viagem
    );

    sleep(Duration::from_millis(1100));
}

fn obter_a_distância_da_viagem(
    exercício_informações: &ExercícioInformações
) -> f32 {
    loop {
        println!(
            "Digite a distância da viagem em kms:"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<f32>() {
                    Ok(distância) => {

                        let distância_formatada = format!(
                            "{:.2}", distância
                        );

                        match distância_formatada.parse::<f32>() {
                            Ok(distância_final) => {
                                limpar_terminal();

                                exercício_informações.mostrar_informações();

                                println!(
                                    "A distância de {:.2}km,\nfoi adicionada com sucesso!\n",
                                    distância_final
                                );

                                return distância_final;
                            }
                            Err(_) => (),
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite um número real!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
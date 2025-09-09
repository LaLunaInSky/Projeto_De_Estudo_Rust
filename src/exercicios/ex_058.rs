use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::buscar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    final_do_exercicio::rodar_final_do_exercício
};

mod sequencia_de_fibonacci;

use sequencia_de_fibonacci::SequênciaDeFibonacci;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        buscar_descrição_do_exercício(
            String::from("058")
        )
    );
 
    exercício_informações.mostrar_informações();

    /* Corpo do Exercício */
    let mut sequência_de_fibonacci = SequênciaDeFibonacci::new();

    loop {
        let quantidade_de_termos: u32 = obter_um_número_inteiro(
            &exercício_informações
        );

        if quantidade_de_termos == 0 {
            break;
        } else {
            mostrar_x_termos_da_sequência_de_fibonacci(
                &mut sequência_de_fibonacci,
                quantidade_de_termos
            );
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn mostrar_x_termos_da_sequência_de_fibonacci(
    sequência_de_fibonacci: &mut SequênciaDeFibonacci,
    quantidade_de_termos: u32
) {
    
    sleep(Duration::from_millis(1500));

    for termo in 0..quantidade_de_termos {
        print!(
            "{}",
            sequência_de_fibonacci.get_penúltimo_elemento()
        );

        if quantidade_de_termos > 1 {
            if termo != (quantidade_de_termos - 1) {
                print!(
                    " -> "
                );
            }
        }

        sequência_de_fibonacci.obter_o_próximo_elemento();
    }

    println!("\n");

    sleep(Duration::from_millis(1100));
}

fn obter_um_número_inteiro(
    exercício_informações: &ExercícioInformações
) -> u32 {
    loop {
        println!(
            "[0 o programa encerra]\nQuantos termos você quer ver?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(quantidade_de_termos) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "A quantidade de {} termos,\nfoi adicionada com sucesso!\n",
                            quantidade_de_termos
                        );

                        return quantidade_de_termos;
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas digite números!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
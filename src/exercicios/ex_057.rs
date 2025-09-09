use std::{
    io::stdin,
    thread::sleep,
    time::Duration
};

use crate::recursos::{
    limpar_terminal::limpar_terminal,
    descricao_de_exercicio::criar_descrição_do_exercício,
    exercicio_informacoes::ExercícioInformações,
    final_do_exercicio::rodar_final_do_exercício
};

mod progressao_aritimetica;

use progressao_aritimetica::PA;

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("057"),
            String::from("Melhore o EX_048, perguntando para o\nusuário se ele quer mostrar mais alguns\ntermos. O programa encerra quando ele\ndisser que quer mostrar 0 termos.")
        )
    );

    exercício_informações.mostrar_informações();

    /* Corpo do Exercício */
    let mut pa = PA::new(
        obter_número_inteiro(
            &exercício_informações, 
            "pa"
        ),
        obter_número_inteiro(
            &exercício_informações,
            "razão"
        )
    );

    mostrar_os_10_primeiros_da_pa(
        &mut pa
    );

    loop {
        let quantidade_de_novos_termos: u32 = obter_quantidade_de_novos_termos(
            &exercício_informações,
            &pa
        );

        if quantidade_de_novos_termos == 0 {
            break;
        } else {
            mostrar_os_x_termos_da_pa(
                &mut pa, 
                quantidade_de_novos_termos
            );
        }
    }

    /* Fim do Exercício */
    rodar_final_do_exercício();
}

fn mostrar_os_x_termos_da_pa(
    pa: &mut PA,
    quantidade_de_novos_termos: u32
) {
    let mut x_próximos_termos: Vec<u32> = vec![];

    for _quantidade in 1..(quantidade_de_novos_termos + 1) {
        pa.buscar_o_próximo_termo();

        x_próximos_termos.push(
            pa.get_próximo_termo()
        );
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Mostrando os {} próximos termos:\n",
        quantidade_de_novos_termos
    );

    sleep(Duration::from_millis(1500));

    println!(
        "{:?}\n",
        x_próximos_termos
    );

    sleep(Duration::from_millis(1100));
}

fn obter_quantidade_de_novos_termos(
    exercício_informações: &ExercícioInformações,
    pa: &PA
) -> u32 {
    loop {
        println!(
            "[0 o programa encerra!]\nQuantos termos quer ver a mais?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        if número > 0 {
                            println!(
                                "A PA de {}, com Razão de {}.\n",
                                pa.get_número(),
                                pa.get_razão()
                            );
    
                            println!(
                                "Os {} próximos termos...\n",
                                número
                            );
                        }

                        return número;
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
            Err(_) => (),
        }
    }
}

fn mostrar_os_10_primeiros_da_pa(
    pa: &mut PA
) {
    let mut dez_primeiros_termos_da_pa: Vec<u32> = vec![
        pa.get_número(), 
        pa.get_próximo_termo()
    ];

    for _quantidade in 1..9 {
        pa.buscar_o_próximo_termo();

        dez_primeiros_termos_da_pa.push(
            pa.get_próximo_termo()
        );
    }

    sleep(Duration::from_millis(1000));

    println!(
        "Mostrando os 10 primeiros termos:\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "{:?}\n",
        dez_primeiros_termos_da_pa
    );

    sleep(Duration::from_millis(1100));
}

fn obter_número_inteiro(
    exercício_informações: &ExercícioInformações,
    nome_da_chamada: &str
) -> u32 {
    loop {
        println!(
            "Digite o número da {}:",
            nome_da_chamada.to_uppercase()
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(número) => {
                        if número == 0 && nome_da_chamada == "razão" {
                            let número: u32 = 1;

                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "A {} de {},\nfoi adicionada com sucesso!\n",
                                nome_da_chamada.to_uppercase(),
                                número
                            );

                            return número;

                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "A {} de {},\nfoi adicionada com sucesso!\n",
                                nome_da_chamada.to_uppercase(),
                                número
                            );

                            return número;
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite número inteiros!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
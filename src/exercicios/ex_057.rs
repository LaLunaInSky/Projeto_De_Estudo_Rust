use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    process::Command
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn descrição_do_exercício() -> String {
    format!(
        "Descrição do exercício 057:
 Melhore o EX_048, perguntando para o\nusuário se ele quer mostrar mais alguns\ntermos. O programa encerra quando ele\ndisser que quer mostrar 0 termos.
"
    )
}

#[derive(Debug)]
struct PA {
    número: u32,
    razão: u32,
    próximo_termo: u32,
}

impl PA {
    fn new(
        número: u32,
        razão: u32
    ) -> Self {
        Self {
            número,
            razão,
            próximo_termo: número + razão
        }
    }

    fn buscar_o_próximo_termo(
        &mut self
    ) {
        self.próximo_termo += self.razão;
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    println!(
        "{}\n{}",
        cabeçalho_do_programa,
        descrição_do_exercício()
    );

    /* Corpo do Exercício */
    let mut pa = PA::new(
        obter_número_inteiro(
            &cabeçalho_do_programa, 
            "pa"
        ),
        obter_número_inteiro(
            &cabeçalho_do_programa,
            "razão"
        )
    );

    mostrar_os_10_primeiros_da_pa(
        &mut pa
    );

    loop {
        let quantidade_de_novos_termos: u32 = obter_quantidade_de_novos_termos(
            &cabeçalho_do_programa,
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
    sleep(Duration::from_millis(3000));

    println!(
        "\nVoltando ao menu de exercícios...\n"
    );

    sleep(Duration::from_millis(3000));

    clean_terminal_linux();
}

fn mostrar_os_x_termos_da_pa(
    pa: &mut PA,
    quantidade_de_novos_termos: u32
) {
    let mut x_próximos_termos: Vec<u32> = vec![];

    for _quantidade in 1..(quantidade_de_novos_termos + 1) {
        pa.buscar_o_próximo_termo();

        x_próximos_termos.push(
            pa.próximo_termo
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

    sleep(Duration::from_millis(1500));
}

fn obter_quantidade_de_novos_termos(
    cabeçalho_do_programa: &String,
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
                        clean_terminal_linux();
                    
                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

                        if número > 0 {
                            println!(
                                "A PA de {}, com Razão de {}.\n",
                                pa.número,
                                pa.razão
                            );
    
                            println!(
                                "Os {} próximos termos...\n",
                                número
                            );
                        }

                        return número;
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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
        pa.número, pa.próximo_termo
    ];

    for _quantidade in 1..9 {
        pa.buscar_o_próximo_termo();

        dez_primeiros_termos_da_pa.push(
            pa.próximo_termo
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

    sleep(Duration::from_millis(1500));
}

fn obter_número_inteiro(
    cabeçalho_do_programa: &String,
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

                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "A {} de {},\nfoi adicionada com sucesso!\n",
                                nome_da_chamada.to_uppercase(),
                                número
                            );

                            return número;

                        } else {
                            clean_terminal_linux();

                            println!(
                                "{}\n{}",
                                cabeçalho_do_programa,
                                descrição_do_exercício()
                            );

                            println!(
                                "A {} de {},\nfoi adicionada com sucesso!\n",
                                nome_da_chamada.to_uppercase(),
                                número
                            );

                            return número;
                        }
                    }
                    Err(_) => {
                        clean_terminal_linux();

                        println!(
                            "{}\n{}",
                            cabeçalho_do_programa,
                            descrição_do_exercício()
                        );

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
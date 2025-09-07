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

mod pessoa;
mod generos;

use pessoa::Pessoa;
use generos::Gêneros;

struct Pessoas {
    lista_de_pessoas: Vec<Pessoa>,
    média_das_idades: u32,
    dados_do_homem_mais_velho: Pessoa,
    lista_de_mulheres_com_menos_de_20_anos: Vec<Pessoa>
}

impl Pessoas {
    fn new() -> Self {
        let dados_do_homem_mais_velho = Pessoa::new(
            String::from("fulano de tal"),
            0,
            'm'
        );

        Self {
            lista_de_pessoas: vec![],
            média_das_idades: 0,
            dados_do_homem_mais_velho,
            lista_de_mulheres_com_menos_de_20_anos: vec![]
        }
    }

    fn adicionar_uma_nova_pessoa(
        &mut self,
        pessoa: Pessoa
    ) {
        if self.lista_de_pessoas.len() == 0 {
            // Calcular a média de idades
            self.média_das_idades = pessoa.get_idade() as u32;

            // Obter os dados do homem mais velho

        } else {
            // Calcular a média de idades
            let mut soma_das_idades: u32 = 0;

            for pessoa_na_lista in self.get_lista_de_pessoas() {
                soma_das_idades += pessoa_na_lista.get_idade() as u32;
            }

            soma_das_idades += pessoa.get_idade() as u32;

            self.média_das_idades = soma_das_idades / self.get_lista_de_pessoas().len() as u32;

            // Obter os dados do homem mais velho

        }

        self.lista_de_pessoas.push(
            pessoa
        );
    }

    fn get_lista_de_pessoas(
        &self
    ) -> Vec<Pessoa> {
        return self.lista_de_pessoas.clone();
    }

    fn get_média_das_idades(
        &self
    ) -> u32 {
        return self.média_das_idades;
    }

    fn get_dados_do_homem_mais_velho(
        &self
    ) -> Pessoa {
        return self.dados_do_homem_mais_velho.clone();
    }

    fn get_lista_de_mulheres_com_menos_de_20_anos(
        &self
    ) -> Vec<Pessoa> {
        return self.lista_de_mulheres_com_menos_de_20_anos.clone();
    }
}

pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("053"),
            String::from("Um programa que lê o nome, idade e\ngênero de 4 pessoas. No final do\nprograma, motra:

- A média de idade do grupo.
- Qual é o nome do homem mais velho.
- Quantas mulheres têm menos de 20 anos.")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut pessoas = Pessoas::new();

        for quantidade in 1..5 {
            pessoas.adicionar_uma_nova_pessoa(
                Pessoa::new(
                    obter_o_nome(
                        quantidade, 
                        &exercício_informações
                    ),
                    obter_a_idade(
                        quantidade, 
                        &exercício_informações
                    ),
                    obter_o_gênero(
                        quantidade, 
                        &exercício_informações
                    )
                )
            );
        }

        analisar_as_pessoas(
            &pessoas
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

fn analisar_as_pessoas(
    pessoas: &Pessoas
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando as pessoas...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "A média de idade é de {}.\nO homem mais velho é o {} de {} anos.\nExiste {} mulheres com menos de 20 anos.\n",
        pessoas.get_média_das_idades(),
        pessoas.get_dados_do_homem_mais_velho().get_nome(),
        pessoas.get_dados_do_homem_mais_velho().get_idade(),
        pessoas.get_lista_de_mulheres_com_menos_de_20_anos().len()
    );

    sleep(Duration::from_millis(1100));
}

fn obter_o_nome(
    index_da_chamada: u8,
    exercício_informações: &ExercícioInformações
) -> String {
    loop {
        println!(
            "Qual o nome da {}ª Pessoa:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let nome = input.trim().to_string();

                limpar_terminal();

                exercício_informações.mostrar_informações();

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

fn obter_a_idade(
    index_da_chamada: u8,
    exercício_informações: &ExercícioInformações
) -> u8 {
    loop {
        println!(
            "Digite a idade da {}ª Pessoa:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(idade) => {
                        if idade <= 100 {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "A idade de {} anos,\nfoi adicionado com sucesso!\n",
                                idade
                            );

                            return idade;
                        } else {
                            limpar_terminal();

                            exercício_informações.mostrar_informações();

                            println!(
                                "Erro! Apenas idade até 100!\n"
                            );
                        }
                    }
                    Err(_) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Digite apenas número!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn obter_o_gênero(
    index_da_chamada: u8,
    exercício_informações: &ExercícioInformações
) -> char {
    loop {
        println!(
            "Digite o gênero: [F/M]"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                let resposta = input.trim().to_lowercase();

                let resposta = resposta.as_str();

                match resposta {
                    "f" | "m" => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        let gênero: char = resposta.parse().unwrap();

                        println!(
                            "O Gênero {},\nfoi adicionado com sucesso!\n",
                            gênero
                        );

                        return gênero;
                    }
                    _ => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "Erro! Apenas é aceito F ou M!\n"
                        );
                    }
                }
            }
            Err(_) => (),
        }
    }
}
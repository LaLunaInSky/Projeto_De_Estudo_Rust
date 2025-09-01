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

enum TiposDeTriângulos {
    NENHUM,
    EQUILATERO,
    ISOSCELES,
    ESCALENO
}

struct Segmentos {
    lista_segmentos: [u32; 3],
    forma_um_triângulo: bool,
    tipo_do_triângulo: TiposDeTriângulos,
}

impl Segmentos {
    fn new() -> Self {
        Self {
            lista_segmentos: [0, 0, 0],
            forma_um_triângulo: false,
            tipo_do_triângulo: TiposDeTriângulos::NENHUM
        }
    }

    fn adicionar_um_segmento(
        &mut self,
        segmento: u32,
        posição_na_lista: usize
    ) {
        self.lista_segmentos[posição_na_lista] = segmento;
    
        self.verificar_se_formar_um_triângulo();
    }

    fn verificar_se_formar_um_triângulo(
        &mut self
    ) {
        if (
            self.lista_segmentos[0] + self.lista_segmentos[1]
        ) > self.lista_segmentos[2] 
          && 
        (
            self.lista_segmentos[0] + self.lista_segmentos[2]
        ) > self.lista_segmentos[1] 
          && 
        (
            self.lista_segmentos[1] + self.lista_segmentos[2]
        ) > self.lista_segmentos[0] 
        {
        
            self.forma_um_triângulo = true;

            self.verificar_o_tipo_do_triângulo();
        }
    }
    
    fn verificar_o_tipo_do_triângulo(
        &mut self
    ) {
        if self.lista_segmentos[0] == self.lista_segmentos[1] 
            && 
           self.lista_segmentos[1] == self.lista_segmentos[2] 
            && 
           self.lista_segmentos[0] == self.lista_segmentos[2] 
        {
            
            self.tipo_do_triângulo = TiposDeTriângulos::EQUILATERO;

        } else if self.lista_segmentos[0] == self.lista_segmentos[1] 
                   || 
                  self.lista_segmentos[0] == self.lista_segmentos[2] 
                   || 
                  self.lista_segmentos[1] == self.lista_segmentos[2] 
        {
            
            self.tipo_do_triângulo = TiposDeTriângulos::ISOSCELES;

        } else {
            
            self.tipo_do_triângulo = TiposDeTriângulos::ESCALENO;
        } 
    }

    fn get_lista_de_segmentos(
        &self
    ) -> [u32; 3] {
        return self.lista_segmentos.clone();
    }

    fn get_forma_um_triângulo(
        &self
    ) -> bool {
        return self.forma_um_triângulo;
    }

    fn get_tipo_do_triângulo(
        &self
    ) -> String {
        match self.tipo_do_triângulo {
            TiposDeTriângulos::EQUILATERO => return "EQUILÁTERO".to_string(),
            TiposDeTriângulos::ISOSCELES => return "ISÓSCELES".to_string(),
            TiposDeTriângulos::ESCALENO => return "ESCALENO".to_string(),
            TiposDeTriângulos::NENHUM => return "NEHHUM".to_string(),
        }
    }
}


pub fn rodar_o_exercício(
    cabeçalho_do_programa: &String
) {
    /* Começo do Exercício */
    let exercício_informações = ExercícioInformações::new(
        &cabeçalho_do_programa,
        criar_descrição_do_exercício(
            String::from("039"),
            String::from("Refaça o EX_032 dos triângulos\nacrescentando recurso de mostrar que tipo\nde triângulo será formado:
        
- Equilátero: todos os lados iguais
- Isósceles: dois lados iguais
- Escaleno: todos os lados diferentes")
        )
    );

    loop {
        exercício_informações.mostrar_informações();

        /* Corpo do Exercício */
        let mut segmentos = Segmentos::new();

        for indice in 1..3 {
            segmentos.adicionar_um_segmento(
                obter_o_lado_de_um_suposto_triângulo(
                    &exercício_informações, 
                    indice
                ),
                (indice - 1) as usize
            );
        }

        analisar_os_segmentos(
            &segmentos
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

fn analisar_os_segmentos(
    segmentos: &Segmentos
) {
    sleep(Duration::from_millis(1000));

    println!(
        "Analisando os segmentos...\n"
    );

    sleep(Duration::from_millis(1500));

    println!(
        "O segmentos {:?},\n{}formam um triângulo!\n",
        segmentos.get_lista_de_segmentos(),
        if !segmentos.get_forma_um_triângulo() {"NÃO "} else {""}
    );

    if segmentos.get_forma_um_triângulo() {
        println!(
            "\nEste é um triângulo {}!\n",
            segmentos.get_tipo_do_triângulo()
        )
    }

    sleep(Duration::from_millis(1100));
}

fn obter_o_lado_de_um_suposto_triângulo(
    exercício_informações: &ExercícioInformações,
    index_da_chamada: u8
) -> u32 {
    loop {
        println!(
            "Digite o {}º Segmento:",
            index_da_chamada
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(segmento) => {
                        limpar_terminal();

                        exercício_informações.mostrar_informações();

                        println!(
                            "O segmento de {},\nfoi adicionado com sucesso!\n",
                            segmento
                        );

                        return segmento;
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
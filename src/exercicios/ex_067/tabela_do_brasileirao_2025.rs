use crate::exercicios::ex_067::orgnizacao_alfabeticamente::orginizar_alfabéticamente;

pub struct TabelaDoBrasileirão2025 {
    tabela_em_ordem_de_colocação: Vec<String>,
    cinco_primeiros_colocados: Vec<String>,
    quatro_últimos_colocados: Vec<String>,
    tabela_em_ordem_alfabética: Vec<String>,
    posição_do_time_chapecoense: i8
}

impl TabelaDoBrasileirão2025 {
    pub fn new() -> Self {
        let tabela_em_ordem_de_colocação: Vec<String> = vec![
            String::from("flamengo"), String::from("cruzeiro"), String::from("palmeiras"), String::from("mirassol"),  String::from("bahia"), String::from("botafogo"), String::from("são paulo"), String::from("bragantino"), String::from("corinthians"), String::from("fluminense"), String::from("ceará"), String::from("internacional"), String::from("atlético-MG"), String::from("grêmio"), String::from("vasco"), String::from("santos"), String::from("vitória"), String::from("juventude"), String::from("fortaleza"), String::from("sport")
        ];

        let mut cinco_primeiros_colocados: Vec<String> = vec![];

        let mut quatro_últimos_colocados: Vec<String> = vec![];

        let tabela_em_ordem_alfabética: Vec<String> = orginizar_alfabéticamente(
            &tabela_em_ordem_de_colocação
        );

        let mut posição_do_time_chapecoense: i8 = -1;

        for (
            index, 
            time
        ) in tabela_em_ordem_de_colocação.iter().enumerate() {
            if index <= 4 {
                cinco_primeiros_colocados.push(
                    time.clone()
                );
            } else if index >= 16 {
                quatro_últimos_colocados.push(
                    time.clone()
                );
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

    pub fn get_tabela_em_ordem_de_colocação(
        &self
    ) -> String {
        let mut tabela_em_string: String = String::new();

        for (
            index,
            time
        ) in self.tabela_em_ordem_de_colocação.iter().enumerate() {
            if index == (self.tabela_em_ordem_de_colocação.len() - 1) {
                tabela_em_string.push_str(
                    format!(
                        "{}.",
                        time
                    ).as_str()
                );
            } else if index != 0 && index % 3 == 0 {
                tabela_em_string.push_str(
                    format!(
                        "{},\n",
                        time
                    ).as_str()
                );
            } else {
                tabela_em_string.push_str(
                    format!(
                        "{}, ",
                        time
                    ).as_str()
                );
            }
        }

        return tabela_em_string;
    }

    pub fn get_cinco_primeiros_colocados(
        &self
    ) -> String {
        let mut tabela_em_string = String::new();

        for (
            index,
            time
        ) in self.cinco_primeiros_colocados.iter().enumerate() {
            if index == (self.cinco_primeiros_colocados.len() - 1) {
                tabela_em_string.push_str(
                    format!(
                        "{}.",
                        time
                    ).as_str()
                );
            } else if index != 0 && index % 2 == 0 {
                tabela_em_string.push_str(
                    format!(
                        "{},\n",
                        time
                    ).as_str()
                );
            } else {
                tabela_em_string.push_str(
                    format!(
                        "{}, ",
                        time
                    ).as_str()
                );
            }
        }

        return tabela_em_string;
    }

    pub fn get_quatro_últimos_colocados(
        &self
    ) -> String {
                let mut tabela_em_string = String::new();

        for (
            index,
            time
        ) in self.quatro_últimos_colocados.iter().enumerate() {
            if index == (self.quatro_últimos_colocados.len() - 1) {
                tabela_em_string.push_str(
                    format!(
                        "{}.",
                        time
                    ).as_str()
                );
            } else if index != 0 && index % 2 == 0 {
                tabela_em_string.push_str(
                    format!(
                        "{},\n",
                        time
                    ).as_str()
                );
            } else {
                tabela_em_string.push_str(
                    format!(
                        "{}, ",
                        time
                    ).as_str()
                );
            }
        }

        return tabela_em_string;
    }

    pub fn get_tabela_em_ordem_alfabética(
        &self
    ) -> String {
        let mut tabela_em_string = String::new();

        for (
            index,
            time
        ) in self.tabela_em_ordem_alfabética.iter().enumerate() {
            if index == (self.tabela_em_ordem_alfabética.len() - 1) {
                tabela_em_string.push_str(
                    format!(
                        "{}.",
                        time
                    ).as_str()
                );
            } else if index != 0 && index % 3 == 0 && index > 3 || index == 2 {
                tabela_em_string.push_str(
                    format!(
                        "{},\n",
                        time
                    ).as_str()
                );
            } else {
                tabela_em_string.push_str(
                    format!(
                        "{}, ",
                        time
                    ).as_str()
                );
            }
        }

        return tabela_em_string;
    }

    pub fn get_posição_do_time_chapecoense(
        &self
    ) -> i8 {
        return self.posição_do_time_chapecoense;
    }
}
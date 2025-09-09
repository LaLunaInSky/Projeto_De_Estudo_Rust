pub fn formatar_a_descrição(
    descrição_do_exercício: &String
) -> String {
    let linhas_na_frase: Vec<&str> = descrição_do_exercício.split("\n").collect();    

    let mut descrição_final = String::new();

    let limite_de_chars_por_linha: usize = 42;

    for linha in linhas_na_frase {
        let mut nova_linha = linha.to_string();

        let mut parágrafos: Vec<String> = vec![];

        while nova_linha.len() > limite_de_chars_por_linha {
            let mut index_do_ultimo_espaço: usize = 0;

            for (
                index, char
            ) in nova_linha.char_indices() {
                if index <= limite_de_chars_por_linha + 1 {            
                    if char == ' ' {
                        index_do_ultimo_espaço = index;
                    }
                }
            } 

            parágrafos.push(
                nova_linha[
                    0..index_do_ultimo_espaço
                ].to_string()
            );

            nova_linha.replace_range(
                0..index_do_ultimo_espaço+1,
                ""
            );
        }

        parágrafos.push(
            nova_linha
        );

        if parágrafos.len() > 1 {
            nova_linha = String::from("");
    
            for parágrafo in parágrafos {
                nova_linha.push_str(
                    format!(
                        "{}\n",
                        parágrafo
                    ).as_str()
                );
            }
        } else {
            nova_linha = parágrafos[0].to_string();
        }

        descrição_final.push_str(
            format!(
                "{}\n",
                nova_linha.trim_end()
            ).as_str()
        );
    }

    return descrição_final.trim_end().to_string();
}
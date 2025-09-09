use serde::{
    Deserialize,
    Serialize
};

use std::fs::read_to_string;

use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct ListaDeExercícios {
    número_do_exercício: u32,
    descrição: String
}

pub fn obter_a_descrição_do_exercício_x(
    número_do_exercício: u32
) -> String {
    let json = read_to_string(
        "./src/lista_de_exercícios.json"
    ).unwrap();

    let lista_de_exercícios = from_str::<Vec<ListaDeExercícios>>(
        &json
    );

    match lista_de_exercícios {
        Ok(lista) => {
            return lista[
                (número_do_exercício - 1) as usize
            ].descrição
            .to_string();
        },
        Err(_) => return String::from("erro!"),
    }
}
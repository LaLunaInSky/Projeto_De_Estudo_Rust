pub fn orginizar_alfabéticamente(
    lista_de_string: &Vec<String>
) -> Vec<String> {
    let cópia_lista = lista_de_string.clone();

    let mut lista_organizada: Vec<String> = vec![];

    let alfabeto: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    for letra in alfabeto.clone() {
        for letra_2 in alfabeto.clone() {
            for palavra in &cópia_lista {
                let palavra_separada: Vec<char> = palavra.chars().collect();

                if palavra_separada[0] == letra && palavra_separada[1] == letra_2 {
                    lista_organizada.push(
                        palavra.clone()
                    );
                }
            }
        }
    }

    return lista_organizada;
}
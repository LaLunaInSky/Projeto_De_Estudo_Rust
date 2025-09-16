pub fn orginizar_alfabéticamente(
    lista_de_string: &Vec<String>
) -> Vec<String> {
    let mut cópia_lista = lista_de_string.clone();

    let mut lista_organizada: Vec<String> = vec![];

    let alfabeto = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"  
    ];

    println!(
        "{:?}",
        lista_organizada
    );

    return lista_organizada;
}
use crate::lista_de_exercicios::lista_de_exercícios;

pub fn obter_a_descrição_do_exercício_x(
    número_do_exercício: u32
) -> String {
    let descrição_do_exercício = lista_de_exercícios(
        número_do_exercício
    );

    return descrição_do_exercício;
}
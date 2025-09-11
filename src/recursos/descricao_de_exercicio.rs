mod separacao_do_numero_do_exercicio;
mod formatacao_da_descricao;

use separacao_do_numero_do_exercicio::separar_o_número_do_exercício;
use formatacao_da_descricao::formatar_a_descrição;
use crate::lista_de_exercicios::obter_a_descrição_do_exercício_x; 

pub fn buscar_descrição_do_exercício(
    número_do_exercício: String
) -> String {
    let número_do_exercício_separado = separar_o_número_do_exercício(
        &número_do_exercício
    );

    let descrição_do_exercício_x = obter_a_descrição_do_exercício_x(
        número_do_exercício_separado
    );

    let descrição_do_exercício_x = formatar_a_descrição(
        &descrição_do_exercício_x
    );

    format!(
        "Descrição do exercício {}:
 {}
",
        número_do_exercício,
        descrição_do_exercício_x
    )
}
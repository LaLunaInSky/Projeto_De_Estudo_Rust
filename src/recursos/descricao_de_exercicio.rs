pub fn criar_descrição_do_exercício(
    número_do_exercício: String,
    descrição_do_exercício: String
) -> String {
    format!(
        "Descrição do exercício {}:
 {}
",
        número_do_exercício,
        descrição_do_exercício
    )
}

mod separacao_do_numero_do_exercicio;
mod obtencao_de_descricao_de_exercicio;

use separacao_do_numero_do_exercicio::separar_o_número_do_exercício;
use obtencao_de_descricao_de_exercicio::obter_a_descrição_do_exercício_x;

pub fn buscar_descrição_do_exercício(
    número_do_exercício: String
) -> String {
    let número_do_exercício_separado = separar_o_número_do_exercício(
        &número_do_exercício
    );

    let descrição_do_exercício_x = obter_a_descrição_do_exercício_x(
        número_do_exercício_separado
    );

    format!(
        "Descrição do exercício {}:
 {}        
",
        número_do_exercício,
        descrição_do_exercício_x
    )
}
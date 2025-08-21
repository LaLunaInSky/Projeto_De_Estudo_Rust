pub fn descrição_de_exercício(
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
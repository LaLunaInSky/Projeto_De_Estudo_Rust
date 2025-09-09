pub fn separar_o_número_do_exercício(
    número_do_exercício: &String
) -> u32 {
    let número_do_exercício = número_do_exercício.replace(
        "ex_", 
        ""
    );

    let número_do_exercício: u32 = número_do_exercício.parse().unwrap();

    return número_do_exercício;
}
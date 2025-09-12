pub fn arredondadar_preço(
    número_real: f32
) -> f32 {
    let número_string = format!(
        "{:.2}",
        número_real
    );

    match número_string.trim().parse::<f32>() {
        Ok(número_arredondado) => {
            return número_arredondado;
        }
        Err(_) => return número_real,
    }
}
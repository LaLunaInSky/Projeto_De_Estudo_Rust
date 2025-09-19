pub fn arredondador_de_número_real(
    número_real: f32
) -> f32 {
    let número_formatado = format!(
        "{:.2}",
        número_real
    );

    match número_formatado.parse::<f32>() {
        Ok(número_final) => {
            return número_final;
        }
        Err(_) => {
            return número_real;
        }
    }
}
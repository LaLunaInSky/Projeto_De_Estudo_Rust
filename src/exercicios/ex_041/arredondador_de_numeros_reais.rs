pub fn arrendondar_um_número_real(
    número: f32, 
    quantidade_depois_do_ponto: u8
) -> f32 {
    let mut número_formatado = String::new();
    
    match quantidade_depois_do_ponto {
        1 => {
            número_formatado = format!(
                "{:.1}",
                número
            );
        }
        2 => {
            número_formatado = format!(
                "{:.2}",
                número
            );
        }
        _ => (),
    }

    match número_formatado.parse::<f32>() {
        Ok(número_final) => número_final,
        Err(_) => 0.0,
    }
}
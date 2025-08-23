pub struct Parede {
    pub largura: f32,
    pub altura: f32,
    pub área: f32
}

impl Parede {
    pub fn new(
        largura: f32,
        altura: f32
    ) -> Self {
        let área: f32 = altura * largura;
        let área = format!(
            "{:.1}",
            área
        );

        match área.parse::<f32>() {
            Ok(área) => {
                return Self {
                    largura,
                    altura,
                    área
                };
            }
            Err(_) => (),
        }

        Self {
            largura,
            altura,
            área: 0.0
        }
    }
}
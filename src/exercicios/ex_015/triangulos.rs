pub struct Triângulo {
    pub cateto_oposto: u32,
    pub cateto_adjacente: u32,
    pub hipotenusa: f64
}

impl Triângulo {
    pub fn new(
        cateto_oposto: u32,
        cateto_adjacente: u32
    ) -> Self {
        let hipotenusa = (
            (
                (cateto_adjacente * cateto_adjacente) 
                + 
                (cateto_oposto * cateto_oposto)
            ) as f64
        ).sqrt();

        Self {
            cateto_oposto,
            cateto_adjacente,
            hipotenusa
        }
    }
}
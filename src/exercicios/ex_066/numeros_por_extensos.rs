pub struct NúmerosPorExtenso {
    números_por_extenso_até_vinte: [String; 21]
}

impl NúmerosPorExtenso {
    pub fn new() -> Self {
        let números_por_extenso_até_vinte: [String; 21] = [
            String::from("zero"), String::from("um"), String::from("dois"), String::from("três"), String::from("quatro"), String::from("cinco"), String::from("seis"), String::from("sete"), String::from("oito"), String::from("nove"), String::from("dez"), String::from("onze"), String::from("doze"), String::from("treze"), String::from("quatorze"), String::from("quinze"), String::from("dezesseis"), String::from("dezessete"), String::from("dezoito"), String::from("dezenove"), String::from("vinte")
        ];

        Self {
            números_por_extenso_até_vinte
        }
    }

    pub fn get_número_por_extenso(
        &self,
        número_inteiro: u8
    ) -> String {
        return self.números_por_extenso_até_vinte[
            número_inteiro as usize
        ].clone();
    }
}
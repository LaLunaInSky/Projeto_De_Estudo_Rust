use crate::exercicios::ex_039::tipos_de_triangulos::TiposDeTriângulos;

pub struct Segmentos {
    lista_segmentos: [u32; 3],
    forma_um_triângulo: bool,
    tipo_do_triângulo: TiposDeTriângulos,
}

impl Segmentos {
    pub fn new() -> Self {
        Self {
            lista_segmentos: [0, 0, 0],
            forma_um_triângulo: false,
            tipo_do_triângulo: TiposDeTriângulos::NENHUM
        }
    }

    pub fn adicionar_um_segmento(
        &mut self,
        segmento: u32,
        posição_na_lista: usize
    ) {
        self.lista_segmentos[posição_na_lista] = segmento;
    
        self.verificar_se_formar_um_triângulo();
    }

    fn verificar_se_formar_um_triângulo(
        &mut self
    ) {
        if (
            self.lista_segmentos[0] + self.lista_segmentos[1]
        ) > self.lista_segmentos[2] 
          && 
        (
            self.lista_segmentos[0] + self.lista_segmentos[2]
        ) > self.lista_segmentos[1] 
          && 
        (
            self.lista_segmentos[1] + self.lista_segmentos[2]
        ) > self.lista_segmentos[0] 
        {
        
            self.forma_um_triângulo = true;

            self.verificar_o_tipo_do_triângulo();
        }
    }
    
    fn verificar_o_tipo_do_triângulo(
        &mut self
    ) {
        if self.lista_segmentos[0] == self.lista_segmentos[1] 
            && 
           self.lista_segmentos[1] == self.lista_segmentos[2] 
            && 
           self.lista_segmentos[0] == self.lista_segmentos[2] 
        {
            
            self.tipo_do_triângulo = TiposDeTriângulos::EQUILATERO;

        } else if self.lista_segmentos[0] == self.lista_segmentos[1] 
                   || 
                  self.lista_segmentos[0] == self.lista_segmentos[2] 
                   || 
                  self.lista_segmentos[1] == self.lista_segmentos[2] 
        {
            
            self.tipo_do_triângulo = TiposDeTriângulos::ISOSCELES;

        } else {
            
            self.tipo_do_triângulo = TiposDeTriângulos::ESCALENO;
        } 
    }

    pub fn get_lista_de_segmentos(
        &self
    ) -> [u32; 3] {
        return self.lista_segmentos.clone();
    }

    pub fn get_forma_um_triângulo(
        &self
    ) -> bool {
        return self.forma_um_triângulo;
    }

    pub fn get_tipo_do_triângulo(
        &self
    ) -> String {
        match self.tipo_do_triângulo {
            TiposDeTriângulos::EQUILATERO => return "EQUILÁTERO".to_string(),
            TiposDeTriângulos::ISOSCELES => return "ISÓSCELES".to_string(),
            TiposDeTriângulos::ESCALENO => return "ESCALENO".to_string(),
            TiposDeTriângulos::NENHUM => return "NEHHUM".to_string(),
        }
    }
}
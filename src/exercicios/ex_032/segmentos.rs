pub struct Segmentos {
    lista_de_segmentos: [u32; 3],
    forma_um_triângulo: bool
}

impl Segmentos {
    pub fn new() -> Self {
        Self {
            lista_de_segmentos: [0, 0, 0],
            forma_um_triângulo: false
        }
    }

    pub fn adicionar_novo_segmento(
        &mut self,
        indice_da_chamada: usize,
        segmento: u32
    ) {
        self.lista_de_segmentos[indice_da_chamada] = segmento;
    }

    fn analisar_se_forma_um_triângulo(
        &mut self
    ) {
        if self.lista_de_segmentos[0] + self.lista_de_segmentos[1] > self.lista_de_segmentos [2] && 
           self.lista_de_segmentos[0] + self.lista_de_segmentos[2] > self.lista_de_segmentos[1] && 
           self.lista_de_segmentos[1] + self.lista_de_segmentos[2] > self.lista_de_segmentos[0] 
        {
            self.forma_um_triângulo = true;
        }
    }

    pub fn get_lista_de_segmentos(
        &self
    ) -> [u32; 3] {
        return self.lista_de_segmentos.clone();
    }

    pub fn get_forma_um_triângulo(
        &mut self
    ) -> bool {
        self.analisar_se_forma_um_triângulo();

        return self.forma_um_triângulo;
    }
}
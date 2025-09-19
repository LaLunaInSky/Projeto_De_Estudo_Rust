pub struct PalavrasESuasVogais {
    lista_de_palavras: Vec<String>,
    lista_de_vogais_de_cada_palavra: Vec<String>
}

impl PalavrasESuasVogais {
    pub fn new() -> Self {
        Self {
            lista_de_palavras: vec![],
            lista_de_vogais_de_cada_palavra: vec![]
        }
    }

    pub fn adicionar_uma_nova_palavra(
        &mut self, 
        palavra: String
    ) {
        self.lista_de_palavras.push(
            palavra.clone()
        );

        let vogais = [
            'a', 'e', 'i', 'o', 'u'
        ];

        let mut vogais_da_palavra = String::new();

        for vogal in vogais {
            for letra in palavra.chars() {
                if letra == vogal {
                    if vogais_da_palavra.len() > 0 {
                        vogais_da_palavra.push_str(
                            ", "
                        );
                    } 

                    vogais_da_palavra.push(
                        letra
                    );
                }
            }
        }

        self.lista_de_vogais_de_cada_palavra.push(
            vogais_da_palavra
        );
    }

    pub fn get_lista_de_palavras(
        &self
    ) -> Vec<String> {
        return self.lista_de_palavras.clone();
    }

    pub fn get_lista_de_vogais_de_cada_palavra(
        &self
    ) -> Vec<String> {
        return self.lista_de_vogais_de_cada_palavra.clone();
    }
}
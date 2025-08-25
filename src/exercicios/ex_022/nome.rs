pub struct Nome {
    pub nome: String,
    pub possui_silva: bool,
}

impl Nome {
    pub fn new(
        nome: String
    ) -> Self {
        let mut possui_silva = false;

        if nome.contains("silva") {
            possui_silva = true;
        }

        Self {
            nome,
            possui_silva
        }
    }
}
pub struct Frase {
    _frase: String,
    frase_sem_espaços: String,
    é_palíndromo: bool,
}

impl Frase {
    pub fn new(
        frase: String
    ) -> Self {
        let mut frase_sem_espaços = String::new();

        for char in frase.chars() {
            if char != ' ' {
                let char = format!(
                    "{}",
                    char
                );

                let char = char.as_str();

                frase_sem_espaços.push_str(char);
            }
        }

        let é_palíndromo = if frase_sem_espaços == frase_sem_espaços.chars().rev().collect::<String>() {
            true
        } else {
            false
        };

        Self {
            _frase: frase,
            frase_sem_espaços,
            é_palíndromo
        }
    }

    pub fn get_frase_sem_espaços(
        &self
    ) -> String {
        return self.frase_sem_espaços.clone();
    }

    pub fn get_é_palíndromo(
        &self
    ) -> bool {
        return self.é_palíndromo;
    }
}
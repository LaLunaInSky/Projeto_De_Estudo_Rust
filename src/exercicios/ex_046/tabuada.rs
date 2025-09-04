pub struct Tabuada {
    número: u32
}

impl Tabuada {
    pub fn new(
        número: u32
    ) -> Self {
        Self {
            número
        }
    }

    pub fn mostrar_tabuada(
        &self
    ) {
        for count in 1..11 {
            println!(
                "{} X {:>2} = {}",
                self.número, 
                count,
                (self.número * count)
            );
        }

        println!();
    }
}
pub struct Números {
    pub números: Vec<u32>,
    pub soma_dos_números: u32
}

impl Números {
    pub fn new(
        números: Vec<u32>
    ) -> Self {
        let mut soma_dos_números: u32 = 0;

        for número in &números {
            soma_dos_números += número;
        }

        Self {
            números,
            soma_dos_números
        }
    }
}
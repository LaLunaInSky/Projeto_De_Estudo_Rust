pub struct ValorASacar {
    _valor: u32,
    quantidade_de_notas_de_cinquenta: u8,
    quantidade_de_notas_de_vinte: u8,
    quantidade_de_notas_de_dez: u8,
    quantidade_de_notas_de_um: u8
}

impl ValorASacar {
    pub fn new(
        valor: u32
    ) -> Self {
        let mut valor_do_saque = valor;

        let mut quantidade_de_notas_de_cinquenta: u8 = 0;
 
        let mut quantidade_de_notas_de_vinte: u8 = 0;

        let mut quantidade_de_notas_de_dez: u8 = 0;

        let mut quantidade_de_notas_de_um: u8 = 0;

        while valor_do_saque > 0 {
            if valor_do_saque % 50 == 0 {
                valor_do_saque -= 50;

                quantidade_de_notas_de_cinquenta += 1;
            } else if valor_do_saque % 20 == 0 {
                valor_do_saque -= 20;

                quantidade_de_notas_de_vinte += 1;
            } else if valor_do_saque % 10 == 0 {
                valor_do_saque -= 10;

                quantidade_de_notas_de_dez += 1;
            } else {
                valor_do_saque -= 1;

                quantidade_de_notas_de_um += 1;
            }
        }

        Self {
            _valor: valor,
            quantidade_de_notas_de_cinquenta,
            quantidade_de_notas_de_vinte,
            quantidade_de_notas_de_dez,
            quantidade_de_notas_de_um
        }
    }

    pub fn get_quantidade_de_notas_de_cinquenta(
        &self
    ) -> u8 {
        return self.quantidade_de_notas_de_cinquenta;
    }

    pub fn get_quantidade_de_notas_de_vinte(
        &self
    ) -> u8 {
        return self.quantidade_de_notas_de_vinte;
    }

    pub fn get_quantidade_de_notas_de_dez(
        &self
    ) -> u8 {
        return self.quantidade_de_notas_de_dez;
    }

    pub fn get_quantidade_de_notas_de_um(
        &self
    ) -> u8 {
        return self.quantidade_de_notas_de_um;
    }
}
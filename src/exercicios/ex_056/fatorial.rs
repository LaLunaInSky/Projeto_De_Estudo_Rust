pub struct Fatorial {
    _número: u32,
    fatorial: String
}

impl Fatorial {
    pub fn new(
        número: u32
    ) -> Self {

        fn obter_o_fatorial(
            número: &u32
        ) -> String {
            let mut fatorial = format!(
                "{}! = ",
                *número
            );

            let mut multiplicação: u64 = 1;
            let mut número_do_fatorial = *número;

            while número_do_fatorial > 0 {
                let mut _número_formatado = String::new();

                multiplicação *= número_do_fatorial as u64;

                if número_do_fatorial > 1 {
                    _número_formatado = format!(
                        "{} x ",
                        número_do_fatorial
                    );
                } else {
                    _número_formatado = format!(
                        "{} = {}",
                        número_do_fatorial,
                        multiplicação
                    )
                }

                let número_formatado = _número_formatado.as_str();

                fatorial.push_str(
                    número_formatado
                );

                número_do_fatorial -= 1;
            }

            return fatorial;
        }

        let fatorial = obter_o_fatorial(
            &número
        );

        Self {
            _número: número,
            fatorial
        }
    }

    pub fn get_fatorial(
        &self
    ) -> String {
        return self.fatorial.clone();
    }
}
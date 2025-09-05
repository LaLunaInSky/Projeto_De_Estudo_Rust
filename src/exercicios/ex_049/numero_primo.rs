pub struct NúmeroPrimo {
    número: u32,
    é_primo: bool
}

impl NúmeroPrimo {
    pub fn new(
        número: u32
    ) -> Self {
        fn identificar_se_é_primo(
            número: &u32
        ) -> bool {
            if *número > 1 {    
                if número % 2 == 0 {
                    if *número == 2 {
                        return true;
                    } else {
                        return false;
                    }
                } else if número % 3 == 0 {
                    if *número == 3 {
                        return true;
                    } else {
                        return false;
                    }
                } else if número % 5 == 0 {
                    if *número == 5 {
                        return true;
                    } else {
                        return false
                    }
                } else if número % 7 == 0 {
                    if *número == 7 {
                        return true;
                    } else {
                        return false;
                    }
                }

                return true;
            } else {
                return false;
            }
        }

        let é_primo = identificar_se_é_primo(
            &número
        );

        Self {
            número,
            é_primo
        }
    }

    pub fn get_número(
        &self
    ) -> u32 {
        return self.número;
    }

    pub fn get_é_primo(
        &self
    ) -> bool {
        return self.é_primo;
    }
}
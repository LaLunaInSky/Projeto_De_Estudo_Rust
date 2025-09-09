pub struct PA {
    número: u32,
    razão: u32,
    próximo_termo: u32,
}

impl PA {
    pub fn new(
        número: u32,
        razão: u32
    ) -> Self {
        Self {
            número,
            razão,
            próximo_termo: número + razão
        }
    }

    pub fn buscar_o_próximo_termo(
        &mut self
    ) {
        self.próximo_termo += self.razão;
    }

    pub fn get_número(
        &self
    ) -> u32 {
        return self.número;
    }

    pub fn get_razão(
        &self
    ) -> u32 {
        return self.razão;
    }

    pub fn get_próximo_termo(
        &self
    ) -> u32 {
        return self.próximo_termo;
    }
}
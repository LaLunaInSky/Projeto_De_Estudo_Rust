pub struct Comprimentos {
    pub km: f32,
    pub hm: f32,
    pub dam: f32,
    pub m: f32,
    pub dm: f32,
    pub cm: f32,
    pub mm: f32
}

impl Comprimentos {
    pub fn new(
        metro: f32
    ) -> Self {
        let km = metro / 1000.0;
        let hm = metro / 100.0;
        let dam = metro / 10.0;
        let dm = metro * 10.0;
        let cm = metro * 100.0;
        let mm = metro * 1000.0;

        Self {
            km,
            hm,
            dam,
            m: metro,
            dm,
            cm,
            mm
        }
    }
}
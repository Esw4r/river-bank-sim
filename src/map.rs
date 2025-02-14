struct Map {
    n: u8,
    eqn_deg: u8,
    a_coeffs: Vec<u8>,
    b_coeffs: Vec<u8>,
    river_width: u8,
}

impl Map {
    pub fn river_eqn(&self, x: u8) -> u8 {
        let mut y = 0;
        let mut j = 0;
        for _ in 1..self.eqn_deg {
            let a = self.a_coeffs[j] as f64;
            let b = self.b_coeffs[j] as f64;
            let x = x as f64;
            let n = self.n as f64;
            let cur_term: f64 = a*((b*3.14*x)/n).sin();
            y += cur_term as u8;
            j += 1;
        }
        y+x
    }
}
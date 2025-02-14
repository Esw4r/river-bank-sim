use crate::bison;

pub struct Lion {
    name: [String; 2],
    health: u8,
    age: u8,
    exp: u8,
    gender: bool,
    hunger: u8,
    thirst: u8,
    speed: u8,
    vision: u8,
    stamina: u8,
    strength: u8,
    dir: u8, // 0 to 179
    eye: [u8; 2],
    ear: [u8; 2],
    limbs: [u8; 4],
    head: u8,
    torso: u8,
    genitals: u8,
    fov: u8,
    posn: [u8; 2],
    fear: u8,
    herd_tend: u8,
    bison_cache: Vec<bison::Bison>,
    lion_cache: Vec<Lion>,
}

impl Lion {
    pub fn get_name(&self) -> &[String; 2] {
        &self.name
    }
    pub fn set_name(mut self, x: &str, y: &str) {
        self.name = [x.to_string(), y.to_string()];
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }
    pub fn set_health(mut self, x: u8) {
        self.health = x;
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
    pub fn set_age(mut self, x: u8) {
        self.age = x;
    }

    pub fn get_exp(&self) -> u8 {
        self.exp
    }
    pub fn set_exp(mut self, x: u8) {
        self.exp = x;
    }

    pub fn get_gen(&self) -> bool {
        self.gender
    }
    pub fn set_gen(mut self, x: bool) {
        self.gender = x;
    }

    pub fn get_hunger(&self) -> u8 {
        self.hunger
    }
    pub fn set_hunger(mut self, x: u8) {
        self.hunger = x;
    }

    pub fn get_thirst(&self) -> u8 {
        self.thirst
    }
    pub fn set_thirst(mut self, x: u8) {
        self.thirst = x;
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }
    pub fn set_speed(mut self, x: u8) {
        self.speed = x;
    }

    pub fn get_vision(&self) -> u8 {
        self.vision
    }
    pub fn set_vision(mut self, x: u8) {
        self.vision = x;
    }

    pub fn get_stamina(&self) -> u8 {
        self.stamina
    }
    pub fn set_stamina(mut self, x: u8) {
        self.stamina = x;
    }

    pub fn get_strength(&self) -> u8 {
        self.strength
    }
    pub fn set_strength(mut self, x: u8) {
        self.strength = x;
    }

    pub fn get_dir(&self) -> u8 {
        self.dir
    }
    pub fn set_dir(mut self, x: u8) {
        self.dir = x;
    }

    pub fn get_eye(&self) -> [u8; 2] {
        self.eye
    }
    pub fn set_eye(mut self, x: u8, y: u8) {
        self.eye = [x, y];
    }

    pub fn get_ear(&self) -> [u8; 2] {
        self.ear
    }
    pub fn set_ear(mut self, x: u8, y: u8) {
        self.ear = [x, y];
    }

    pub fn get_limbs(&self) -> [u8; 4] {
        self.limbs
    }
    pub fn set_limbs(mut self, w: u8, x: u8, y: u8, z: u8) {
        self.limbs = [w, x, y, z];
    }

    pub fn get_head(&self) -> u8 {
        self.head
    }
    pub fn set_head(mut self, x: u8) {
        self.head = x;
    }

    pub fn get_torso(&self) -> u8 {
        self.torso
    }
    pub fn set_torso(mut self, x: u8) {
        self.torso = x;
    }

    pub fn get_genitals(&self) -> u8 {
        self.genitals
    }
    pub fn set_genitals(mut self, x: u8) {
        self.genitals = x;
    }

    pub fn get_fov(&self) -> u8 {
        self.fov
    }
    pub fn set_fov(mut self, x: u8) {
        self.fov = x;
    }

    pub fn get_posn(&self) -> [u8; 2] {
        self.posn
    }
    pub fn set_posn(mut self, x: u8, y: u8) {
        self.posn = [x, y];
    }

    pub fn get_fear(&self) -> u8 {
        self.fear
    }
    pub fn set_fear(mut self, x: u8) {
        self.fear = x;
    }

    pub fn get_herd_tend(&self) -> u8 {
        self.herd_tend
    }
    pub fn set_herd_tend(mut self, x: u8) {
        self.herd_tend = x;
    }
}

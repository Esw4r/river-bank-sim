use crate::lion;

pub struct Bison {
    first_name: String,
    last_name: String,
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
    dir_x: u8,
    dir_y: u8,
    left_eye: u8,
    right_eye: u8,
    left_ear: u8,
    right_ear: u8,
    front_left_leg: u8,
    front_right_leg: u8,
    rear_left_leg: u8,
    rear_right_leg: u8,
    head: u8,
    torso: u8,
    genitals: u8,
    fov: u8,
    posn_x: u8,
    posn_y: u8,
    fear: u8,
    herd_tend: u8,
    bison_cache: Vec<lion::Lion>,
    lion_cache: Vec<Bison>,
}

impl Bison {
    pub fn get_first_name(&self) -> &String { &self.first_name }
    pub fn set_first_name(mut self, x: &str) { self.first_name = x.to_string(); }

    pub fn get_last_name(&self) -> &String { &self.last_name }
    pub fn set_last_name(mut self, x: &str) { self.last_name = x.to_string(); }

    pub fn get_health(&self) -> u8 { self.health }
    pub fn set_health(mut self, x: u8) { self.health = x; }

    pub fn get_age(&self) -> u8 { self.age }
    pub fn set_age(mut self, x: u8) { self.age = x; }
    
    pub fn get_exp(&self) -> u8 { self.exp }
    pub fn set_exp(mut self, x: u8) { self.exp = x; }

    pub fn get_gen(&self) -> bool { self.gender }
    pub fn set_gen(mut self, x: bool) { self.gender = x; }
    
    pub fn get_hunger(&self) -> u8 { self.hunger }
    pub fn set_hunger(mut self, x: u8) { self.hunger = x; }
    
    pub fn get_thirst(&self) -> u8 { self.thirst }
    pub fn set_thirst(mut self, x: u8) { self.thirst = x; }
    
    pub fn get_speed(&self) -> u8 { self.speed }
    pub fn set_speed(mut self, x: u8) { self.speed = x; }
    

    pub fn get_vision(&self) -> u8 { self.vision }
    pub fn set_vision(mut self, x: u8) { self.vision = x; }

    pub fn get_stamina(&self) -> u8 { self.stamina }
    pub fn set_stamina(mut self, x: u8) { self.stamina = x; }

    pub fn get_strength(&self) -> u8 { self.strength }
    pub fn set_strength(mut self, x: u8) { self.strength = x; }

    pub fn get_dir_x(&self) -> u8 { self.dir_x }
    pub fn set_dir_x(mut self, x: u8) { self.dir_x = x; }

    pub fn get_dir_y(&self) -> u8 { self.dir_y }
    pub fn set_dir_y(mut self, x: u8) { self.dir_y = x; }

    pub fn get_left_eye(&self) -> u8 { self.left_eye }
    pub fn set_left_eye(mut self, x: u8) { self.left_eye = x; }

    pub fn get_right_eye(&self) -> u8 { self.right_eye }
    pub fn set_right_eye(mut self, x: u8) { self.right_eye = x; }

    pub fn get_left_ear(&self) -> u8 { self.left_ear }
    pub fn set_left_ear(mut self, x: u8) { self.left_ear = x; }

    pub fn get_front_left_leg(&self) -> u8 { self.front_left_leg }
    pub fn set_front_left_leg(mut self, x: u8) { self.front_left_leg = x; }
    
    pub fn get_front_right_leg(&self) -> u8 { self.front_right_leg }
    pub fn set_front_right_leg(mut self, x: u8) { self.front_right_leg = x; }

    pub fn get_rear_left_leg(&self) -> u8 { self.rear_left_leg }
    pub fn set_rear_left_leg(mut self, x: u8) { self.rear_left_leg = x; }

    pub fn get_rear_right_leg(&self) -> u8 { self.rear_right_leg }
    pub fn set_rear_right_leg(mut self, x: u8) { self.rear_right_leg = x; }

    pub fn get_head(&self) -> u8 { self.head }
    pub fn set_head(mut self, x: u8) { self.head = x; }

    pub fn get_torso(&self) -> u8 { self.torso }
    pub fn set_torso(mut self, x: u8) { self.torso = x; }

    pub fn get_genitals(&self) -> u8 { self.genitals }
    pub fn set_genitals(mut self, x: u8) { self.genitals = x; }

    pub fn get_fov(&self) -> u8 { self.fov }
    pub fn set_fov(mut self, x: u8) { self.fov = x; }

    pub fn get_posn_x(&self) -> u8 { self.posn_x }
    pub fn set_posn_x(mut self, x: u8) { self.posn_x = x; }

    pub fn get_posn_y(&self) -> u8 { self.posn_y }
    pub fn set_posn_y(mut self, x: u8) { self.posn_y = x; }

    pub fn get_fear(&self) -> u8 { self.fear }
    pub fn set_fear(mut self, x: u8) { self.fear = x; }

    pub fn get_herd_tend(&self) -> u8 { self.herd_tend }
    pub fn set_herd_tend(mut self, x: u8) { self.herd_tend = x; }
}
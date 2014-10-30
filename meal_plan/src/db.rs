use std::io::BufferedWriter;
use std::io::File;
use std::io::fs::PathExtensions;

#[deriving(Decodable, Encodable)]
pub enum WeightType {
    G, Dkg, Kg
}


impl WeightType {
    pub fn to_g(&self, value: i32) -> f32 {
        match *self {
            G => value as f32,
            Dkg => (value as f32 * 100f32),
            Kg => (value as f32 * 1000f32),
        }
    }
}

#[deriving(Decodable, Encodable)]
pub struct Food {
    pub name: String,
    pub size: i32, 
    pub weight_type: WeightType,
    pub protein: f32,
    pub ch: f32,
    pub fat: f32,
    pub price: i32,
    pub price_weight: i32,
    pub price_weight_type: WeightType,
}

impl Food {
    pub fn new() -> Food {
        Food {
            name: "".into_string(),
            size: 100, 
            weight_type: G,
            protein: 0f32,
            ch: 0f32,
            fat: 0f32,
            price: 0,
            price_weight: 100,
            price_weight_type: G,
        }
    }

    pub fn get_kcal(&self) -> i32 {
        (self.protein * 4f32 + self.ch * 4f32 + self.fat*9f32) as i32
    }
}

pub struct Dao;

impl Dao {
    pub fn new() -> Dao {
        Dao
    }

    pub fn load_foods(&self) -> Vec<Food> {
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\foods.csv")).has_headers(false);
        rdr.decode().map(|r| r.unwrap()).collect::<Vec<Food>>()
    }

    pub fn persist_foods(&mut self, foods: &[Food]) {
        let mut enc = ::csv::Writer::from_file(&Path::new("data\\foods.csv"));
        for food in foods.iter() {
            enc.encode(food);
        }
        enc.flush();
    }
}
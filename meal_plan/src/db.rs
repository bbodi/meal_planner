
use std::io;
use imgui::base;

#[deriving(Decodable, Encodable)]
pub enum WeightType {
    G, Dkg, Kg, Lb
}

#[deriving(Decodable, Encodable)]
impl WeightType {
    pub fn to_g(&self, value: f32) -> f32 {
        match *self {
            G => value,
            Dkg => (value * 100f32),
            Kg => (value * 1000f32),
            Lb => 1000f32*(value * 0.453592f32),
        }
    }
}

impl base::IndexValue for WeightType {
    fn set(&mut self, value: uint) {
        *self = match value {
            0 => G,
            1 => Dkg,
            2 => Kg,
            _ => Lb,
        };
    }
    fn get(&self) -> uint {
        *self as uint
    }
}



#[deriving(Decodable, Encodable)]
pub enum GoalType {
    Bulking, Cutting
}

impl base::IndexValue for GoalType {
    fn set(&mut self, value: uint) {
        *self = match value {
            0 => Bulking,
            _ => Cutting,
        };
    }
    fn get(&self) -> uint {
        *self as uint
    }
}

#[deriving(Decodable, Encodable)]
pub enum ActivityModifier {
    Sedentary, Lightly, Moderately, Very, Extremely
}

impl base::IndexValue for ActivityModifier {
    fn set(&mut self, value: uint) {
        *self = match value {
            0 => Sedentary,
            1 => Lightly,
            2 => Moderately,
            3 => Very,
            _ => Extremely,
        };
    }
    fn get(&self) -> uint {
        *self as uint
    }
}

impl ActivityModifier {
    pub fn get_modified_value(&self, value: f32) -> f32 {
        match *self {
            Sedentary => (value as f32 * 1.2f32),
            Lightly => (value as f32 * 1.375f32),
            Moderately => (value as f32 * 1.55f32),
            Very => (value as f32 * 1.725f32),
            Extremely => (value as f32 * 1.9f32),
        }
    }
}


#[deriving(Decodable, Encodable)]
pub struct RecommendedMacros {
    pub protein: i32,
    pub ch: i32,
    pub fat: i32,
    pub age: i32,
    pub height: i32,
    pub weight: f32,
    pub weight_type: WeightType,
    pub activity_mod: ActivityModifier,
    pub height_type: i32,
    pub goal_type: GoalType,
    pub protein_per_kg: f32,
    pub protein_percent: f32,
    pub ch_percent: f32,
    pub fat_percent: f32,
    pub bmr: f32,
    pub target_calories: f32,
}

impl RecommendedMacros {
    pub fn new(p: i32, ch: i32, f: i32) -> RecommendedMacros {
        RecommendedMacros {
            protein: p,
            ch: ch,
            fat: f,
            age: 0,
            height: 0,
            weight: 0f32,
            weight_type: Kg,
            activity_mod: Sedentary,
            height_type: 0,
            goal_type: Bulking,
            protein_per_kg: 0f32,
            protein_percent: 0f32,
            ch_percent: 0f32,
            fat_percent: 0f32,
            bmr: 0f32,
            target_calories: 0f32,
        }
    }
}

#[deriving(Decodable, Encodable)]
pub struct Food {
    pub id: uint,
    pub name: String,
    pub weight: f32,
    pub weight_type: WeightType,
    pub protein: f32,
    pub ch: f32,
    pub fat: f32,
    pub price: i32,
    pub price_weight: i32,
    pub price_weight_type: WeightType,
}

impl Food {
    pub fn new(id: uint) -> Food {
        Food {
            id: id,
            name: "".into_string(),
            weight: 100f32,
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

#[deriving(Decodable, Encodable)]
pub struct MealFood {
    id: uint,
    pub food_id: uint,
    pub weight: f32,
    pub weight_type: WeightType,
}

impl MealFood {
    pub fn new(id: uint, food_id: uint) -> MealFood {
        MealFood { 
            id: id,
            food_id: food_id,
            weight: 0f32,
            weight_type: G,
        }
    }
}

#[deriving(Decodable, Encodable)]
pub struct Meal {
    id: uint,
    pub name: String,
    pub foods: Vec<MealFood>,
}

impl Meal {
    pub fn new(id: uint) -> Meal {
        Meal {
            name: "".into_string(),
            foods: vec![],
            id: id,
        }
    }
}

#[deriving(Decodable, Encodable)]
pub struct DailyMenu {
    pub id: uint,
    pub name: String, 
    pub meals: Vec<Meal>
}

impl DailyMenu {
    pub fn new(id: uint) -> DailyMenu {
        DailyMenu {
            id: id,
            name: "".into_string(),
            meals: vec![],
        }
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
    }

    fn remove_meal_food(meal_food_id: uint, meal_foods: &mut Vec<MealFood>) -> MealFood {
        let mut idx = 0;
        for (i, meal_food) in meal_foods.iter().enumerate() {
            if meal_food.id == meal_food_id {
                idx = i;
                break;
            }
        }
        return meal_foods.remove(idx).unwrap();
    }

    fn remove_meal(meal_id: uint, meals: &mut Vec<Meal>) -> Meal {
        let mut idx = 0;
        for (i, meal) in meals.iter().enumerate() {
            if meal.id == meal_id {
                idx = i;
                break;
            }
        }
        return meals.remove(idx).unwrap();
    }

    pub fn load_daily_menus(&self) -> Vec<DailyMenu> {
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\meal_foods.csv")).has_headers(false);
        let mut meal_foods = rdr.decode().map(|r| r.unwrap()).collect::<Vec<MealFood>>();

        let mut meals = vec![];
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\meals.csv")).has_headers(false);
        for row in rdr.decode() {
            let (id, name, meal_food_ids): (uint, String, String) = row.unwrap();
            let mut meal = Meal::new(id);
            meal.name = name.into_string();
            for id in meal_food_ids.split(';') {
                let id = from_str::<uint>(id).unwrap_or(0);
                if id == 0 {
                    continue;
                }
                let meal_food = Dao::remove_meal_food(id, &mut meal_foods);
                meal.foods.push(meal_food);
            }
            meals.push(meal);
        }

        let mut daily_menus = vec![];
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\dailies.csv")).has_headers(false);
        for row in rdr.decode() {
            let (id, name, meal_ids): (uint, String, String) = row.unwrap();
            let mut daily_menu = DailyMenu::new(id);
            daily_menu.name = name.into_string();
            for id in meal_ids.split(';') {
                let id = from_str::<uint>(id).unwrap_or(0);
                if id == 0 {
                    continue;
                }
                let meal = Dao::remove_meal(id, &mut meals);
                daily_menu.meals.push(meal);
            }
            daily_menus.push(daily_menu);
        }
        return daily_menus;
    }

    pub fn persist_daily_menu(&mut self, daily_menus: &[DailyMenu]) {
        let mut enc = ::csv::Writer::from_file(&Path::new("data\\dailies.csv"));
        let mut meal_writer = ::csv::Writer::from_file(&Path::new("data\\meals.csv"));
        let mut meal_food_writer = ::csv::Writer::from_file(&Path::new("data\\meal_foods.csv"));

        for daily_menu in daily_menus.iter() {
            for meal in daily_menu.meals.as_slice().iter() {
                for meal_food in meal.foods.as_slice().iter() {
                    meal_food_writer.encode(*meal_food);
                }
                let dao = (meal.id, meal.name.as_slice(), meal.foods.iter().fold("".into_string(), |a, b| a + format!("{};", b.id)));
                meal_writer.encode(dao);
            }
            let dao = (daily_menu.id, daily_menu.name.as_slice(), daily_menu.meals.iter().fold("".into_string(), |a, b| a + format!("{};", b.id)));
            enc.encode(dao);
        }
    }

    pub fn load_recommended_macros(&self) -> RecommendedMacros {
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\recommended.csv")).has_headers(false);
        return match rdr.decode().take(1).next() {
            Some(r) => r.unwrap(),
            None => RecommendedMacros::new(0, 0, 0),
        };
    }

    pub fn persist_recommended_macros(&mut self, recommended_macros: &RecommendedMacros) {
        let mut enc = ::csv::Writer::from_file(&Path::new("data\\recommended.csv"));
        enc.encode(recommended_macros);
    }
}

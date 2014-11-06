
use imgui::base;

#[deriving(Decodable, Encodable, PartialEq, PartialOrd)]
pub enum FoodType {
    Meat, Fruit, Vegetable, Nuts, Legume, Dairy, Egg, Grain
}

impl FoodType {
    pub fn names() -> [&'static str, ..8] {
        ["Hús", "Gyümölcs", "Zöldség", "Mag", "Hüvelyes", "Tej", "Tojás", "Gabona"]
    }
}

impl base::IndexValue for FoodType {
    fn set(&mut self, value: uint) {
        unsafe {
            *(self as *mut FoodType as *mut uint) = value;
        }
    }
    fn get(&self) -> uint {
        *self as uint
    }
}

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
pub struct MacroNutrient {
    pub protein: f32,
    pub ch: f32,
    pub fat: f32,
}

impl MacroNutrient {
    pub fn new(p: f32, ch: f32, f: f32) -> MacroNutrient {
        MacroNutrient {
            protein: p,
            ch: ch,
            fat: f,
        }
    }

    pub fn kcal(&self) -> f32 {
        self.protein * 4f32 + self.ch * 4f32 + self.fat*9f32
    }
}

impl Add<MacroNutrient, MacroNutrient> for MacroNutrient {
    fn add(&self, rhs: &MacroNutrient) -> MacroNutrient {
        let p = self.protein + rhs.protein;
        let ch = self.ch + rhs.ch;
        let fat = self.fat + rhs.fat;
        MacroNutrient::new(p, ch, fat)
    }
}

#[deriving(Decodable, Encodable)]
pub struct NutritionGoal {
    pub macros: MacroNutrient,
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

impl NutritionGoal {
    pub fn new(p: f32, ch: f32, f: f32) -> NutritionGoal {
        NutritionGoal {
            macros: MacroNutrient::new(p, ch, f),
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
    pub food_type: FoodType,
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
            food_type: Meat,
        }
    }

    pub fn get_kcal(&self) -> i32 {
        (self.protein * 4f32 + self.ch * 4f32 + self.fat*9f32) as i32
    }
}

#[deriving(Decodable, Encodable)]
pub struct MealFood {
    parent_id: uint,
    pub food_id: uint,
    pub weight: f32,
    pub weight_type: WeightType,
}

impl MealFood {
    pub fn new(meal: &Meal, food_id: uint) -> MealFood {
        MealFood { 
            parent_id: meal.id,
            food_id: food_id,
            weight: 0f32,
            weight_type: G,
        }
    }
}

#[deriving(Decodable, Encodable)]
pub struct Meal {
    id: uint,
    parent_id: uint,
    pub name: String,
    pub foods: Vec<MealFood>,
}

impl Meal {

    fn new(id: uint, name: String, parent_id: uint) -> Meal {
        Meal {
            name: name,
            foods: vec![],
            id: id,
            parent_id: parent_id,
        }
    }

    pub fn add_food(&mut self, food_id: uint) {
        let meal_food = MealFood::new(self, food_id);
        self.foods.push(meal_food)
    }
    pub fn add_meal_food(&mut self, meal_food: MealFood) {
        self.foods.push(meal_food);
    }

    pub fn from_meal(id: uint, src: &Meal) -> Meal {
        let mut meal = Meal::new(id, src.name.clone(), src.parent_id);
        for meal_food in src.foods.iter() {
            let mut new_meal_food = *meal_food;
            new_meal_food.parent_id = id;
            meal.foods.push(new_meal_food);
        }
        meal
    }

    pub fn id(&self) -> uint {self.id}
}

#[deriving(Decodable, Encodable)]
pub struct DailyMenu {
    id: uint,
    pub name: String, 
    pub meals: Vec<Meal>
}

impl DailyMenu {
    pub fn new(id: uint, name: String) -> DailyMenu {
        DailyMenu {
            id: id,
            name: name,
            meals: vec![],
        }
    }

    pub fn add_new_meal(&mut self, meal_id: uint) {
        let meal = Meal {
            name: "".into_string(),
            foods: vec![],
            id: meal_id,
            parent_id: self.id,
        };
        self.meals.push(meal);
    }

    pub fn add_meal(&mut self, mut meal: Meal) {
        meal.parent_id = self.id;
        self.meals.push(meal);   
    }

    pub fn id(&self) -> uint {self.id}
}

pub struct Dao;

impl Dao {
    pub fn new() -> Dao {
        Dao
    }

    pub fn load_foods(&self) -> Vec<Food> {
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\foods.csv")).has_headers(false);
        rdr.decode().map(|r| {let r: Food = r.unwrap();println!("{}", r.name);r}).collect::<Vec<Food>>()
    }

    pub fn persist_foods(&mut self, foods: &[Food]) {
        let mut enc = ::csv::Writer::from_file(&Path::new("data\\foods.csv"));
        for food in foods.iter() {
            let _ = enc.encode(food);
        }
    }

    fn get_daily_menu<'a>(daily_menu_id: uint, daily_menus: &'a mut Vec<DailyMenu>) -> &'a mut DailyMenu {
        match daily_menus.iter_mut().filter(|x| x.id == daily_menu_id).next() {
            None => panic!("DailyMenu not found: {}", daily_menu_id),
            Some(m) => m
        }
    }

    fn get_meal<'a>(meal_id: uint, meals: &'a mut Vec<Meal>) -> &'a mut Meal {
        match meals.iter_mut().filter(|x| x.id == meal_id).next() {
            None => panic!("Meal not found: {}", meal_id),
            Some(m) => m
        }
    }

    pub fn load_daily_menus(&self) -> Vec<DailyMenu> {
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\meal_foods.csv")).has_headers(false);
        let meal_foods = rdr.decode().map(|r| r.unwrap()).collect::<Vec<MealFood>>();

        let mut meals = vec![];
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\meals.csv")).has_headers(false);
        for row in rdr.decode() {
            let (id, name, parent_id): (uint, String, uint) = row.unwrap();
            let meal = Meal::new(id, name.into_string(), parent_id);
            meals.push(meal);
        }

        let mut daily_menus = vec![];
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\dailies.csv")).has_headers(false);
        for row in rdr.decode() {
            let (id, name): (uint, String) = row.unwrap();
            let daily_menu = DailyMenu::new(id, name.into_string());
            daily_menus.push(daily_menu);
        }
        for meal_food in meal_foods.into_iter() {
            let parent_meal = Dao::get_meal(meal_food.parent_id, &mut meals);
            parent_meal.add_meal_food(meal_food);
        }
        for meal in meals.into_iter() {
            let parent_daily_menu = Dao::get_daily_menu(meal.parent_id, &mut daily_menus);
            parent_daily_menu.add_meal(meal);   
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
                    let _ = meal_food_writer.encode(*meal_food);
                }
                let dao = (meal.id, meal.name.as_slice(), meal.parent_id);
                // , meal.foods.iter().fold("".into_string(), |a, b| a + format!("{};", b.id)));
                let _ = meal_writer.encode(dao);
            }
            let dao = (daily_menu.id, daily_menu.name.as_slice());
            let _ = enc.encode(dao);
        }
    }

    pub fn load_nutritional_goals(&self) -> NutritionGoal {
        let mut rdr = ::csv::Reader::from_file(&Path::new("data\\recommended.csv")).has_headers(false);
        return match rdr.decode().take(1).next() {
            Some(r) => r.unwrap(),
            None => NutritionGoal::new(0f32, 0f32, 0f32),
        };
    }

    pub fn persist_nutritional_goals(&mut self, recommended_macros: &NutritionGoal) {
        let mut enc = ::csv::Writer::from_file(&Path::new("data\\recommended.csv"));
        let _ = enc.encode(recommended_macros);
    }
}

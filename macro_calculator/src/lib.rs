use json::{object, JsonValue};

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let (mut cals, mut carbs, mut proteins, mut fats) = (0.0, 0.0, 0.0, 0.0);

    for f in foods {
        let kcal: f64 = f.calories.1
            .strip_suffix("kcal")
            .unwrap()
            .parse()
            .unwrap();

        cals += kcal * f.nbr_of_portions;
        carbs += f.carbs * f.nbr_of_portions;
        proteins += f.proteins * f.nbr_of_portions;
        fats += f.fats * f.nbr_of_portions;
    }

    object! {
        cals: round2(cals),
        carbs: round2(carbs),
        proteins: round2(proteins),
        fats: round2(fats)
    }
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
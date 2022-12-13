pub struct Elf {
    food: Vec<i32>
}

impl Elf {
    pub fn new(food: Vec<i32>) -> Self {
        Self {
            food: food
        }
    }

    pub fn get_total_calories(&self) -> i32 {
        self.food
            .iter()
            .sum::<i32>()
    }
}
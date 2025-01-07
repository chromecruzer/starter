// #![allow(dead_code)]

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Others,
}
pub struct Criteria {
    age: i32,
    gender: Gender,
    nationality: String,
}

impl Criteria {
    pub fn new(age: i32, gender: Gender, nationality: String) -> Criteria {
        Criteria {
            age,
            gender,
            nationality,
        }
    }

    pub fn display_stats(&self) {
        println!(
            "Your age is {}, nationality {}, And you are a {:?}!!",
            self.age, self.nationality, self.gender
        )
    }

    pub fn mutable_opt(&mut self, val: i32) {
        self.gender = Gender::Others;
        println!("{:?}, {}", self.gender, val)
    }
}

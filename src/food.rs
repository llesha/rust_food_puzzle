use std::fmt::Debug;

pub trait FoodTrait {
    fn cut(&self) {
        println!("Cut!")
    }
    fn dice(&self) {}
    fn carve(&self) {}

    fn boil(&self) {}
    fn fry(&self) {}
    fn steam(&self) {}
    fn bake(&self) {}
    fn grill(&self) {}
}

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub emoji: String,
    pub taste: Taste,
}

impl FoodTrait for Food {}

pub trait Liquid {
    fn pour(&self) {}
}

#[derive(Debug)]
pub struct Taste {
    pub umami: i8,
    pub sour: i8,
    pub sweet: i8,
    pub spicy: i8,
}

/// calories are in , other are in grams per 100 gram
pub struct Value {
    calories: i32,
    protein: i8,
    fat: i8,
    carbohydrate: i8,
    fiber: i8,
}

struct Operations {
    made: [Operation],
}

enum Operation {
    Cut,
    Dice,
    Carve,
    Boil,
    Fry,
    Steam,
    Bake,
    Grill,
}

pub enum Liquids {
    Water,
    Oil,
}

pub enum Porridge {
    Rice,
    Oats,
}

pub enum Vegetables {
    Carrot,
    Cucumber,
    Eggplant,
    Potato,
    Pumpkin,
    Tomato,
}
pub enum Fruits {
    Apple,
    Banana,
    Pear,
}
pub enum Nuts {
    Almond,
    Wallnut,
}

#[derive(Debug)]
pub struct FoodGroup {
    pub foods: Vec<Food>,
}

pub enum Container {
    Pan,
    Bowl,
    Plate,
}

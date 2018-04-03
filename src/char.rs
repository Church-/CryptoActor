extern crate rouler;
extern crate rand;
extern crate fake;

mod data;
 
use std::collections::HashMap;
use rouler::roll_dice;
use rand::Rng;
 
//Set up a char struct to pass around instead of several free variables and a hashmap.
#[derive(Debug)]
struct Char {
    name: String,
    trueName: String,
    race: String,
    age: i64,
    stats: HashMap<&'a str, i64>,
    health: i64,
    mana: i64,
    skills: Vec<String>,
    personality: Personality,
    looks: Looks
 
}
#[derive(Debug)] 
struct Looks {
    build: String,
    eyes: String,
    hair: String,
    skin: String    
}

#[derive(Debug)]
struct Personality {
     desires: String,
     fears: String,
     tends: String,
     used: String
 }
 
// Rewrite entire generator as methods implemented on your char struct and acting on it's fields.
//Much more clean this way.
impl Char {
 
    pub fn new(&mut self) -> Char {
        char = Char {
            name: "",
            trueName: "",
            race: "",
            stats: HashMap::new(),
            health: 0,
            mana: 0,
            skills: Vec::new(),
            personality: self.personality = Personality {
                desires: "",
                fears: "",
                tends: "",
                used: ""
            },
            looks: self.looks = Looks {
                build: "",
                eyes: "",
                hair: "",
                skin: ""
            }
            
        };
   } 
 
    fn genName(&mut self) {
        self.name = fake::Name::first_name;
        self.trueName = fake::Name::first_name;
    }
 
    fn genRace(&mut self) {
        let raceList = ["Human","Elf","Dwarf"];
        self.race = raceList[Rng::thread_rng::gen_range(0,raceList.len())];
    }
 
    fn genAge(&mut self) {
        match &self.race {
        "Elf" => self.age = Rng::thread_rng::gen_range(26,500),
        "Human" => self.age = Rng::thread_rng::gen_range(15,100),
        "Dwarf" => self.age = Rng::thread_rng::gen_range(20,300),
        _ => println!("Incorrect race detected."),
        }
    }
 
    fn genStats(&mut self) {
        let mut statRange = ['4', '6', '8'];
        let statList = ["Agillity","Endurance",""];
        for i in statList.iter() {
            self.stats.insert(i,statRange[Rng::thread_rng::gen_range(0,statRange.len())]);
        }
        self.health = self.stats.get("Endurance") + 5 ;        
        self.mana = self.stats.get("Will") + 5;
    }
 
    fn genSkills(&self) {
 
        
    }
    
    fn genPersonality(&mut self) {
        self.personality.desires = data::data::desires[Rng::thread_rng::gen_range(0,data::data::desires.len())];
        self.personality.fears = data::data::fears[Rng::thread_rng::gen_range(0,data::data::fears.len())];
        self.personality.tends = data::data::tendsTo[Rng::thread_rng::gen_range(0,data::data::tendsTo.len())];
        self.personality.used = data::data::usedTo[Rng::thread_rng::gen_range(0,data::data::usedTo.len())];
    }
 
    fn genLooks(&mut self) {
        self.looks.build = data::data::build[Rng::thread_rng::gen_range(0,data::data::build.len())];
        self.looks.eyes = data::data::eyes[Rng::thread_rng::gen_range(0,data::data::eyes.len())];
        self.looks.hair = data::data::hair[Rng::thread_rng::gen_range(0,data::data::hair.len())];
        self.looks.skin = data::data::skin[Rng::thread_rng::gen_range(0,data::data::skin.len())];
 
 
        }
 }
 
//All going to be deleted during restructure, keeping to use as guide during reimplementation.
//Use a hashmap instead of a vec for storing char attributes and stats. 
 
fn genStats(char: &mut Char,statList: [&str], diceType: &str ) {
    let mut stat = roll_dice(diceType);
    for i in statList.iter() {
        char.stats.insert(i,stat);
        stat = roll_dice(diceType);
    }
}
 
 
fn main() {}

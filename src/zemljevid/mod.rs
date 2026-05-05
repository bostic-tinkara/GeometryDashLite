use crate::{ovire::*, player::*};

enum Ovire {
    Kvadrat,
    Trikotnik,
}

pub struct Zemljevid {
    stopnja: Stopnja,
    poligon: Vec<(f64, Ovire)>,
}

enum Stopnja {
    Beginner,
}

impl Zemljevid {
    fn new() -> Self {
        Zemljevid {
            stopnja: Stopnja::Beginner,
            poligon: vec![]
        }
        
    }
}


// work in progress
// struct OviraZemlj {
//     pozicija: f64,
//     lik: Ovire,
// }

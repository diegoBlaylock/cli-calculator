use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use rand::{rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arity {
    pub lower: Option<u8>,
    pub upper: Option<u8>,
}

impl Arity {
    pub fn is_valid(&self, length: usize) -> bool {
        self.lower.map_or(true, |i| i as usize <= length)
            && self.upper.map_or(true, |i| i as usize >= length)
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub arity: Arity,
    pub function: Arc<dyn Fn(&[f64]) -> f64 + Send + Sync + 'static>,
}

pub static FUNCTIONS: LazyLock<HashMap<String, Function>> = LazyLock::new(move || {
    let sine: Function = Function {
        name: "sine".to_string(),
        arity: Arity {
            lower: Some(1),
            upper: Some(1),
        },
        function: Arc::new(move |v| v[0].sin()),
    };

    let log: Function = Function {
        name: "log".to_string(),
        arity: Arity {
            lower: Some(1),
            upper: Some(1),
        },
        function: Arc::new(|v| v[0].ln()),
    };

    let rand: Function = Function {
        name: "rand".to_string(),
        arity: Arity {
            lower: Some(0),
            upper: Some(0),
        },
        function: Arc::new(move |_| rng().random()),
    };

    let sqrt: Function = Function {
        name: "sqrt".to_string(),
        arity: Arity {
            lower: Some(1),
            upper: Some(1),
        },
        function: Arc::new(|v| v[0].sqrt()),
    };

    let mut map = HashMap::<String, Function>::new();

    map.insert("sin".to_string(), sine.clone());
    map.insert("sine".to_string(), sine.clone());
    map.insert("log".to_string(), log);
    map.insert("rand".to_string(), rand);
    map.insert("sqrt".to_string(), sqrt);
    map
});

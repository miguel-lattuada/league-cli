use std::collections::HashMap;
use lazy_static::{lazy_static};

lazy_static! {
    pub static ref Queues: HashMap<&'static u64, &'static str> = {
        let mut map = HashMap::new();
        map.insert(&420, "Ranked Solo");
        map
    };
    
    pub static ref Champions: HashMap<u64, String> = {
        let mut map = HashMap::new();
        map.insert(147, "Seraphine".to_owned());
        map.insert(121, "Kha'Zix".to_owned());
        map
    };
}
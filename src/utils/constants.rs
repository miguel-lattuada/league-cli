use std::collections::HashMap;
use lazy_static::{lazy_static};

lazy_static! {
    pub static ref QUEUES: HashMap<&'static u64, &'static str> = {
        let mut map = HashMap::new();
        map.insert(&420, "Ranked Solo");
        map.insert(&450, "ARAM");
        map
    };
    
    pub static ref CHAMPIONS: HashMap<u64, String> = {
        let mut map = HashMap::new();
        map.insert(147, "Seraphine".to_owned());
        map.insert(121, "Kha'Zix".to_owned());
        map.insert(54, "Malphite".to_owned());
        map.insert(412, "Thresh".to_owned());
        map.insert(91, "Talon".to_owned());
        map
    };
}
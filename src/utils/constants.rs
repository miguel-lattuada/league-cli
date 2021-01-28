use std::collections::HashMap;
use lazy_static::{lazy_static};

lazy_static! {
    pub static ref Queues: HashMap<u64, String> = {
        let mut map = HashMap::new();
        map.insert(420, "Ranked Solo".to_owned());
        map
    };
}
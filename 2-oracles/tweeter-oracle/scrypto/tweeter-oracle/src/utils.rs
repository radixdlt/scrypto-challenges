use scrypto::prelude::*;

pub fn insert_items(
    key: String,
    hashmap: &mut HashMap<String, HashSet<String>>,
    new_items: HashSet<String>,
) {
    let collections = hashmap.entry(key).or_insert(HashSet::new());

    for item in new_items {
        if   !(*item).is_empty() && !collections.contains(&item) {
            collections.insert(item);
        }
    }
}

pub fn remove_items(
    key: String,
    hashmap: &mut HashMap<String, HashSet<String>>,
    remove_items: HashSet<String>,
) {
    let collections = hashmap.entry(key).or_insert(HashSet::new());

    for item in remove_items {
        if collections.contains(&item) {
            collections.remove(&item);
        }
    }
}

pub fn is_item_exist(
    key: String,
    hashmap: &mut HashMap<String, HashSet<String>>,
    item: String,
) -> bool {
    let collections = hashmap.entry(key).or_insert(HashSet::new());
    return collections.contains(&item);
}

use crate::database::Database;

fn remove_suffix(string: &mut String, suffix: &'static str) {
    if string.ends_with(suffix) {
        string.replace_range((string.len() - suffix.len())..string.len(), "");
    }
}

pub fn lookup(mut entry: String, database: &mut Database) -> Option<String> {
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    if entry.ends_with("ies") {
        entry.replace_range((entry.len() - 3)..entry.len(), "");
        entry.push('y');
    }

    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "ly");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "ness");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "ed");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "ings");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "ing");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "al");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    remove_suffix(&mut entry, "s");
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    if entry.is_empty() { return None; }

    let result = database.lookup(&format!("{}e", entry));
    if result.is_some() { return result; }

    entry.remove(entry.len() - 1);
    let result = database.lookup(&entry);
    if result.is_some() { return result; }

    None
}
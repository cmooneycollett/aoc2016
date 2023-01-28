use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Represents a single room as described in the AOC 2016 Day 4 problem
/// (https://adventofcode.com/2016/day/4).
pub struct Room {
    name: String,
    sector_id: u64,
    checksum: String,
}

impl Room {
    pub fn new(name: &str, sector_id: u64, checksum: &str) -> Room {
        Room {
            name: name.to_string(),
            sector_id,
            checksum: checksum.to_string(),
        }
    }

    /// Gets a reference to the "name" field.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Gets the value of the "sector_id" field.
    pub fn sector_id(&self) -> u64 {
        self.sector_id
    }

    /// Gets a reference to the "checksum" field.
    pub fn checksum(&self) -> &String {
        &self.checksum
    }

    /// Checks if the encrypted room name is valid according to the room checksum.
    pub fn is_real_room(&self) -> bool {
        // Char counts
        let mut counts: HashMap<char, i64> = HashMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            if let Entry::Vacant(e) = counts.entry(c) {
                e.insert(1);
            } else {
                *counts.get_mut(&c).unwrap() += 1;
            }
        }
        if counts.len() < 5 {
            return false;
        }
        // Sort elements by count (highest to lowest) then alphabetical order
        let mut elements = counts.into_iter().collect::<Vec<(char, i64)>>();
        elements.sort_by_key(|a| (-a.1, a.0));
        // Generate output string to check against checksum
        let mut checksum_candidate = String::new();
        for c in elements.iter().map(|t| t.0).take(5) {
            checksum_candidate.push(c);
        }
        checksum_candidate == self.checksum
    }

    /// Determines the unencrypted name for the room.
    pub fn decrypted_name(&self) -> String {
        unimplemented!();
    }
}

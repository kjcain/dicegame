use crate::dice::Die;

#[derive(Debug, Clone)]
pub struct Roll {
    pub die: Die,
    pub value: u8,
}

impl Roll {
    pub fn new(die: Die) -> Self {
        let value = die.roll();
        Roll { die, value }
    }
}

#[derive(Debug)]
pub struct Game {
    pub dice: Vec<Die>,
    pub target: u32,
}

impl Game {
    pub fn new(dice: Vec<Die>, target: u32) -> Self {
        Game {
            dice,
            target,
        }
    }
    
    pub fn play(&self) -> bool {
        // 1. Roll all dice
        let mut rolls: Vec<Roll> = self.dice.iter().map(|die| Roll::new(die.clone())).collect();
        log::trace!("Initial rolls: {:?}", rolls);
        
        // 2. Group dice by the rolled value
        let mut groups: std::collections::HashMap<u8, Vec<usize>> = std::collections::HashMap::new();
        for (idx, roll) in rolls.iter().enumerate() {
            groups.entry(roll.value).or_insert_with(Vec::new).push(idx);
        }
        
        // 3. Remove the smallest die by number of faces from each group (with matches)
        // Also remove all dice with unique values (no matches)
        let mut indices_to_remove = Vec::new();
        for (value, indices) in groups.iter() {
            if indices.len() > 1 {
                // For groups with matches, find the die with the smallest number of sides
                if let Some(&smallest_idx) = indices.iter()
                    .min_by_key(|&&idx| rolls[idx].die.sides) {
                    indices_to_remove.push(smallest_idx);
                    log::trace!("Removing {} (d{}) from group with value {}", 
                                rolls[smallest_idx].die.to_string(), 
                                rolls[smallest_idx].die.sides, 
                                value);
                }
            } else {
                // For unique values, remove the die entirely
                indices_to_remove.push(indices[0]);
                log::trace!("Removing {} with unique value {}", 
                            rolls[indices[0]].die.to_string(), 
                            value);
            }
        }
        
        // Sort in reverse order to remove from back to front (avoid index shifting)
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        
        for idx in indices_to_remove {
            rolls.remove(idx);
        }
        
        log::trace!("After removing matching dice: {:?}", rolls);
        
        // 4. Roll the remaining dice again
        let final_rolls: Vec<Roll> = rolls.iter().map(|roll| Roll::new(roll.die.clone())).collect();
        
        log::trace!("Final rolls: {:?}", final_rolls);
        
        // 5. Sum the results of the remaining dice
        let sum: u32 = final_rolls.iter().map(|roll| roll.value as u32).sum();
        
        log::trace!("Sum: {}, Target: {}", sum, self.target);
        
        // 6. If the sum is >= target, return true (win), else false
        sum >= self.target
    }
}
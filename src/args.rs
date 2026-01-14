use crate::dice::Die;
use clap::Parser;

/// Parse a dice specification string like "d6" or "2d10"
fn parse_dice(s: &str) -> Result<Vec<Die>, String> {
    let s = s.trim();

    // Check if it starts with a digit or 'd'
    if let Some(d_pos) = s.find('d') {
        let count_str = &s[..d_pos];
        let sides_str = &s[d_pos + 1..];

        let count = if count_str.is_empty() {
            1
        } else {
            count_str
                .parse::<u32>()
                .map_err(|_| format!("Invalid dice count: {}", count_str))?
        };

        let sides = sides_str
            .parse::<u8>()
            .map_err(|_| format!("Invalid dice sides: {}", sides_str))?;

        // Validate that it's a valid die type
        match sides {
            4 | 6 | 8 | 10 | 12 | 20 => Ok((0..count).map(|_| Die { sides }).collect()),
            _ => Err(format!(
                "Invalid die type: d{}. Valid types are d4, d6, d8, d10, d12, d20",
                sides
            )),
        }
    } else {
        Err(format!(
            "Invalid dice format: {}. Expected format like 'd6' or '2d10'",
            s
        ))
    }
}

/// Parse a comma-separated list of dice
fn parse_dice_list(s: &str) -> Result<Vec<Die>, String> {
    s.split(',')
        .map(|part| parse_dice(part.trim()))
        .collect::<Result<Vec<Vec<Die>>, String>>()
        .map(|vec_of_vecs| vec_of_vecs.into_iter().flatten().collect())
}

#[derive(Parser, Debug)]
#[command(name = "dicegame")]
#[command(about = "A dice rolling game", long_about = None)]
pub struct Args {
    /// Dice to roll (e.g., "d4,d6,d8,2d10,d12,d20")
    #[arg(default_value = "d4,d6,d8,2d10,d12,d20")]
    pub dice: String,

    /// Target number to roll
    #[arg(short, long, default_value_t = 19)]
    pub target: u32,

    /// Number of iterations to run
    #[arg(short = 'n', long, default_value_t = 100_000)]
    pub iterations: u32,
}

impl Args {
    pub fn parse_args() -> Self {
        let args = Args::parse();

        if let Ok(dice) = args.get_dice() {
            log::debug!("Rolling dice: {:?}", dice);
        }

        log::debug!("Target: {}", args.target);
        log::debug!("Iterations: {}", args.iterations);

        args
    }

    pub fn get_dice(&self) -> Result<Vec<Die>, String> {
        parse_dice_list(&self.dice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_die() {
        let result = parse_dice("d6");
        assert!(result.is_ok());
        let dice = result.unwrap();
        assert_eq!(dice.len(), 1);
        assert_eq!(dice[0].sides, 6);
    }

    #[test]
    fn test_parse_multiple_dice() {
        let result = parse_dice("2d10");
        assert!(result.is_ok());
        let dice = result.unwrap();
        assert_eq!(dice.len(), 2);
        assert_eq!(dice[0].sides, 10);
        assert_eq!(dice[1].sides, 10);
    }

    #[test]
    fn test_parse_dice_list() {
        let result = parse_dice_list("d4,d6,2d10");
        assert!(result.is_ok());
        let dice_list = result.unwrap();
        assert_eq!(dice_list.len(), 4); // d4, d6, and 2xd10
        assert_eq!(dice_list[0].sides, 4);
        assert_eq!(dice_list[1].sides, 6);
        assert_eq!(dice_list[2].sides, 10);
        assert_eq!(dice_list[3].sides, 10);
    }

    #[test]
    fn test_invalid_die_type() {
        let result = parse_dice("d7");
        assert!(result.is_err());
    }
}

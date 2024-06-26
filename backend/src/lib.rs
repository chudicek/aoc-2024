use itertools::Itertools;

// todo ~~load from config~~ would load from config :D
const EMPTY_CHAR: char = '.';

#[derive(Default, Debug)]
struct Context {
    line_sum: usize,
    current_sum: Option<usize>,
    should_flush: bool,
}

impl Context {
    fn is_activating(character: char) -> bool {
        !character.is_numeric() && character != EMPTY_CHAR
    }

    pub fn update(mut self, prev: char, analyzed: char, next: char) -> Self {
        if let Some(parsed_digit) = analyzed.to_digit(10) {
            self.should_flush |= Self::is_activating(prev) || Self::is_activating(next);

            self.current_sum = Some(self.current_sum.map_or(parsed_digit as usize, |old| {
                old * 10 + parsed_digit as usize
            }));
        } else {
            let currrent_activates = Self::is_activating(prev)
                || Self::is_activating(analyzed)
                || Self::is_activating(next);

            let number_to_flush = self.current_sum.take().unwrap_or(0);
            if self.should_flush || currrent_activates {
                self.line_sum += number_to_flush;
            }

            // notice the override, not `|=`
            self.should_flush = currrent_activates;
        }

        self
    }

    /// Get the sum of the current line.
    ///
    /// Should only be used after processing the whole line.
    ///
    /// Intentionally consumes self.
    pub fn get_sum(self) -> usize {
        self.line_sum
            + match self.should_flush {
                true => self.current_sum.unwrap_or(0),
                false => 0,
            }
    }
}

/// Counts the sum of active numbers in a line of a *map*.

/// # Arguments
/// * `previous_line` - A line above the `analyzed_line` in a *map*.
/// * `analyzed_line` - A line in a *map* to get the sum of active numbers in.
/// * `next_line` - A line below the `analyzed_line` in a *map*.

/// # Preconditions
/// * The sum of the lengths of the characters in `analyzed_line` must not overflow a `usize`.
/// * All three input strings (`previous_line`, `analyzed_line`, `next_line`) must have the same length.

/// # Returns
/// The number of characters in `analyzed_line` that differ from the corresponding characters
/// in either `previous_line` or `next_line`.
///
/// If the preconditions are not met, the return value is undefined.
///
/// # Examples
/// ```rust
/// use backend::line_sum_active;
///
/// let previous_line = "..*";
/// let analyzed_line = ".1.";
/// let next_line =     "...";
///
/// // The number `1` is activated by the `*` character in the upper line.
/// assert_eq!(line_sum_active(previous_line, analyzed_line, next_line), 1);
/// ```
pub fn line_sum_active(previous_line: &str, analyzed_line: &str, next_line: &str) -> usize {
    previous_line
        .chars()
        .zip(analyzed_line.chars())
        .zip(next_line.chars())
        .fold(Context::default(), |ctx, ((prev, analyzed), next)| {
            ctx.update(prev, analyzed, next)
        })
        .get_sum()
}

/// Counts the sum of active numbers in a *map*.
///
/// # Arguments
/// * `lines` - An iterator of lines in a *map*.
///
/// # Returns
/// The sum of active numbers in the *map*.
///
/// # Examples
/// ```rust
/// use backend::map_sum;
///
/// let smol_input = r#"467..257..
///     ...*......
///     ..35..633.
///     ......#...
///     617*......
///     .....+.13.
///     ..592.....
///     ......755.
///     ...$.*....
///     .664.598.."#
///     .lines()
///     .map(|s| s.trim().to_string()); // <- must trim
///
/// assert_eq!(map_sum(smol_input), 4361);
/// ```
pub fn map_sum<I>(mut lines: I) -> usize
where
    I: Iterator<Item = String>,
{
    let Some(fist_line) = lines.next() else {
        return 0;
    };

    let line_lenght = fist_line.chars().count();

    let padding = std::iter::repeat(EMPTY_CHAR)
        .take(line_lenght)
        .collect::<String>();

    let padded_input = std::iter::once(padding.clone())
        .chain(std::iter::once(fist_line))
        .chain(lines)
        .chain(std::iter::once(padding));

    padded_input
        .tuple_windows()
        .map(|(prev, analyzed, next)| line_sum_active(&prev, &analyzed, &next))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufRead};

    use super::*;

    // prevent the edge case where the activating character used in the test is the same as the one used in the implementation
    const SOME_ACTIVATING_CHAR: char = if EMPTY_CHAR == '*' { '-' } else { '*' };

    #[test]
    fn empty_sum_should_be_0() {
        let previous_line = "";
        let analyzed_line = "";
        let next_line = "";

        assert_eq!(line_sum_active(previous_line, analyzed_line, next_line), 0);
    }

    #[test]
    fn inactive_numbers_should_be_ignored() {
        let previous_line = EMPTY_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = EMPTY_CHAR.to_string();

        assert_eq!(
            line_sum_active(&previous_line, analyzed_line, &next_line),
            0
        );
    }

    #[test]
    fn top_activating_should_be_counted() {
        let previous_line = EMPTY_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = SOME_ACTIVATING_CHAR.to_string();

        assert_eq!(
            line_sum_active(&previous_line, analyzed_line, &next_line),
            1
        );
    }

    #[test]
    fn bottom_activating_should_be_counted() {
        let previous_line = SOME_ACTIVATING_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = EMPTY_CHAR.to_string();

        assert_eq!(
            line_sum_active(&previous_line, analyzed_line, &next_line),
            1
        );
    }

    #[test]
    fn top_and_bottom_activating_should_be_counted() {
        let previous_line = SOME_ACTIVATING_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = SOME_ACTIVATING_CHAR.to_string();

        assert_eq!(
            line_sum_active(&previous_line, analyzed_line, &next_line),
            1
        );
    }

    #[test]
    fn diagonal_activating_should_be_counted() {
        {
            let previous_line = format!("{}{}{}", SOME_ACTIVATING_CHAR, EMPTY_CHAR, EMPTY_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);

            assert_eq!(
                line_sum_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }

        {
            let previous_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, SOME_ACTIVATING_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);

            assert_eq!(
                line_sum_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }

        {
            let previous_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", SOME_ACTIVATING_CHAR, EMPTY_CHAR, EMPTY_CHAR);

            assert_eq!(
                line_sum_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }

        {
            let previous_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, SOME_ACTIVATING_CHAR);

            assert_eq!(
                line_sum_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }
    }

    #[test]
    fn activating_character_besides_is_registered() {
        {
            let empty_previous_line = std::iter::repeat(EMPTY_CHAR).take(3).collect::<String>();
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", SOME_ACTIVATING_CHAR);
            let empty_next_line = std::iter::repeat(EMPTY_CHAR).take(3).collect::<String>();

            assert_eq!(
                line_sum_active(&empty_previous_line, &analyzed_line, &empty_next_line),
                1
            );
        }

        {
            let empty_previous_line = std::iter::repeat(EMPTY_CHAR).take(3).collect::<String>();
            let analyzed_line = format!("{}{}{}", SOME_ACTIVATING_CHAR, "1", EMPTY_CHAR);
            let empty_next_line = std::iter::repeat(EMPTY_CHAR).take(3).collect::<String>();

            assert_eq!(
                line_sum_active(&empty_previous_line, &analyzed_line, &empty_next_line),
                1
            );
        }
    }

    const DATASET_DIR: &str = "dataset";
    const EXAMPLE0: &str = "example0";

    #[test]
    fn test_map_sum_larger() {
        let mut dataset_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dataset_path = dataset_path.parent().unwrap().to_path_buf();
        dataset_path.push(DATASET_DIR);
        dataset_path.push(EXAMPLE0);

        let dataset_file = std::fs::File::open(dataset_path).unwrap();

        let lines = io::BufReader::new(dataset_file)
            .lines()
            .map(|line| line.unwrap());

        assert_eq!(map_sum(lines), 557705); // hopefully lol
    }
}

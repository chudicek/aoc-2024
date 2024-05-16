#![allow(dead_code)] // todo remove

// todo load from config
const EMPTY_CHAR: char = '.';

#[derive(Default, Debug)]
struct Context {
    line_sum: usize,
    current_sum: Option<usize>,
    should_flush: bool,
}

impl Context {
    fn new() -> Self {
        Self {
            line_sum: 0,
            current_sum: None,
            should_flush: false,
        }
    }

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
            let currrent_activates = Self::is_activating(prev) || Self::is_activating(next);

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
/// let previous_line = "..*";
/// let analyzed_line = ".1.";
/// let next_line =     "...";
///
/// // The number `1` is activated by the `*` character in the upper line.
/// assert_eq!(line_count_active(previous_line, analyzed_line, next_line), 1);
/// ```
pub fn line_count_active(previous_line: &str, analyzed_line: &str, next_line: &str) -> usize {
    previous_line
        .chars()
        .zip(analyzed_line.chars())
        .zip(next_line.chars())
        .fold(Context::default(), |ctx, ((prev, analyzed), next)| {
            ctx.update(prev, analyzed, next)
        })
        .get_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // prevent the edge case where the activating character used in the test is the same as the one used in the implementation
    const SOME_ACTIVATING_CHAR: char = if EMPTY_CHAR == '*' { '-' } else { '*' };

    #[test]
    fn empty_sum_should_be_0() {
        let previous_line = "";
        let analyzed_line = "";
        let next_line = "";

        assert_eq!(
            line_count_active(previous_line, analyzed_line, next_line),
            0
        );
    }

    #[test]
    fn inactive_numbers_should_be_ignored() {
        let previous_line = EMPTY_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = EMPTY_CHAR.to_string();

        assert_eq!(
            line_count_active(&previous_line, analyzed_line, &next_line),
            0
        );
    }

    #[test]
    fn top_activating_should_be_counted() {
        let previous_line = EMPTY_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = SOME_ACTIVATING_CHAR.to_string();

        assert_eq!(
            line_count_active(&previous_line, analyzed_line, &next_line),
            1
        );
    }

    #[test]
    fn bottom_activating_should_be_counted() {
        let previous_line = SOME_ACTIVATING_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = EMPTY_CHAR.to_string();

        assert_eq!(
            line_count_active(&previous_line, analyzed_line, &next_line),
            1
        );
    }

    #[test]
    fn top_and_bottom_activating_should_be_counted() {
        let previous_line = SOME_ACTIVATING_CHAR.to_string();
        let analyzed_line = "1";
        let next_line = SOME_ACTIVATING_CHAR.to_string();

        assert_eq!(
            line_count_active(&previous_line, analyzed_line, &next_line),
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
                line_count_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }

        {
            let previous_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, SOME_ACTIVATING_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);

            assert_eq!(
                line_count_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }

        {
            let previous_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", SOME_ACTIVATING_CHAR, EMPTY_CHAR, EMPTY_CHAR);

            assert_eq!(
                line_count_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }

        {
            let previous_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, EMPTY_CHAR);
            let analyzed_line = format!("{}{}{}", EMPTY_CHAR, "1", EMPTY_CHAR);
            let next_line = format!("{}{}{}", EMPTY_CHAR, EMPTY_CHAR, SOME_ACTIVATING_CHAR);

            assert_eq!(
                line_count_active(&previous_line, &analyzed_line, &next_line),
                1
            );
        }
    }

    #[test]
    fn test_single_activating() {
        let previous_line = "..*";
        let analyzed_line = ".1.";
        let next_line = "...";

        assert_eq!(
            line_count_active(previous_line, analyzed_line, next_line),
            1
        );
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_skip_history_true() {
        let mut options = PublishOptions {
            skip_history: false,
        };
        let option = with_skip_history(true);
        option(&mut options);
        assert_eq!(options.skip_history, true);
    }

    #[test]
    fn test_with_skip_history_false() {
        let mut options = PublishOptions { skip_history: true };
        let option = with_skip_history(false);
        option(&mut options);
        assert_eq!(options.skip_history, false);
    }

    #[test]
    fn test_with_skip_history_multiple_calls() {
        let mut options = PublishOptions {
            skip_history: false,
        };
        let option1 = with_skip_history(true);
        let option2 = with_skip_history(false);
        let option3 = with_skip_history(true);

        option1(&mut options);
        assert_eq!(options.skip_history, true);

        option2(&mut options);
        assert_eq!(options.skip_history, false);

        option3(&mut options);
        assert_eq!(options.skip_history, true);
    }

    #[test]
    fn test_with_skip_history_no_error_with_default_options() {
        let mut options = PublishOptions {
            skip_history: false,
        };
        let option = with_skip_history(true);

        // This should not panic or throw an error
        option(&mut options);

        assert_eq!(options.skip_history, true);
    }
}

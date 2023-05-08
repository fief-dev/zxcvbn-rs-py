extern crate zxcvbn;

use pyo3::prelude::*;
use std::fmt;

#[pyclass]
#[pyo3(module = "zxcvbn")]
#[derive(Clone)]
enum Warning {
    StraightRowsOfKeysAreEasyToGuess,
    ShortKeyboardPatternsAreEasyToGuess,
    RepeatsLikeAaaAreEasyToGuess,
    RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess,
    ThisIsATop10Password,
    ThisIsATop100Password,
    ThisIsACommonPassword,
    ThisIsSimilarToACommonlyUsedPassword,
    SequencesLikeAbcAreEasyToGuess,
    RecentYearsAreEasyToGuess,
    AWordByItselfIsEasyToGuess,
    DatesAreOftenEasyToGuess,
    NamesAndSurnamesByThemselvesAreEasyToGuess,
    CommonNamesAndSurnamesAreEasyToGuess,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::StraightRowsOfKeysAreEasyToGuess => {
                write!(f, "Straight rows of keys are easy to guess.")
            }
            Warning::ShortKeyboardPatternsAreEasyToGuess => {
                write!(f, "Short keyboard patterns are easy to guess.")
            }
            Warning::RepeatsLikeAaaAreEasyToGuess => {
                write!(f, "Repeats like \"aaa\" are easy to guess.")
            }
            Warning::RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess => write!(
                f,
                "Repeats like \"abcabcabc\" are only slightly harder to guess than \"abc\"."
            ),
            Warning::ThisIsATop10Password => write!(f, "This is a top-10 common password."),
            Warning::ThisIsATop100Password => write!(f, "This is a top-100 common password."),
            Warning::ThisIsACommonPassword => write!(f, "This is a very common password."),
            Warning::ThisIsSimilarToACommonlyUsedPassword => {
                write!(f, "This is similar to a commonly used password.")
            }
            Warning::SequencesLikeAbcAreEasyToGuess => {
                write!(f, "Sequences like abc or 6543 are easy to guess.")
            }
            Warning::RecentYearsAreEasyToGuess => write!(f, "Recent years are easy to guess."),
            Warning::AWordByItselfIsEasyToGuess => write!(f, "A word by itself is easy to guess."),
            Warning::DatesAreOftenEasyToGuess => write!(f, "Dates are often easy to guess."),
            Warning::NamesAndSurnamesByThemselvesAreEasyToGuess => {
                write!(f, "Names and surnames by themselves are easy to guess.")
            }
            Warning::CommonNamesAndSurnamesAreEasyToGuess => {
                write!(f, "Common names and surnames are easy to guess.")
            }
        }
    }
}

#[pymethods]
impl Warning {
    fn __str__(&self) -> PyResult<String> {
        return Ok(format!("{self}"));
    }
}

fn match_warning(warning: zxcvbn::feedback::Warning) -> Warning {
    match warning {
        zxcvbn::feedback::Warning::StraightRowsOfKeysAreEasyToGuess => {
            Warning::StraightRowsOfKeysAreEasyToGuess
        }
        zxcvbn::feedback::Warning::ShortKeyboardPatternsAreEasyToGuess => {
            Warning::ShortKeyboardPatternsAreEasyToGuess
        }
        zxcvbn::feedback::Warning::RepeatsLikeAaaAreEasyToGuess => {
            Warning::RepeatsLikeAaaAreEasyToGuess
        }
        zxcvbn::feedback::Warning::RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess => {
            Warning::RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess
        }
        zxcvbn::feedback::Warning::ThisIsATop10Password => Warning::ThisIsATop10Password,
        zxcvbn::feedback::Warning::ThisIsATop100Password => Warning::ThisIsATop100Password,
        zxcvbn::feedback::Warning::ThisIsACommonPassword => Warning::ThisIsACommonPassword,
        zxcvbn::feedback::Warning::ThisIsSimilarToACommonlyUsedPassword => {
            Warning::ThisIsSimilarToACommonlyUsedPassword
        }
        zxcvbn::feedback::Warning::SequencesLikeAbcAreEasyToGuess => {
            Warning::SequencesLikeAbcAreEasyToGuess
        }
        zxcvbn::feedback::Warning::RecentYearsAreEasyToGuess => Warning::RecentYearsAreEasyToGuess,
        zxcvbn::feedback::Warning::AWordByItselfIsEasyToGuess => {
            Warning::AWordByItselfIsEasyToGuess
        }
        zxcvbn::feedback::Warning::DatesAreOftenEasyToGuess => Warning::DatesAreOftenEasyToGuess,
        zxcvbn::feedback::Warning::NamesAndSurnamesByThemselvesAreEasyToGuess => {
            Warning::NamesAndSurnamesByThemselvesAreEasyToGuess
        }
        zxcvbn::feedback::Warning::CommonNamesAndSurnamesAreEasyToGuess => {
            Warning::CommonNamesAndSurnamesAreEasyToGuess
        }
    }
}

#[pyclass]
#[pyo3(module = "zxcvbn")]
#[derive(Clone)]
enum Suggestion {
    UseAFewWordsAvoidCommonPhrases,
    NoNeedForSymbolsDigitsOrUppercaseLetters,
    AddAnotherWordOrTwo,
    CapitalizationDoesntHelpVeryMuch,
    AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase,
    ReversedWordsArentMuchHarderToGuess,
    PredictableSubstitutionsDontHelpVeryMuch,
    UseALongerKeyboardPatternWithMoreTurns,
    AvoidRepeatedWordsAndCharacters,
    AvoidSequences,
    AvoidRecentYears,
    AvoidYearsThatAreAssociatedWithYou,
    AvoidDatesAndYearsThatAreAssociatedWithYou,
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suggestion::UseAFewWordsAvoidCommonPhrases => {
                write!(f, "Use a few words, avoid common phrases.")
            }
            Suggestion::NoNeedForSymbolsDigitsOrUppercaseLetters => {
                write!(f, "No need for symbols, digits, or uppercase letters.")
            }
            Suggestion::AddAnotherWordOrTwo => {
                write!(f, "Add another word or two. Uncommon words are better.")
            }
            Suggestion::CapitalizationDoesntHelpVeryMuch => {
                write!(f, "Capitalization doesn't help very much.")
            }
            Suggestion::AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase => write!(
                f,
                "All-uppercase is almost as easy to guess as all-lowercase."
            ),
            Suggestion::ReversedWordsArentMuchHarderToGuess => {
                write!(f, "Reversed words aren't much harder to guess.")
            }
            Suggestion::PredictableSubstitutionsDontHelpVeryMuch => write!(
                f,
                "Predictable substitutions like '@' instead of 'a' don't help very much."
            ),
            Suggestion::UseALongerKeyboardPatternWithMoreTurns => {
                write!(f, "Use a longer keyboard pattern with more turns.")
            }
            Suggestion::AvoidRepeatedWordsAndCharacters => {
                write!(f, "Avoid repeated words and characters.")
            }
            Suggestion::AvoidSequences => write!(f, "Avoid sequences."),
            Suggestion::AvoidRecentYears => write!(f, "Avoid recent years."),
            Suggestion::AvoidYearsThatAreAssociatedWithYou => {
                write!(f, "Avoid years that are associated with you.")
            }
            Suggestion::AvoidDatesAndYearsThatAreAssociatedWithYou => {
                write!(f, "Avoid dates and years that are associated with you.")
            }
        }
    }
}

#[pymethods]
impl Suggestion {
    fn __str__(&self) -> PyResult<String> {
        return Ok(format!("{self}"));
    }
}

fn match_suggestion(suggestion: zxcvbn::feedback::Suggestion) -> Suggestion {
    match suggestion {
        zxcvbn::feedback::Suggestion::UseAFewWordsAvoidCommonPhrases => {
            Suggestion::UseAFewWordsAvoidCommonPhrases
        }
        zxcvbn::feedback::Suggestion::NoNeedForSymbolsDigitsOrUppercaseLetters => {
            Suggestion::NoNeedForSymbolsDigitsOrUppercaseLetters
        }
        zxcvbn::feedback::Suggestion::AddAnotherWordOrTwo => Suggestion::AddAnotherWordOrTwo,
        zxcvbn::feedback::Suggestion::CapitalizationDoesntHelpVeryMuch => {
            Suggestion::CapitalizationDoesntHelpVeryMuch
        }
        zxcvbn::feedback::Suggestion::AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase => {
            Suggestion::AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase
        }
        zxcvbn::feedback::Suggestion::ReversedWordsArentMuchHarderToGuess => {
            Suggestion::ReversedWordsArentMuchHarderToGuess
        }
        zxcvbn::feedback::Suggestion::PredictableSubstitutionsDontHelpVeryMuch => {
            Suggestion::PredictableSubstitutionsDontHelpVeryMuch
        }
        zxcvbn::feedback::Suggestion::UseALongerKeyboardPatternWithMoreTurns => {
            Suggestion::UseALongerKeyboardPatternWithMoreTurns
        }
        zxcvbn::feedback::Suggestion::AvoidRepeatedWordsAndCharacters => {
            Suggestion::AvoidRepeatedWordsAndCharacters
        }
        zxcvbn::feedback::Suggestion::AvoidSequences => Suggestion::AvoidSequences,
        zxcvbn::feedback::Suggestion::AvoidRecentYears => Suggestion::AvoidRecentYears,
        zxcvbn::feedback::Suggestion::AvoidYearsThatAreAssociatedWithYou => {
            Suggestion::AvoidYearsThatAreAssociatedWithYou
        }
        zxcvbn::feedback::Suggestion::AvoidDatesAndYearsThatAreAssociatedWithYou => {
            Suggestion::AvoidDatesAndYearsThatAreAssociatedWithYou
        }
    }
}

#[pyclass]
#[pyo3(module = "zxcvbn")]
#[derive(Clone)]
struct Feedback {
    #[pyo3(get)]
    warning: Option<Warning>,
    #[pyo3(get)]
    suggestions: Vec<Suggestion>,
}

#[pyclass]
#[pyo3(module = "zxcvbn")]
struct Entropy {
    /// Estimated guesses needed to crack the password
    #[pyo3(get)]
    guesses: u64,

    /// Order of magnitude of `guesses`
    #[pyo3(get)]
    guesses_log10: f64,

    // /// List of back-of-the-envelope crack time estimations based on a few scenarios.
    // crack_times: time_estimates::CrackTimes,
    /// Overall strength score from 0-4.
    /// Any score less than 3 should be considered too weak.
    #[pyo3(get)]
    score: u8,

    /// Verbal feedback to help choose better passwords. Set when `score` <= 2.
    #[pyo3(get)]
    feedback: Option<Feedback>,

    /// The list of patterns the guess calculation was based on
    // sequence: Vec<Match>,

    /// How long it took to calculate the answer.
    #[pyo3(get)]
    calc_time: u128,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(name = "zxcvbn")]
fn zxcvbn_py(password: &str) -> PyResult<Entropy> {
    let estimate = zxcvbn::zxcvbn(password, &[]).unwrap();
    let feedback: Option<Feedback> = match estimate.feedback() {
        None => None,
        Some(f) => Some(Feedback {
            warning: match f.warning() {
                None => None,
                Some(w) => Some(match_warning(w)),
            },
            suggestions: f
                .suggestions()
                .iter()
                .map(|s| match_suggestion(*s))
                .collect::<Vec<Suggestion>>()
                .to_vec(),
        }),
    };
    return Ok(Entropy {
        guesses: estimate.guesses(),
        guesses_log10: estimate.guesses_log10(),
        score: estimate.score(),
        feedback: feedback,
        calc_time: estimate.calculation_time().as_millis(),
    });
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "zxcvbn")]
fn zxcvbn_rs_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Entropy>()?;
    m.add_class::<Warning>()?;
    m.add_class::<Suggestion>()?;
    m.add_class::<Feedback>()?;
    m.add_function(wrap_pyfunction!(zxcvbn_py, m)?)?;
    Ok(())
}

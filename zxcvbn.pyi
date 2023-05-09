from enum import Enum, auto

class Warning(Enum):
    StraightRowsOfKeysAreEasyToGuess = auto()
    ShortKeyboardPatternsAreEasyToGuess = auto()
    RepeatsLikeAaaAreEasyToGuess = auto()
    RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess = auto()
    ThisIsATop10Password = auto()
    ThisIsATop100Password = auto()
    ThisIsACommonPassword = auto()
    ThisIsSimilarToACommonlyUsedPassword = auto()
    SequencesLikeAbcAreEasyToGuess = auto()
    RecentYearsAreEasyToGuess = auto()
    AWordByItselfIsEasyToGuess = auto()
    DatesAreOftenEasyToGuess = auto()
    NamesAndSurnamesByThemselvesAreEasyToGuess = auto()
    CommonNamesAndSurnamesAreEasyToGuess = auto()

class Suggestion(Enum):
    UseAFewWordsAvoidCommonPhrases = auto()
    NoNeedForSymbolsDigitsOrUppercaseLetters = auto()
    AddAnotherWordOrTwo = auto()
    CapitalizationDoesntHelpVeryMuch = auto()
    AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase = auto()
    ReversedWordsArentMuchHarderToGuess = auto()
    PredictableSubstitutionsDontHelpVeryMuch = auto()
    UseALongerKeyboardPatternWithMoreTurns = auto()
    AvoidRepeatedWordsAndCharacters = auto()
    AvoidSequences = auto()
    AvoidRecentYears = auto()
    AvoidYearsThatAreAssociatedWithYou = auto()
    AvoidDatesAndYearsThatAreAssociatedWithYou = auto()

class Feedback:
    warning: Warning | None
    suggestions: list[Suggestion]

class Entropy:
    guesses: int
    guesses_log10: float
    score: int
    feedback: Feedback | None
    calc_time: int

def zxcvbn(password: str, user_inputs: list[str] | None = None) -> Entropy: ...

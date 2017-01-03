use std::error::Error;
use std::fmt;

/// Error kind of parsing.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParseErrorKind {
    /// Unable to parse year.
    InvalidYear,
    /// Unable to parse month.
    InvalidMonth,
    /// Unable to parse day.
    InvalidDay,
    /// Unable to parse hour.
    InvalidHour,
    /// Unable to parse minutes.
    InvalidMinute,
    /// Unable to parse seconds.
    InvalidSeconds,
    /// Unable to parse nanoseconds.
    InvalidNanoseconds,
    /// Invalid format.
    InvalidFormat,
    /// Unexpected token.
    InvalidToken,
    /// Invalid value range. Value is too low.
    InvalidLowValue,
    /// Invalid value range. Value is too high.
    InvalidHighValue,
    /// Date is not exists.
    InvalidDate,
    /// Time is not exists.
    InvalidTime,
    /// Date is parsed, but there is some text after date.
    StringNotEnded,
}

impl Error for ParseErrorKind {
    fn description(&self) -> &str {
        match self {
            &ParseErrorKind::InvalidYear => "Unable to parse year.",
            &ParseErrorKind::InvalidMonth => "Unable to parse month.",
            &ParseErrorKind::InvalidDay => "Unable to parse day.",
            &ParseErrorKind::InvalidHour => "Unable to parse hour.",
            &ParseErrorKind::InvalidMinute => "Unable to parse minutes.",
            &ParseErrorKind::InvalidSeconds => "Unable to parse seconds.",
            &ParseErrorKind::InvalidNanoseconds => "Unable to parse nanoseconds.",
            &ParseErrorKind::InvalidFormat => "Invalid format.",
            &ParseErrorKind::InvalidToken => "Unexpected token.",
            &ParseErrorKind::InvalidLowValue => "Invalid value range. Value is too low.",
            &ParseErrorKind::InvalidHighValue => "Invalid value range. Value is too high.",
            &ParseErrorKind::InvalidDate => "Date is not exists.",
            &ParseErrorKind::InvalidTime => "Time is not exists.",
            &ParseErrorKind::StringNotEnded => "Date is parsed, but there is some text after date.",
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseErrorKind::InvalidYear => write!(f,"Unable to parse year."),
            &ParseErrorKind::InvalidMonth => write!(f,"Unable to parse month."),
            &ParseErrorKind::InvalidDay => write!(f,"Unable to parse day."),
            &ParseErrorKind::InvalidHour => write!(f,"Unable to parse hour."),
            &ParseErrorKind::InvalidMinute => write!(f,"Unable to parse minutes."),
            &ParseErrorKind::InvalidSeconds => write!(f,"Unable to parse seconds."),
            &ParseErrorKind::InvalidNanoseconds => write!(f,"Unable to parse nanoseconds."),
            &ParseErrorKind::InvalidFormat => write!(f,"Invalid write."),
            &ParseErrorKind::InvalidToken => write!(f,"Unexpected token."),
            &ParseErrorKind::InvalidLowValue => write!(f,"Invalid value range. Value is too low."),
            &ParseErrorKind::InvalidHighValue => write!(f,"Invalid value range. Value is too high."),
            &ParseErrorKind::InvalidDate => write!(f,"Date is not exists."),
            &ParseErrorKind::InvalidTime => write!(f,"Time is not exists."),
            &ParseErrorKind::StringNotEnded => write!(f,"Date is parsed, but there is some text after date."),
        }
    }
}

/// An error from the parse_* functions.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ParseError {
    /// Kind of parse error.
    pub error_kind: ParseErrorKind,
    /// Start of error position.
    pub position_begin: usize,
    /// End of error position.
    pub position_end: usize,
}
impl ParseError {
    /// Returns error.
    pub fn invalid(error_kind: ParseErrorKind,position: usize,length: usize) -> ParseError {
        ParseError {
            position_begin: position,
            position_end: position + length,
            error_kind: error_kind,
        }
    }
    /// Returns error due the invalid format.
    pub fn invalid_format(position: usize,length: usize) -> ParseError {
        ParseError {
            position_begin: position,
            position_end: position + length,
            error_kind: ParseErrorKind::InvalidFormat,
        }
    }
    /// Returns error due the invalid token.
    pub fn invalid_token(position: usize,length: usize) -> ParseError {
        ParseError {
            position_begin: position,
            position_end: position + length,
            error_kind: ParseErrorKind::InvalidToken,
        }
    }
    /// Returns error due the value is too low.
    pub fn invalid_low_value(position: usize,length: usize) -> ParseError {
        ParseError {
            position_begin: position,
            position_end: position + length,
            error_kind: ParseErrorKind::InvalidLowValue,
        }
    }
    /// Returns error due the value is too high.
    pub fn invalid_high_value(position: usize,length: usize) -> ParseError {
        ParseError {
            position_begin: position,
            position_end: position + length,
            error_kind: ParseErrorKind::InvalidHighValue,
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        return self.error_kind.description();
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.error_kind.fmt(f);
    }
}

/// Same to `Result<T, ParseError>`.
pub type ParseResult<T> = Result<T, ParseError>;
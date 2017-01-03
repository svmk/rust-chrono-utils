use chrono::offset::fixed::FixedOffset;
use super::error::{ParseErrorKind,ParseError,ParseResult};
pub fn parse_i32(str: &str,position: &mut usize,length: usize,error_kind: ParseErrorKind) -> ParseResult<i32> {
    if str.len() >= *position + length {
        let text = &str[*position..*position+length];
        if let Ok(value) = text.parse::<i32>() {
            *position = *position + length;
            return Ok(value);
        }        
    }
    return Err(ParseError::invalid(error_kind,position.clone(),length));
}
pub fn parse_u32(str: &str,position: &mut usize,length: usize,error_kind: ParseErrorKind) -> ParseResult<u32> {
    if str.len() >= *position + length {
        let text = &str[*position..*position+length];
        if let Ok(value) = text.parse::<u32>() {
            *position = *position + length;
            return Ok(value);
        }
    }
    return Err(ParseError::invalid(error_kind,position.clone(),length));
}
pub fn parse_full_year(str: &str,position: &mut usize) ->  ParseResult<i32> {
    return parse_i32(str,position,4,ParseErrorKind::InvalidYear);
}
pub fn validate_range(result: ParseResult<u32>,min: u32,max: u32,position: &usize,length: usize) -> ParseResult<u32> {
    if let Ok(value) = result {
        if value < min {
            return Err(ParseError::invalid_low_value(position.clone(),length));
        }
        if value > max {
            return Err(ParseError::invalid_high_value(position.clone(),length));   
        }
    }
    return result;
}
pub fn parse_month_number(str: &str,position: &mut usize) ->  ParseResult<u32> {
    let result = parse_u32(str,position,2,ParseErrorKind::InvalidMonth);
    return validate_range(result,1,12,position,2);
}
pub fn parse_day_number(str: &str,position: &mut usize) ->  ParseResult<u32> {
    let result = parse_u32(str,position,2,ParseErrorKind::InvalidDay);
    return validate_range(result,1,31,position,2);
}
pub fn parse_hour_24(str: &str,position: &mut usize) ->  ParseResult<u32> {
    let result = parse_u32(str,position,2,ParseErrorKind::InvalidHour);
    return validate_range(result,0,23,position,2);
}
pub fn parse_hour_timezone(str: &str,position: &mut usize) ->  ParseResult<u32> {
    let result = parse_u32(str,position,2,ParseErrorKind::InvalidHour);
    return validate_range(result,0,12,position,2);
}
pub fn parse_minute(str: &str,position: &mut usize) ->  ParseResult<u32> {
    let result = parse_u32(str,position,2,ParseErrorKind::InvalidMinute);
    let result = validate_range(result,0,59,position,2);
    return result;
}
pub fn parse_seconds(str: &str,position: &mut usize) ->  ParseResult<u32> {
    let result = parse_u32(str,position,2,ParseErrorKind::InvalidSeconds);
    return validate_range(result,0,59,position,2);
}
pub fn parse_nanosecond(str: &str,position: &mut usize) -> ParseResult<u32> {
    let mut length = 0;
    if str.len() >= *position {
        let chars = str[*position..].chars();
        for c in chars {
            if !c.is_digit(10) {
                break;
            }
            length = length + c.len_utf8();
        }
        if length > 0 && length <= 9 {
            let text = &str[*position..*position+length];
            if let Ok(value) = text.parse::<u32>() {
                *position = *position + length;
                let pow = 10u32.pow(9 - length as u32);
                let value = value * pow;
                return Ok(value);
            }
        }
    }
    return Err(ParseError::invalid(ParseErrorKind::InvalidNanoseconds,position.clone(),length));   
}
pub fn parse_tzd(str: &str,position: &mut usize) ->  ParseResult<FixedOffset> {
    if try!(parse_is_token(str,position,"Z")) {
        return Ok(FixedOffset::east(0));
    }
    let is_positive = try!(parse_is_token(str,position,"+"));
    let is_negative = try!(parse_is_token(str,position,"-"));
    if is_positive || is_negative {
        let hour = try!(parse_hour_timezone(str,position));
        let _ = try!(parse_token(str,position,":"));
        let minute = try!(parse_minute(str,position));
        let offset = (hour * 60 * 60 + minute * 60) as i32;
        if is_negative {
            return Ok(FixedOffset::west(offset));
        } else {
            return Ok(FixedOffset::east(offset));
        }
    }
    return Err(ParseError::invalid_token(position.clone(),1));
}
pub fn parse_token(str:&str,position: &mut usize,token: &str) -> ParseResult<()> {
    let length = token.len();
    if str.len() >= *position + length {
        let token_str = &str[*position..*position+length];
        if token_str == token {
            *position = *position + length;
            return Ok(());
        }
    }
    return Err(ParseError::invalid_token(position.clone(),length));
}
pub fn parse_token_or_end(str:&str,position: &mut usize,token: &str) -> ParseResult<bool> {
    let length = token.len();
    if str.len() >= *position + length {
        let token_str = &str[*position..*position+length];
        if token_str == token {
            *position = *position + length;
            return Ok(true);
        } else {
            return Err(ParseError::invalid_token(position.clone(),length));
        }
    }
    return Ok(false);
}
pub fn parse_is_token(str:&str,position: &mut usize,token: &str) -> ParseResult<bool> {
    let length = token.len();
    if str.len() >= *position + length {
        let token_str = &str[*position..*position+length];
        if token_str == token {
            *position = *position + length;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
    return Err(ParseError::invalid_token(position.clone(),length));
}
pub fn parse_end_of_string(str: &str,position: &usize) -> ParseResult<()> {
    if str.len() == *position {
        return Ok(());
    }
    return Err(ParseError::invalid(ParseErrorKind::StringNotEnded,position.clone(),0));
}
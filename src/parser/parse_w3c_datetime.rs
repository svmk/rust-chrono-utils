use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use chrono::naive::date::NaiveDate;
use chrono::naive::time::NaiveTime;
use chrono::offset::Offset;
use super::helper::*;
use super::error::*;
/// Parses an W3C date and time string then returns a new `DateTime` with a parsed `FixedOffset`.
///
/// W3C note: https://www.w3.org/TR/NOTE-datetime
///
/// Valid formats: `YYYY-MM-DD`, 
/// `YYYY-MM-DDThh:mmTZD`, 
/// `YYYY-MM-DDThh:mm:ssTZD`, 
/// `YYYY-MM-DDThh:mm:ss.sTZD`
///
/// Invalid formats: 
/// `YYYY`,
/// `YYYY-MM`
pub fn parse_w3c_datetime(str: &str) ->  ParseResult<DateTime<FixedOffset>> {
    // https://www.w3.org/TR/NOTE-datetime
    // Year:
    //   YYYY (eg 1997)
    // Year and month:
    //   YYYY-MM (eg 1997-07)
    // Complete date:
    //   YYYY-MM-DD (eg 1997-07-16)
    // Complete date plus hours and minutes:
    //   YYYY-MM-DDThh:mmTZD (eg 1997-07-16T19:20+01:00)
    // Complete date plus hours, minutes and seconds:
    //   YYYY-MM-DDThh:mm:ssTZD (eg 1997-07-16T19:20:30+01:00)
    // Complete date plus hours, minutes, seconds and a decimal fraction of a second
    //   YYYY-MM-DDThh:mm:ss.sTZD (eg 1997-07-16T19:20:30.45+01:00)
    // 
    // where:
    // YYYY = four-digit year
    // MM   = two-digit month (01=January, etc.)
    // DD   = two-digit day of month (01 through 31)
    // hh   = two digits of hour (00 through 23) (am/pm NOT allowed)
    // mm   = two digits of minute (00 through 59)
    // ss   = two digits of second (00 through 59)
    // s    = one or more digits representing a decimal fraction of a second
    // TZD  = time zone designator (Z or +hh:mm or -hh:mm)
    let mut position = 0;
    let year = try!(parse_full_year(str,&mut position));
    let _ = try!(parse_token(str,&mut position,"-"));
    let month = try!(parse_month_number(str,&mut position));
    let _ = try!(parse_token(str,&mut position,"-"));
    let day = try!(parse_day_number(str,&mut position));   
    let mut hour = 0;
    let mut minute = 0;
    let mut seconds = 0;
    let mut nanosecond = 0;
    let mut offset = FixedOffset::east(0);
    if try!(parse_token_or_end(str,&mut position,"T")) {
        hour = try!(parse_hour_24(str,&mut position));
        let _ = try!(parse_token(str,&mut position,":"));
        minute = try!(parse_minute(str,&mut position));
        if try!(parse_is_token(str,&mut position,":")) {
            seconds = try!(parse_seconds(str,&mut position));
            if try!(parse_is_token(str,&mut position,".")) {
                nanosecond = try!(parse_nanosecond(str,&mut position));
            }
            offset = try!(parse_tzd(str,&mut position));
        } else {
            offset = try!(parse_tzd(str,&mut position));
        }        
    }
    let _ = try!(parse_end_of_string(str,&position));
    if let Some(date) = NaiveDate::from_ymd_opt(year,month,day) {
        if let Some(time) = NaiveTime::from_hms_nano_opt(hour, minute, seconds, nanosecond) {
            let naive_date_time = date.and_time(time);
            if let Some(naive_date_time) = naive_date_time.checked_sub(offset.local_minus_utc()) {
                return Ok(DateTime::from_utc(naive_date_time, offset));
            }
        }
    }
    return Err(ParseError::invalid_format(0,str.len()));
}
#[cfg(test)]
#[test]
fn test_w3c() {
    extern crate chrono;
    use formatter::format_w3c;
    // Test data - (input, Ok(expected result after parse and format) or Err(error code))
    let testdates = [
        ("2015-01-20", Ok("2015-01-20T00:00:00Z")),

        ("2015-01-20T17:35:20-08:00", Ok("2015-01-20T17:35:20-08:00")),
        ("1944-06-06T04:04:00Z", Ok("1944-06-06T04:04:00Z")),
        ("2001-09-11T09:45:00-08:00", Ok("2001-09-11T09:45:00-08:00")),
        ("2015-01-20T17:35:20.001-08:00", Ok("2015-01-20T17:35:20.001-08:00")),
        ("2015-01-20T17:35:20.000031-08:00", Ok("2015-01-20T17:35:20.000031-08:00")),
        ("2015-01-20T17:35:20.000000004-08:00", Ok("2015-01-20T17:35:20.000000004-08:00")),
        ("2015-01-20T17:35:20.000000000452-08:00", Err(ParseErrorKind::InvalidNanoseconds)),
        ("2015-02-30T17:35:20-08:00", Err(ParseErrorKind::InvalidFormat)),               // bad day of month
        ("2015-01-20T25:35:20-08:00", Err(ParseErrorKind::InvalidHighValue)),               // bad hour
        ("2015-01-20T17:65:20-08:00", Err(ParseErrorKind::InvalidHighValue)),               // bad minute
        ("2015-01-20T17:35:90-08:00", Err(ParseErrorKind::InvalidHighValue)),               // bad second
        ("2015-01-20T17:35:20-24:00", Err(ParseErrorKind::InvalidHighValue)),               // bad offset
        ("2015", Err(ParseErrorKind::InvalidToken)),
        ("2015-", Err(ParseErrorKind::InvalidMonth)),
        ("2015-03", Err(ParseErrorKind::InvalidToken)),
        ("2015-03-", Err(ParseErrorKind::InvalidDay)),
        ("2015-03-04", Ok("2015-03-04T00:00:00Z")),
        ("2015-03-04T", Err(ParseErrorKind::InvalidHour)),
        ("2015-03-04T15", Err(ParseErrorKind::InvalidToken)),
        ("2015-03-04T15:", Err(ParseErrorKind::InvalidMinute)),
        ("2015-03-04T15:34", Err(ParseErrorKind::InvalidToken)),
        ("2015-03-04T15:34:45", Err(ParseErrorKind::InvalidToken)),
        ("2015-03-04T15:34:", Err(ParseErrorKind::InvalidSeconds)),
        ("2015-03-04T15:34:45Z", Ok("2015-03-04T15:34:45Z")),
        ("2015-03-04T15:34:45.008", Err(ParseErrorKind::InvalidToken)),
        ("2015-03-04T15:34:45", Err(ParseErrorKind::InvalidToken)),
        ("2015-03-04T15:34:45.008Z", Ok("2015-03-04T15:34:45.008Z")),
        ("2015-03-04T15:34:45.008+05:00", Ok("2015-03-04T15:34:45.008+05:00")),
        ("2015-03-04Z", Err(ParseErrorKind::InvalidToken)),
        ("2015-3-04", Err(ParseErrorKind::InvalidMonth)),
        ("2015-3-4", Err(ParseErrorKind::InvalidMonth)),
        ("2015-03-04T5:34:45Z", Err(ParseErrorKind::InvalidHour)),
        ("2015-03-04T15:4:45Z", Err(ParseErrorKind::InvalidMinute)),
        ("2015-03-04T15:34:4Z", Err(ParseErrorKind::InvalidSeconds)),
        ("2015-01-20T17:35:20.452-08:00s", Err(ParseErrorKind::StringNotEnded)),
        ("2015-01-20T17:35:20.452-08:00ss", Err(ParseErrorKind::StringNotEnded)),
    ];
    

    // Test against test data above
    for &(date, checkdate) in testdates.iter() {
        let d = parse_w3c_datetime(date);          // parse a date
        let dt = match d {                          // did we get a value?
            Ok(dt) => Ok(format_w3c(&dt)), // yes, go on
            Err(e) => {
                Err(e.error_kind)
            },                       // otherwise keep an error for the comparison
        };
        if dt != checkdate.map(|s| s.to_string()) { // check for expected result
            panic!("Date conversion failed for {}\nReceived: {:?}\nExpected: {:?}",
                   date, dt, checkdate);
        }
    };
}

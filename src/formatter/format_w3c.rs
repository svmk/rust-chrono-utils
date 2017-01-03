use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use chrono::Timelike;
use chrono::offset::Offset;
/// Returns an W3C date and time string such as `1996-12-19T16:39:57Z`.
pub fn format_w3c(datetime: &DateTime<FixedOffset>) -> String {
    let mut format = String::from("%FT%T");
    let offset = datetime.timezone().local_minus_utc().num_seconds();
    if datetime.nanosecond() > 0 {
        format = format + "%.f";
    }
    if offset == 0 {
        format = format + "Z";
    } else {
        let sign;
        if offset >= 0 {
            sign = "+";
        } else {
            sign = "-";
        }
        let offset = offset.abs();
        let hour = offset / 3600;
        let offset_seconds = offset - hour * 3600;
        let minute = offset_seconds / 60;
        format = format + &format!("{}{:02}:{:02}",sign,hour,minute);
    }
    return format!("{}",datetime.format(&format));
}
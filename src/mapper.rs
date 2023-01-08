/*
 * @Author: Quinn Tao @t-quinn-t 
 * @Date: 2023-01-06 21:00:41 
 * @Last Modified by: Quinn Tao
 * @Last Modified time: 2023-01-07 00:37:50
 */

use anyhow::{Result, Ok, Context};
use chrono::prelude::*;
use log::debug;

/// Supporting format:
/// - YYYY-mm-dd OR mm/dd/YYYY
/// - mm-dd OR mm/dd, defaulting to current year 
/// - dd defaulting to current year/month 
pub fn arg_to_time(date_str: &str) -> Result<NaiveDate> {

    let try_parse = |d, s| {
        debug!("evaluating {}", s);
        NaiveDate::parse_from_str(d, s)
    };

    // TODO: may use macros for these closures?
    let curr_year_as_prefix = |d| -> String {
        let mut t = Local::now().year().to_string();
        t.push('-');
        t.push_str(d);
        t
    };

    let curr_year_as_suffix = |d: &str| -> String {
        let mut t = String::from(d);
        t.push('/');
        t.push_str(&Local::now().year().to_string());
        t
    };

    let date = try_parse(date_str, "%F")
        .or_else(|_| try_parse(date_str, "%v"))
        .or_else(|_| try_parse(date_str, "%m/%d/%Y"));
    if date.is_ok() {
        return Ok(date.unwrap());
    }

    let yy_prefix = curr_year_as_prefix(date_str);
    let date = date.or_else(|_| {
        try_parse(&yy_prefix, "%F")
            .or_else(|_| try_parse(&yy_prefix, "%v"))
            .or_else(|_| try_parse(&yy_prefix, "%m/%d/%Y"))
    });
    if date.is_ok() {
        return Ok(date.unwrap());
    }

    let yy_suffix = curr_year_as_suffix(date_str);
    let date = date.or_else(|_| {
        try_parse(&yy_suffix, "%F")
            .or_else(|_| try_parse(&yy_suffix, "%v"))
            .or_else(|_| try_parse(&yy_suffix, "%m/%d/%Y"))
    });
    if date.is_ok() {
        return Ok(date.unwrap());
    }

    let day = date_str.to_string().parse::<u32>()?;
    let date = NaiveDate::from_ymd_opt(Local::now().year(), Local::now().month(), day).context("date out of range")?;
    Ok(date)
}

/// Convert user input into cents. Using cents to avoid all float point overhead
pub fn arg_to_cents(float_str: &str) -> Result<i32> {
    let mut chunks = float_str.split('.');
    let int = chunks.next().ok_or(anyhow::anyhow!("Invalid amount input {}", float_str))?.parse::<i32>()?;
    let decimal = match chunks.next() {
        Some(val) => {
            if val.len() > 2 {
                Err(anyhow::anyhow!("Bad decimal point in input {}", float_str))?
            } else if val.len() == 1 {
                val.parse::<i32>()? * 10
            } else {
                val.parse::<i32>()?
            }

        },
        None => 0,
    };

    if chunks.next().is_some() {
        return Err(anyhow::anyhow!("Not a valid decimal input"));
    }
    if int >= 0 {
        if int > i32::MAX / 100 || (int * 100) > i32::MAX - decimal {
            return Err(anyhow::anyhow!("Invalid amount, too large"))
        }
        Ok(int * 100 + decimal)
    } else {
        if int < i32::MIN / 100 || (int * 100) < i32::MIN + decimal {
            return Err(anyhow::anyhow!("Invalid amount, too large"))
        }
        Ok(int * 100 - decimal)
    }
}

#[cfg(test)]
mod test_eval {
    use super::*;

    #[test]
    fn test_get_time_dd_mm_yy() {
        assert!(arg_to_time("2001-07-08").is_ok());
        assert!(arg_to_time("2003-3-23").is_ok());
        assert!(arg_to_time("3-Jul-2011").is_ok());
        assert!(arg_to_time("13-Jul-2011").is_ok());
        assert!(arg_to_time("05/06/2038").is_ok());

        assert!(arg_to_time("13/06/2038").is_err());
        assert!(arg_to_time("13/06/88").is_err());
        assert!(arg_to_time("2011-Jul-3").is_err());
        assert!(arg_to_time("2003-3-32").is_err());
    }

    #[test]
    fn test_get_time_dd_mm() {
        assert!(arg_to_time("05-04").is_ok());
        assert!(arg_to_time("12/23").is_ok());
        assert!(arg_to_time("12/33").is_err());
        assert!(arg_to_time("0-0").is_err());

        let t0 = arg_to_time("12/23").unwrap();
        let y = Local::now().year();
        assert_eq!(t0.year(), y);
    }

    #[test]
    fn test_get_time_dd() {
        assert!(arg_to_time("1").is_ok());
        assert!(arg_to_time("01").is_ok());
        assert!(arg_to_time("21").is_ok());
        assert!(arg_to_time("32").is_err());

        let t0 = arg_to_time("3").unwrap();
        let now = Local::now();
        let (y, m) = (now.year(), now.month());
        assert_eq!((t0.year(), t0.month()), (y, m));
    }

    #[test]
    fn test_get_cents() {
        assert!(arg_to_cents("10.03").is_ok());
        assert!(arg_to_cents("10").is_ok());
        assert!(arg_to_cents("-10.9").is_ok());
        assert!(arg_to_cents("10.011").is_err());

        assert_eq!(1003, arg_to_cents("10.03").unwrap());
        assert_eq!(1000, arg_to_cents("10").unwrap());
        assert_eq!(-1090, arg_to_cents("-10.9").unwrap());
    }
}

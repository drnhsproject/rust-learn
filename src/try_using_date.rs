#[cfg(test)]
mod tests {
    use chrono::{
        DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime,
        TimeZone, Timelike, Utc,
    };
    use chrono_tz::{Asia, Tz};

    #[test]
    fn test_date() {
        let now = Local::now();
        println!("Current date and time: {}", now);

        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 09).unwrap();
        println!("Year: {}", date.year());
        println!("Month: {}", date.month());
        println!("Day: {}", date.day());
    }

    #[test]
    fn test_duration() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 09).unwrap();
        let new_date: NaiveDate = date + Duration::days(10);
        let duration = new_date.signed_duration_since(date);
        println!("Duration: {}", duration);
        println!("New date: {}", new_date);
    }

    #[test]
    fn test_time() {
        let time: NaiveTime = NaiveTime::from_hms_milli_opt(10, 30, 45, 500).unwrap();
        println!("Time: {}", time.hour());
        println!("Time: {}", time.minute());
        println!("Time: {}", time.second());
        println!("Time: {}", time.nanosecond());
    }

    #[test]
    fn test_date_time() {
        let date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 12, 09).unwrap(),
            NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        );

        println!("Date time: {}", date_time);
        println!("Year: {}", date_time.year());
        println!("Month: {}", date_time.month());
        println!("Day: {}", date_time.day());
        println!("Hour: {}", date_time.hour());
        println!("Minute: {}", date_time.minute());
        println!("Second: {}", date_time.second());
        println!("Nanosecond: {}", date_time.nanosecond());
    }

    #[test]
    fn test_fixed_offset() {
        let date_time: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 12, 09).unwrap(),
                NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
            ),
            FixedOffset::east_opt(3600).unwrap(),
        );

        println!("Date time: {}", date_time);

        let utc_date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 12, 09).unwrap(),
            NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        );

        let asia_jakarta: FixedOffset = FixedOffset::east_opt(7 * 3600).unwrap();
        let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

        println!("UTC Date time: {}", utc_date_time);
        println!("Asia Jakarta Offset: {}", asia_jakarta);
        println!("Asia Jakarta Date time: {}", asia_jakarta_date_time);
    }

    #[test]
    fn test_time_zone() {
        let date_time: DateTime<FixedOffset> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 12, 09).unwrap(),
                NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
            ),
            FixedOffset::east_opt(3600).unwrap(),
        );

        println!("Date time: {}", date_time);

        let utc_date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 12, 09).unwrap(),
            NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        );

        let asia_jakarta = Asia::Jakarta;
        let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

        println!("UTC Date time: {}", utc_date_time);
        println!("Asia Jakarta Offset: {}", asia_jakarta);
        println!("Asia Jakarta Date time: {}", asia_jakarta_date_time);
    }

    #[test]
    fn test_date_time_zone() {
        let utc_date_time: DateTime<Utc> = Utc::now();
        let asia_jakarta_date_time: DateTime<Tz> =
            Asia::Jakarta.from_utc_datetime(&utc_date_time.naive_utc());

        println!("UTC Date time: {}", utc_date_time);
        println!("Asia Jakarta Date time: {}", asia_jakarta_date_time);

        let local_date_time: DateTime<Local> = Local::now();
        println!("Local Date time: {}", local_date_time);

        let asia_jakarta_date_time: DateTime<Tz> = Asia::Jakarta
            .from_local_datetime(&local_date_time.naive_local())
            .unwrap();
        println!("Asia Jakarta Date time: {}", asia_jakarta_date_time);
    }

    #[test]
    fn test_parsing_date_time() {
        let string = String::from("2025-12-09 10:30:00 +0700");
        let time = DateTime::parse_from_str(&string, "%Y-%m-%d %H:%M:%S %z").unwrap();

        println!("Date time: {}", time);
        println!("Year: {}", time.year());
        println!("Month: {}", time.month());
        println!("Day: {}", time.day());
        println!("Hour: {}", time.hour());
        println!("Minute: {}", time.minute());
        println!("Second: {}", time.second());
        println!("Nanosecond: {}", time.nanosecond());
        println!("Offset: {}", time.timezone());
    }

    #[test]
    fn test_format_date_time() {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S %z").to_string();
        println!("Formatted date time: {}", formatted);
    }
}

use crate::{EloquentError, ToSql};
use chrono::prelude::*;

impl ToSql for DateTime<Local> {
    #[inline(always)]
    fn to_sql(&self) -> Result<String, EloquentError> {
        self.with_timezone(&Utc).to_sql()
    }
}

impl ToSql for DateTime<FixedOffset> {
    #[inline(always)]
    fn to_sql(&self) -> Result<String, EloquentError> {
        self.with_timezone(&Utc).to_sql()
    }
}

impl ToSql for DateTime<Utc> {
    fn to_sql(&self) -> Result<String, EloquentError> {
        Ok(self.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    use chrono::DateTime;

    #[test]
    fn test_datetime_local_to_sql() {
        let local_time = DateTime::<Local>::from_str("2025-07-10T00:00:00+08:00").unwrap();
        assert_eq!(local_time.to_sql().unwrap(), "2025-07-09T16:00:00+00:00");
    }

    #[test]
    fn test_datetime_utc_to_sql() {
        let local_time = DateTime::<Utc>::from_str("2025-07-10T00:00:00+00:00").unwrap();
        assert_eq!(local_time.to_sql().unwrap(), "2025-07-10T00:00:00+00:00");
    }
}

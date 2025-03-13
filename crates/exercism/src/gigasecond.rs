use time;
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    const ONE_BILLION: i64 = 1_000_000_000;
    let one_billion_seconds = time::Duration::seconds(ONE_BILLION);
    start + one_billion_seconds
}

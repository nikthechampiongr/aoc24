fn main() {
    let reports = common::read_input();
    let mut safe_reports = 0;
    for report in reports.lines() {
        let levels = report.split_whitespace();

        let mut decrease: Option<bool> = None;
        let mut prev: Option<i32> = None;
        let mut safe = true;
        for level in levels {
            let level = level.parse().unwrap();
            if prev.is_none() {
                prev = Some(level);
                continue;
            }

            let cmp = prev.unwrap();

            if cmp == level || cmp.abs_diff(level) > 3 {
                safe = false;
                break;
            }

            let curr_decrease = level < cmp;

            if decrease.is_none() {
                decrease = Some(curr_decrease);
                prev = Some(level);
                continue;
            }


            if curr_decrease != decrease.unwrap() {
                safe = false;
                break;
            }

            prev = Some(level);
        }

        if safe {
            safe_reports += 1;
        }

    }
    println!("Safe reports: {safe_reports}");
}

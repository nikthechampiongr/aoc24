fn main() {
    let reports = common::read_input();
    let mut safe_reports = 0;
    let mut safe_dampened_reports = 0;

    for report in reports.lines() {
        let report: Vec<i32> = report
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        assert!(!report.is_empty());

        let (safe, _) = check_report(&report);

        if safe {
            safe_reports += 1;
            safe_dampened_reports += 1;
        } else {
            // This is a very inefficient solution
            // However I cannot really iterate on my solution fast because the cooldown between
            // attempts is comically large.
            // As such you get the stupid solution
            // Maybe I'll try to make this better later
            let mut safe_dampened = false;

            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);

                if check_report(&report).0 {
                    safe_dampened = true;
                    break;
                }
            }

            if safe_dampened {
                safe_dampened_reports += 1;
            }
        }
    }
    println!("Safe reports: {safe_reports}");
    println!("Safe dampened reports: {safe_dampened_reports}");
}

fn check_report(report: &[i32]) -> (bool, Option<usize>) {
    let mut decrease: Option<bool> = None;
    let mut prev_report: Option<i32> = None;
    for (i, level) in report.iter().enumerate() {
        if prev_report.is_none() {
            prev_report = Some(*level);
            continue;
        }

        if decrease.is_none() {
            decrease = Some(*level < prev_report.unwrap())
        }

        if !check(*level, prev_report.unwrap(), decrease.unwrap()) {
            return (false, Some(i));
        }

        prev_report = Some(*level);
    }

    (true, None)
}

fn check(a: i32, b: i32, decrease: bool) -> bool {
    a != b && a.abs_diff(b) <= 3 && ((a < b) == decrease)
}

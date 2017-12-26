// use time crate
extern crate time;
use time::PreciseTime;

use postgres::{Connection, TlsMode};

#[derive(Debug)]
struct Leo {
    ukprn: i32,
    provider_name: String,
    region: String,
    subject: String,
    sex: String,

    year_after_graduation: String,
    grads: Option<String>,
    unmatched: Option<String>,
    matched: Option<String>,
    activity_not_captured: Option<String>,

    no_sust_dest: Option<String>,
    sus_temp_only: Option<String>,
    sus_temp: Option<String>,
    sus_tempfs_or_both: Option<String>,
    earnings_include: Option<String>,

    lower_ann_earn: Option<String>,
    median_ann_earn: Option<String>,
    upper_ann_earn: Option<String>,
    polar_gr_pone: Option<String>,
    polar_gr_pone_included: Option<String>,

    pr_att_band: Option<String>,
    pr_att_included: Option<String>,
}

pub fn retrieve_leo() {

    let db_url = "postgresql://yinghua:123@localhost:5432/fyp1";
    let conn = Connection::connect(db_url, TlsMode::None).unwrap();

    println!("BEGIN retrieve data from leo database. ");
    let start = PreciseTime::now();

    for rows in &conn.query("SELECT * FROM leo", &[]).unwrap() {

        let _subject = Leo {
            ukprn: rows.get(0),
            provider_name: rows.get(1),
            region: rows.get(2),
            subject: rows.get(3),
            sex: rows.get(4),

            year_after_graduation: rows.get(5),
            grads: rows.get(6),
            unmatched: rows.get(7),
            matched: rows.get(8),
            activity_not_captured: rows.get(9),

            no_sust_dest: rows.get(10),
            sus_temp_only: rows.get(11),
            sus_temp: rows.get(12),
            sus_tempfs_or_both: rows.get(13),
            earnings_include: rows.get(14),

            lower_ann_earn: rows.get(15),
            median_ann_earn: rows.get(16),
            upper_ann_earn: rows.get(17),
            polar_gr_pone: rows.get(18),
            polar_gr_pone_included: rows.get(19),

            pr_att_band: rows.get(20),
            pr_att_included: rows.get(21),
        };

        //        println!("{:?}", subject);

    }

    let end = PreciseTime::now();
    let duration = start.to(end);
    println!(
        "FINISH retrieve all rows of data from leo database with {} seconds.",
        duration
    );

}

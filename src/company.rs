// use time crate
extern crate time;
use time::PreciseTime;

use postgres::{Connection, TlsMode};

extern crate chrono;
use chrono::NaiveDate;

#[derive(Debug)]
struct Company {
    name: Option<String>,
    number: String,
    careof: Option<String>,
    po_box: Option<String>,
    address_line1: Option<String>,

    address_line2: Option<String>,
    post_town: Option<String>,
    county: Option<String>,
    country: Option<String>,
    post_code: Option<String>,

    company_category: String,
    company_status: String,
    county_of_origin: String,
    dissolution_date: Option<NaiveDate>,
    incorporation_date: Option<NaiveDate>,

    accounting_ref_day: Option<i32>,
    accounting_ref_month: Option<i32>,
    account_next_due_date: Option<NaiveDate>,
    account_last_made_update: Option<NaiveDate>,
    account_category: Option<String>,

    return_next_due_date: Option<NaiveDate>,
    return_last_made_update: Option<NaiveDate>,
    num_mort_changes: Option<i32>,
    num_mort_out_standing: Option<i32>,
    num_mort_part_satisfied: Option<i32>,

    num_mort_satisfied: Option<i32>,
    siccode1: Option<String>,
    siccode2: Option<String>,
    siccode3: Option<String>,
    siccode4: Option<String>,

    num_gen_partners: i32,
    num_lim_partners: i32,
    uri: String,
    pn1_condate: Option<NaiveDate>,
    pn1_companydate: Option<String>,

    pn2_condate: Option<NaiveDate>,
    pn2_companydate: Option<String>,
    pn3_condate: Option<NaiveDate>,
    pn3_companydate: Option<String>,
    pn4_condate: Option<NaiveDate>,

    pn4_companydate: Option<String>,
    pn5_condate: Option<NaiveDate>,
    pn5_companydate: Option<String>,
    pn6_condate: Option<NaiveDate>,
    pn6_companydate: Option<String>,

    pn7_condate: Option<NaiveDate>,
    pn7_companydate: Option<String>,
    pn8_condate: Option<NaiveDate>,
    pn8_companydate: Option<String>,
    pn9_condate: Option<NaiveDate>,

    pn9_companyname: Option<String>,
    pn10_condate: Option<NaiveDate>,
    pn10_companydate: Option<String>,
    conf_stmt_next_due_date: Option<NaiveDate>,
    conf_stmt_last_made_update: Option<NaiveDate>,
}

pub fn retrieve_company() {


    let db_url = "postgresql://yinghua:123@localhost:5432/fyp1";
    let conn = Connection::connect(db_url, TlsMode::None).unwrap();

    println!("BEGIN retrieve data from companydata database. ");
    let start = PreciseTime::now();


    for rows in &conn.query("SELECT * FROM companydata", &[]).unwrap() {
        let company = Company {
            name: rows.get(0),
            number: rows.get(1),
            careof: rows.get(2),
            po_box: rows.get(3),
            address_line1: rows.get(4),

            address_line2: rows.get(5),
            post_town: rows.get(6),
            county: rows.get(7),
            country: rows.get(8),
            post_code: rows.get(9),

            company_category: rows.get(10),
            company_status: rows.get(11),
            county_of_origin: rows.get(12),
            dissolution_date: rows.get(13),
            incorporation_date: rows.get(14),

            accounting_ref_day: rows.get(15),
            accounting_ref_month: rows.get(16),
            account_next_due_date: rows.get(17),
            account_last_made_update: rows.get(18),
            account_category: rows.get(19),

            return_next_due_date: rows.get(20),
            return_last_made_update: rows.get(21),
            num_mort_changes: rows.get(22),
            num_mort_out_standing: rows.get(23),
            num_mort_part_satisfied: rows.get(24),

            num_mort_satisfied: rows.get(25),
            siccode1: rows.get(26),
            siccode2: rows.get(27),
            siccode3: rows.get(28),
            siccode4: rows.get(29),

            num_gen_partners: rows.get(30),
            num_lim_partners: rows.get(31),
            uri: rows.get(32),
            pn1_condate: rows.get(33),
            pn1_companydate: rows.get(34),

            pn2_condate: rows.get(35),
            pn2_companydate: rows.get(36),
            pn3_condate: rows.get(37),
            pn3_companydate: rows.get(38),
            pn4_condate: rows.get(39),

            pn4_companydate: rows.get(40),
            pn5_condate: rows.get(41),
            pn5_companydate: rows.get(42),
            pn6_condate: rows.get(43),
            pn6_companydate: rows.get(44),

            pn7_condate: rows.get(45),
            pn7_companydate: rows.get(46),
            pn8_condate: rows.get(47),
            pn8_companydate: rows.get(48),
            pn9_condate: rows.get(49),

            pn9_companyname: rows.get(50),
            pn10_condate: rows.get(51),
            pn10_companydate: rows.get(52),
            conf_stmt_next_due_date: rows.get(53),
            conf_stmt_last_made_update: rows.get(54),
        };



        //        println!("{:?}", company);
    }

    let end = PreciseTime::now();
    let duration = start.to(end);
    println!(
        "FINISH retrieve all rows of data from companydata database with {} seconds.",
        duration
    );
}

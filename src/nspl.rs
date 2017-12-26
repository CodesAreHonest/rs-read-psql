// use time crate
extern crate time;
use time::PreciseTime;

use postgres::{Connection, TlsMode, types};
use postgres::types::FromSql;

extern crate chrono;
use chrono::NaiveDate;

#[derive(Debug)]
struct Nspl {
    postcode1: String,
    postcode2: String,
    postcode3: String,
    date_introduce: String,
    user_type: i32,

    easting: Option<i32>,
    northing: Option<i32>,
    position_quality: i32,
    countycode: Option<String>,
    countyname: Option<String>,

    county_lac: Option<String>,
    county_lan: Option<String>,
    ward_code: Option<String>,
    ward_name: Option<String>,
    country_code: Option<String>,

    country_name: Option<String>,
    region_code: Option<String>,
    region_name: Option<String>,
    par_cons_code: Option<String>,
    par_cons_name: Option<String>,

    eerc: Option<String>,
    eern: Option<String>,
    pctc: Option<String>,
    pctn: Option<String>,
    isoac: Option<String>,

    isoan: Option<String>,
    msoac: Option<String>,
    msoan: Option<String>,
    oacc: Option<String>,
    oacn: Option<String>,

    longitude: f32,
    latitude: f32,
    spatial_accuracy: Option<String>,
    last_upload: NaiveDate,
    location: Option<String>,
    socrataid: i32,
}

pub fn retrieve_nspl() {

    let db_url = "postgresql://yinghua:123@localhost:5432/fyp1";
    let conn = Connection::connect(db_url, TlsMode::None).unwrap();

    println!("BEGIN retrieve data from nspl database. ");
    let start = PreciseTime::now();

    for rows in &conn.query("SELECT * FROM nspl", &[]).unwrap() {

        let postcode = Nspl {
            postcode1: rows.get(0),
            postcode2: rows.get(1),
            postcode3: rows.get(2),
            date_introduce: rows.get(3),
            user_type: rows.get(4),

            easting: rows.get(5),
            northing: rows.get(6),
            position_quality: rows.get(7),
            countycode: rows.get(8),
            countyname: rows.get(9),

            county_lac: rows.get(10),
            county_lan: rows.get(11),
            ward_code: rows.get(12),
            ward_name: rows.get(13),
            country_code: rows.get(14),

            country_name: rows.get(15),
            region_code: rows.get(16),
            region_name: rows.get(17),
            par_cons_code: rows.get(18),
            par_cons_name: rows.get(19),

            eerc: rows.get(20),
            eern: rows.get(21),
            pctc: rows.get(22),
            pctn: rows.get(23),
            isoac: rows.get(24),

            isoan: rows.get(25),
            msoac: rows.get(26),
            msoan: rows.get(27),
            oacc: rows.get(28),
            oacn: rows.get(29),

            longitude: rows.get(30),
            latitude: rows.get(31),
            spatial_accuracy: rows.get(32),
            last_upload: rows.get(33),
            location: rows.get(34),
            socrataid: rows.get(35),
        };

        //        println!("{:?}", postcode);

    }

    let end = PreciseTime::now();
    let duration = start.to(end);
    println!(
        "FINISH retrieve all rows of data from nspl database with {} seconds.",
        duration
    );

}

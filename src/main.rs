#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate rocket_sat;

use rocket::response::content;
use error_chain::error_chain;
use std::io::Read;

#[get("/")]
fn json() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[get("/tle")]
pub fn tle() -> Result<String> {
    let mut res = reqwest::blocking::get("https://www.celestrak.com/NORAD/elements/active.txt")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    let mut i = 0;
    let mut str_json = vec!["[ ".to_owned()];
    loop {

        str_json.push("{ \"Name of Satellite\": \"".to_owned() + &body[i..i+24] + &"\", ".to_owned());
        str_json.push("\"Linea\": \"".to_owned() + &body[i+26..i+27] + &"\", ".to_owned());
        str_json.push("\"Satellite catalog number\": \"".to_owned() + &body[i+28..i+33] + &"\", ".to_owned());
        str_json.push("\"Classification (U=Unclassified C=Classified S=Secret)\": \"".to_owned() + &body[i+33..i+35] + &"\", ".to_owned());
        str_json.push("\"International Designator (last two digits of launch year)\": \"".to_owned() + &body[i+35..i+37] + &"\", ".to_owned());
        str_json.push("\"International Designator (launch number of the year)\": \"".to_owned() + &body[i+37..i+40] + &"\", ".to_owned());
        str_json.push("\"International Designator (piece of the launch)\": \"".to_owned() + &body[i+40..i+42] + &"\", ".to_owned());
        str_json.push("\"Epoch Year (last two digits of year)\": \"".to_owned() + &body[i+44..i+46] + &"\", ".to_owned());
        str_json.push("\"Epoch (day of the year and fractional portion of the day)\": \"".to_owned() + &body[i+46..i+58] + &"\", ".to_owned());
        str_json.push("\"First Derivative of Mean Motion aka the Ballistic Coefficient\": \"".to_owned() + &body[i+60..i+70] + &"\", ".to_owned());
        str_json.push("\"Second Derivative of Mean Motion (decimal point assumed)\": \"".to_owned() + &body[i+71..i+79] + &"\", ".to_owned());
        str_json.push("\"Drag Term aka Radiation Pressure Coefficient or BSTAR (decimal point assumed)\": \"".to_owned() + &body[i+80..i+87] + &"\", ".to_owned());
        str_json.push("\"Ephemeris type (internal use only - always zero in distributed TLE data)\": \"".to_owned() + &body[i+88..i+89] + &"\", ".to_owned());
        str_json.push("\"Element set number Incremented when a new TLE is generated for this object\": \"".to_owned() + &body[i+91..i+94] + &"\", ".to_owned());
        str_json.push("\"Checksuma\": \"".to_owned() + &body[i+94..i+95] + &"\", ".to_owned());
        str_json.push("\"Lineb\": \"".to_owned() + &body[i+97..i+98] + &"\", ".to_owned());
        str_json.push("\"Satellite Catalog number\": \"".to_owned() + &body[i+99..i+104] + &"\", ".to_owned());
        str_json.push("\"Inclination (degrees)\": \"".to_owned() + &body[i+106..i+113] + &"\", ".to_owned());
        str_json.push("\"Right Ascension of the Ascending Node (degrees)\": \"".to_owned() + &body[i+114..i+122] + &"\", ".to_owned());
        str_json.push("\"Eccentricity (decimal point assumed)\": \"".to_owned() + &body[i+123..i+130] + &"\", ".to_owned());
        str_json.push("\"Argument of Perigee (degrees)\": \"".to_owned() + &body[i+131..i+139] + &"\", ".to_owned());
        str_json.push("\"Mean Anomaly (degrees)\": \"".to_owned() + &body[i+140..i+148] + &"\", ".to_owned());
        str_json.push("\"Mean Motion (revolutions per day)\": \"".to_owned() + &body[i+149..i+160] + &"\", ".to_owned());
        str_json.push("\"Revolution number at epoch (revolutions)\": \"".to_owned() + &body[i+160..i+165] + &"\", ".to_owned());
        str_json.push("\"Checksumb\": \"".to_owned() + &body[i+165..i+166] + &"\"},".to_owned());
        i += 168;
        // println!(" i = {}, body.len() = {}\n", i, body.len());

        if body.len() == i {
            break;
        }
    }
    str_json.push("]".to_owned());

    let joined = str_json.join("");
    // Ok(body)
    Ok(joined)
}

/*fn tle() -> Result<String, Error> {*/
//rocket_sat::queries::tle::tle()
//}

fn main() {
    rocket::ignite().mount("/", routes![json, tle]).launch();

}

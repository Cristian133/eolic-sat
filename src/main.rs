#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket_cors;
use sgp4;
use ureq;

#[get("/")]
fn json() -> content::Json<&'static str> {
    content::Json("{'hi':'world'}")
}

// All GP queries on CelesTrak will take the form:

//     https://celestrak.com/NORAD/elements/gp.php?{QUERY}=VALUE[&FORMAT=VALUE]

// where {QUERY} is:

//     CATNR: Catalog Number (1 to 9 digits). Allows return of data for a single catalog number.
//     INTDES: International Designator (yyyy-nnn). Allows return of data for all objects associated with a particular launch.
//     GROUP: Groups of satellites provided on the CelesTrak Current Data page.
//     NAME: Satellite Name. Allows searching for satellites by parts of their name.

// Allowed formats are:

//     TLE or 3LE: Three-line element sets.
//     2LE: Two-line element sets (no name).
//     XML: OMM XML format including all mandatory elements.
//     KVN: OMM KVN format including all mandatory elements.
//     JSON: OMM keywords for all TLE elements in JSON format.
//     JSON-PRETTY: OMM keywords for all TLE elements in JSON pretty-print format.
//     CSV: OMM keywords for all TLE elements in CSV format.

// The FORMAT specification is optional, but defaults to XML.

#[get("/catnr/<catnr>")]
fn catnr(catnr: String) -> content::Json<String> {
    let response = ureq::get("https://celestrak.com/NORAD/elements/gp.php")
        .query("CATNR", &catnr)
        .query("FORMAT", "json")
        .call();
    let r = response.into_string().unwrap();
    content::Json(r)
}

#[get("/intdes/<intdes>")]
fn intdes(intdes: String) -> content::Json<String> {
    let response = ureq::get("https://celestrak.com/NORAD/elements/gp.php")
        .query("INTDES", &intdes)
        .query("FORMAT", "json")
        .call();
    let r = response.into_string().unwrap();
    content::Json(r)
}

#[get("/group/<group>")]
fn group(group: String) -> content::Json<String> {
    let response = ureq::get("https://celestrak.com/NORAD/elements/gp.php")
        .query("GROUP", &group)
        .query("FORMAT", "json")
        .call();
    let r = response.into_string().unwrap();
    content::Json(r)
}

#[get("/name/<name>")]
fn name(name: String) -> content::Json<String> {
    let response = ureq::get("https://celestrak.com/NORAD/elements/gp.php")
        .query("NAME", &name)
        .query("FORMAT", "json")
        .call();
    let r = response.into_string().unwrap();
    content::Json(r)
}

fn main() {
    let options = rocket_cors::CorsOptions::default();

    rocket::ignite()
        .mount("/", routes![json, catnr, intdes, group, name, tlepv])
        .manage(options)
        .launch();
}

#[get("/tlepv")]
fn tlepv() -> sgp4::Result<()> {
    let response = ureq::get("https://celestrak.com/NORAD/elements/gp.php")
        .query("GROUP", "stations")
        .query("FORMAT", "json")
        .call();
    if response.error() {
        Err(sgp4::Error::new(format!(
            "network error {}: {}",
            response.status(),
            response.into_string()?
        )))
    } else {
        let elements_group: Vec<sgp4::Elements> = response.into_json_deserialize()?;
        for elements in &elements_group {
            println!("{}", elements.object_name.as_ref().unwrap());
            let constants = sgp4::Constants::from_elements(elements)?;
            for hours in &[12, 24] {
                println!("    t = {} min", hours * 60);
                let prediction = constants.propagate((hours * 60) as f64)?;
                println!("        r = {:?} km", prediction.position);
                println!("        ṙ = {:?} km.s⁻¹", prediction.velocity);
            }
        }
        Ok(())
    }
}

// #[get("/tle/<name>")]
// pub fn tle(name: String) -> Result<String> {
//     let base = "https://www.celestrak.com/NORAD/elements/";
//     let path = base.to_owned() + &name + &".txt".to_owned();
//     let mut res = reqwest::blocking::get(&path)?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);

//     let mut i = 0;
//     let mut str_json = vec!["[ ".to_owned()];
//     loop {
//         str_json
//             .push("{ \"Name of Satellite\": \"".to_owned() + &body[i..i + 24] + &"\", ".to_owned());
//         str_json.push("\"Linea\": \"".to_owned() + &body[i + 26..i + 27] + &"\", ".to_owned());
//         str_json.push(
//             "\"Satellite catalog number\": \"".to_owned()
//                 + &body[i + 28..i + 33]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Classification (U=Unclassified C=Classified S=Secret)\": \"".to_owned()
//                 + &body[i + 33..i + 35]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"International Designator (last two digits of launch year)\": \"".to_owned()
//                 + &body[i + 35..i + 37]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"International Designator (launch number of the year)\": \"".to_owned()
//                 + &body[i + 37..i + 40]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"International Designator (piece of the launch)\": \"".to_owned()
//                 + &body[i + 40..i + 42]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Epoch Year (last two digits of year)\": \"".to_owned()
//                 + &body[i + 44..i + 46]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Epoch (day of the year and fractional portion of the day)\": \"".to_owned()
//                 + &body[i + 46..i + 58]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"First Derivative of Mean Motion aka the Ballistic Coefficient\": \"".to_owned()
//                 + &body[i + 60..i + 70]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Second Derivative of Mean Motion (decimal point assumed)\": \"".to_owned()
//                 + &body[i + 71..i + 79]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Drag Term aka Radiation Pressure Coefficient or BSTAR (decimal point assumed)\": \""
//                 .to_owned()
//                 + &body[i + 80..i + 87]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Ephemeris type (internal use only - always zero in distributed TLE data)\": \""
//                 .to_owned()
//                 + &body[i + 88..i + 89]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Element set number Incremented when a new TLE is generated for this object\": \""
//                 .to_owned()
//                 + &body[i + 91..i + 94]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push("\"Checksuma\": \"".to_owned() + &body[i + 94..i + 95] + &"\", ".to_owned());
//         str_json.push("\"Lineb\": \"".to_owned() + &body[i + 97..i + 98] + &"\", ".to_owned());
//         str_json.push(
//             "\"Satellite Catalog number\": \"".to_owned()
//                 + &body[i + 99..i + 104]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Inclination (degrees)\": \"".to_owned()
//                 + &body[i + 106..i + 113]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Right Ascension of the Ascending Node (degrees)\": \"".to_owned()
//                 + &body[i + 114..i + 122]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Eccentricity (decimal point assumed)\": \"".to_owned()
//                 + &body[i + 123..i + 130]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Argument of Perigee (degrees)\": \"".to_owned()
//                 + &body[i + 131..i + 139]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Mean Anomaly (degrees)\": \"".to_owned()
//                 + &body[i + 140..i + 148]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Mean Motion (revolutions per day)\": \"".to_owned()
//                 + &body[i + 149..i + 160]
//                 + &"\", ".to_owned(),
//         );
//         str_json.push(
//             "\"Revolution number at epoch (revolutions)\": \"".to_owned()
//                 + &body[i + 160..i + 165]
//                 + &"\", ".to_owned(),
//         );
//         str_json
//             .push("\"Checksumb\": \"".to_owned() + &body[i + 165..i + 166] + &"\"},".to_owned());
//         i += 168;
//         // println!(" i = {}, body.len() = {}\n", i, body.len());

//         if body.len() == i {
//             break;
//         }
//     }
//     str_json.push("]".to_owned());

//     let joined = str_json.join("");
//     // Ok(body)
//     Ok(joined)
// }


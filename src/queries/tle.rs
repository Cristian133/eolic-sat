/*use serde::Deserialize;*/
//use reqwest::Error;

//#[derive(Deserialize, Debug)]
//struct User {
//login: String,
//id: u32,
//}

//#[tokio::main]
//async fn tle() -> Result<(), Error> {
//let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
//owner = "rust-lang-nursery",
//repo = "rust-cookbook");
//println!("{}", request_url);
//let response = reqwest::get(&request_url).await?;

//let users: Vec<User> = response.json().await?;
//println!("{:?}", users);
//Ok(())
//}


use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn tle() -> Result<String> {
    let mut res = reqwest::blocking::get("https://www.celestrak.com/NORAD/elements/stations.txt")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    let mut i = 0;
    loop {
        println!("Name of Satellite:\n{}", &body[i..i+25]);
        println!("Line number\n{}", &body[i+26..i+27]);
        println!("Satellite catalog number\n{}", &body[i+28..i+33]);
        println!("Classification (U=Unclassified, C=Classified, S=Secret)\n{}", &body[i+33..i+35]);
        println!("International Designator (last two digits of launch year)\n{}", &body[i+35..i+37]);
        println!("International Designator (launch number of the year)\n{}", &body[i+37..i+40]);
        println!("International Designator (piece of the launch)\n{}", &body[i+40..i+42]);
        println!("Epoch Year (last two digits of year)\n{}", &body[i+44..i+46]);
        println!("Epoch (day of the year and fractional portion of the day)\n{}", &body[i+46..i+58]);
        println!("First Derivative of Mean Motion aka the Ballistic Coefficient\n{}", &body[i+60..i+70]);
        println!("Second Derivative of Mean Motion (decimal point assumed)\n{}", &body[i+71..i+79]);
        println!("Drag Term aka Radiation Pressure Coefficient or BSTAR (decimal point assumed)\n{}", &body[i+80..i+87]);
        println!("Ephemeris type (internal use only - always zero in distributed TLE data)\n{}", &body[i+88..i+89]);
        println!("Element set number. Incremented when a new TLE is generated for this object.\n{}", &body[i+91..i+94]);
        println!("Checksum (modulo 10)\n{}", &body[i+94..i+95]);
        println!("-----------------------------------");
        println!("Line number\n{}", &body[i+97..i+98]);
        println!("Satellite Catalog number\n{}", &body[i+99..i+104]);
        println!("Inclination (degrees)\n{}", &body[i+106..i+113]);
        println!("Right Ascension of the Ascending Node (degrees)\n{}", &body[i+114..i+122]);
        println!("Eccentricity (decimal point assumed)\n{}", &body[i+123..i+130]);
        println!("Argument of Perigee (degrees)\n{}", &body[i+131..i+139]);
        println!("Mean Anomaly (degrees)\n{}", &body[i+140..i+148]);
        println!("Mean Motion (revolutions per day)\n{}", &body[i+149..i+160]);
        println!("Revolution number at epoch (revolutions)\n{}", &body[i+160..i+165]);
        println!("Checksum (modulo 10)\n{}", &body[i+165..i+166]);
        println!("--------------------------------------------------------------------------------");
        println!("--------------------------------------------------------------------------------");
        i += 168;
        println!(" i = {}, body.len() = {}\n", i, body.len());

        if body.len() == i {
            break;
        }
    }

    // Ok(body)
    Ok(body)
}


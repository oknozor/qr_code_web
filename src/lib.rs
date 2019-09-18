#![feature(proc_macro_hygiene, decl_macro)]
#![feature(test)]
#[macro_use]
extern crate rocket;
extern crate test;
use rocket::http::RawStr;

use qrcode::types::QrError;
use qrcode::QrCode;

#[get("/qr_code?<content>")]
fn qr_code(content: &RawStr) -> String {
    generate_qr_code(content.as_str()).expect("Unable to generate code")
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![qr_code])
}

pub fn generate_qr_code(input: &str) -> Result<String, QrError> {
    let code = QrCode::new(input.as_bytes())?;

    Ok(code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build())
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[bench]
    fn genrate_a_simple_qr_code(b: &mut Bencher) {
        b.iter(|| generate_qr_code("123515"))
    }

    #[test]
    fn should_genrate_a_simple_qr_code() {
        let qr_code = generate_qr_code("12356").unwrap();
        assert_eq!(qr_code, 
r#"██████████████  ████    ██  ██████████████
██          ██      ████    ██          ██
██  ██████  ██    ████      ██  ██████  ██
██  ██████  ██  ████  ██    ██  ██████  ██
██  ██████  ██  ██  ██  ██  ██  ██████  ██
██          ██  ████████    ██          ██
██████████████  ██  ██  ██  ██████████████
                ██    ████                
██      ██  ████████  ██  ██████████    ██
    ██  ██    ██      ████    ██  ████    
  ██  ████  ████  ██████    ██████  ██  ██
██████            ██      ████  ████  ████
██    ████████  ██████  ██████      ██  ██
                ██████  ██████      ██████
██████████████  ██████  ████      ██  ██  
██          ██      ██████    ██  ██  ██  
██  ██████  ██  ████████    ██████  ██████
██  ██████  ██        ████    ██  ██  ████
██  ██████  ██    ██  ██    ██████████    
██          ██      ██    ████  ██  ████  
██████████████  ██  ██  ██████      ██████"#)
    }
}

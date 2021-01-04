fn power(base: f64, exponent: i32) -> Result<f64, String> {
    if equal(base, 0.0) && exponent < 0 {
        return Ok(0.0f64);
    }

    let mut absExponent = exponent as u32;
    if exponent < 0 {
        absExponent = -exponent as u32;
    }
    let mut result = power_with_unsigned_exponent(base, absExponent); 
    if exponent < 0 {
        result = 1.0 / result;
    } 
    // unimplemented!()
    Ok(result)
}

fn equal(a: f64, b: f64) -> bool {
    if (a - b) < 0.000000001 || b - a < 0.000000001 {
        true
    }else {
        false
    }
}

fn power_with_unsigned_exponent(base: f64, exponent: u32) -> f64 {
    let mut result = 1.0;
    for _ in 1..=exponent {
        result *= base;
    }
    result
    // unimplemented!()
}

#[test]
fn test_power() {
    let number = power(2.0, 3);
    println!("number = {:?}", number);
}

#[test]
fn test_base_is_zero() {
    let number = power(0.0, 3);
    println!("number = {:?}", number);
}
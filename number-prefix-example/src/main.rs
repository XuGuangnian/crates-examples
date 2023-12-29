use number_prefix::{NumberPrefix, Prefix};

fn main() {
    let parse = "12.5 MB".parse::<NumberPrefix<f64>>().unwrap();
    println!("{:?}", parse);
    let size = match "".parse::<NumberPrefix<f64>>() {
        Ok(NumberPrefix::Prefixed(Prefix::Gibi, num)) => num * 1024.0 * 1024.0 * 1024.0,
        Ok(NumberPrefix::Prefixed(Prefix::Mega, num)) => num * 1024.0 * 1024.0,
        Ok(NumberPrefix::Prefixed(_, _)) => todo!(),
        Ok(NumberPrefix::Standalone(_)) => todo!(),
        Err(_) => 0_f64,
    };

    println!("size: {}", size);

    assert_eq!(
        "7.05E".parse::<NumberPrefix<_>>(),
        Ok(NumberPrefix::Prefixed(Prefix::Exa, 7.05_f64))
    );

    assert_eq!(
        "7.05".parse::<NumberPrefix<_>>(),
        Ok(NumberPrefix::Standalone(7.05_f64))
    );

    assert_eq!(
        "7.05 GiB".parse::<NumberPrefix<_>>(),
        Ok(NumberPrefix::Prefixed(Prefix::Gibi, 7.05_f64))
    );
}

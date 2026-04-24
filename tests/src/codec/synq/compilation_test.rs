macro_rules! gen_test {
    ($name:literal) => {
        paste::paste! {
            #[test]
            fn [<compile_ $name>]() {
                let content = crate::compile!(format!("{}.synq", $name).as_str());
                assert!(content.is_ok())
            }
        }
    };
}

gen_test!("frame");
gen_test!("synq");
gen_test!("message");

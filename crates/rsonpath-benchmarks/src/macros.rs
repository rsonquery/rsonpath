#[macro_export]
macro_rules! benchsets {
    (name = $name:ident; config = $config:expr; targets = $( $target:path ),+ $(,)*) => {
        pub fn $name() {
            let mut criterion: ::criterion::Criterion<_> = $config
                .configure_from_args();
            $(
                match $target(&mut criterion)
                {
                    Ok(_) => (),
                    Err(err) => {
                        ::std::panic!("error running benchset: {}", err);
                    }
                }
            )+
        }

        ::criterion::criterion_main! { $name }
    };
    ($name:ident, $( $target:path ),+ $(,)*) => {
        $crate::benchsets!{
            name = $name;
            config = ::criterion::Criterion::default();
            targets = $( $target ),+
        }
    }
}

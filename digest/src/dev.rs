//! Development-related functionality

pub use blobby;

mod fixed;
mod mac;
mod variable;
mod xof;

pub use fixed::*;
pub use mac::*;
pub use variable::*;
pub use xof::*;

/// Define test
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
macro_rules! new_test {
    ($name:ident, $test_name:expr, $hasher:ty, $test_func:ident $(,)?) => {
        #[test]
        fn $name() {
            use digest::dev::blobby::Blob2Iterator;
            let data = include_bytes!(concat!("data/", $test_name, ".blb"));

            for (i, row) in Blob2Iterator::new(data).unwrap().enumerate() {
                let [input, output] = row.unwrap();
                if let Some(desc) = $test_func::<$hasher>(input, output) {
                    panic!(
                        "\n\
                         Failed test №{}: {}\n\
                         input:\t{:?}\n\
                         output:\t{:?}\n",
                        i, desc, input, output,
                    );
                }
            }
        }
    };
}

/// Define benchmark
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
macro_rules! bench {
    ($name:ident, $engine:path, $bs:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut d = <$engine>::default();
            let data = [0; $bs];

            b.iter(|| {
                d.update(&data[..]);
            });

            b.bytes = $bs;
        }
    };

    ($engine:path) => {
        extern crate test;

        use digest::Digest;
        use test::Bencher;

        $crate::bench!(bench1_10, $engine, 10);
        $crate::bench!(bench2_100, $engine, 100);
        $crate::bench!(bench3_1000, $engine, 1000);
        $crate::bench!(bench4_10000, $engine, 10000);
    };
}

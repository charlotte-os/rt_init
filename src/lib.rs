#[macro_export]
macro_rules! rt_init {
    ($($vis:vis static $name:ident: $type:ty = $init:expr;)+) => {
        $(
            $vis static $name: ::spin::Lazy<$type> = ::spin::Lazy::new(|| $init);
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_initialization() {
        rt_init! {
            static STATIC1: Vec<u64> = vec![1, 2, 3];
            static STATIC2: u64 = 42;
            static STATIC3: String = "Hello, World!".to_string();
        }

        assert_eq!(*STATIC1, vec![1, 2, 3]);
        assert_eq!(*STATIC2, 42);
        assert_eq!(STATIC3.as_str(), "Hello, World!");
    }

    #[test]
    fn test_visibility_modifiers() {
        rt_init! {
            pub static PUB_STATIC: u32 = 100;
            pub(crate) static CRATE_STATIC: u32 = 200;
        }

        assert_eq!(*PUB_STATIC, 100);
        assert_eq!(*CRATE_STATIC, 200);
    }

    #[test]
    fn test_complex_initialization() {
        rt_init! {
            static COMPLEX_STATIC: Vec<u64> = {
                let mut vec = Vec::new();
                for i in 0..5 {
                    vec.push(i * 2);
                }
                vec
            };
        }

        assert_eq!(*COMPLEX_STATIC, vec![0, 2, 4, 6, 8]);
    }
}

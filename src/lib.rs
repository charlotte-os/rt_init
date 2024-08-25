#[macro_export]
macro_rules! rt_init {
    ($($vis:vis static $name:ident: $type:ty = $init:expr;)+) => {
        $(
            $vis static $name: ::spin::Lazy<$type> = ::spin::Lazy::new(|| $init);
        )*
    };
}


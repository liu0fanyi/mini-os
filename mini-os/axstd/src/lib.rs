#![no_std]

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
    axhal::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
($fmt: literal $(, $($arg: tt)+)?) => {
    axhal::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

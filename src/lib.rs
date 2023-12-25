#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!("./api.rs");
}

#[cfg(test)]
mod tests {
    use super::*;
}

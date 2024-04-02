use std::convert::TryInto;
use std::os::raw::c_int;
use std::os::raw::c_float;
use std::os::raw::c_double;
use std::ptr::null;
use std::mem::drop;


#[link(name = "emd")]
extern "C" {
    fn simple_emd(
        values1: *const c_float,
        values2: *const c_float,
        size: c_int,
    ) -> c_float;
}


pub fn distance(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    unsafe { simple_emd(x.as_ptr(), y.as_ptr(), x.len().try_into().unwrap()) }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_distance() {
        let x = vec![4.0f32, 3.0f32];
        let y = vec![3.0f32, 5.0f32];
        assert_eq!(distance(&x, &y), 0.5f32);
    }
}

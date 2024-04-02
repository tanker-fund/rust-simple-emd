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
        epsilon: c_double,
    ) -> c_float;
}


pub fn distance(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    unsafe { simple_emd(x.as_ptr(), y.as_ptr(), x.len().try_into().unwrap(), 1e-6) }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_distance() {
        let x = vec![4.0f32, 3.0f32];
        let y = vec![3.0f32, 5.0f32];
        assert_eq!(distance(&x, &y), 0.5f32);

        let x: Vec<f32> = vec![1.0, 11.0, 0.0];
        let y: Vec<f32> = vec![0.44476938, 10.044238, 1.5109923];
        assert!(distance(&x, &y) > 0.0);

        let x: Vec<f32> = vec![1.0, 11.0, 0.0];
        let y: Vec<f32> = vec![10.326917, 1.2101055, 0.46297744];
        assert!(distance(&x, &y) > 0.0);

        let x: Vec<f32> = vec![1.0, 11.0, 0.0];
        let y: Vec<f32> = vec![10.326917, 1.2101055, 0.46297744];
        assert!(distance(&x, &y) > 0.0);

        let x: Vec<f32> = vec![1.0, 11.0, 0.0];
        let y: Vec<f32> = vec![6.609991, 4.5159807, 0.8740281];
        assert!(distance(&x, &y) > 0.0);

        let x: Vec<f32> = vec![1.0, 11.0, 0.0];
        let y: Vec<f32> = vec![3.8477046, 7.859374, 0.29292116];
        assert!(distance(&x, &y) > 0.0);

    }
}

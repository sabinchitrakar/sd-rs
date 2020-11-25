[![Build Status](https://travis-ci.com/immortalinfidel/sd-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/sd-rs)

# Standard Deviation (SD)
```
use ta_common::traits::Indicator;
use sd_rs::SD;
let mut sd = SD::new(5);
assert_eq!(sd.next(81.59), None);
assert_eq!(sd.next(81.06), None);
assert_eq!(sd.next(82.87), None);
assert_eq!(sd.next(83.00), None);
assert_eq!(sd.next(83.61), Some(0.9479789027188307));
assert_eq!(sd.next(83.15), Some(0.8754290376723858));
assert_eq!(sd.next(82.84), Some(0.28032837887020867));
assert_eq!(sd.next(83.99), Some(0.42300827415075176));
assert_eq!(sd.next(84.55), Some(0.6051247805205106));
assert_eq!(sd.next(84.36), Some(0.67139854036183));
assert_eq!(sd.next(85.53), Some(0.8713346085173014));
assert_eq!(sd.next(86.54), Some(0.9256910931839016));
assert_eq!(sd.next(86.89), Some(1.0187561042761923));
assert_eq!(sd.next(87.77), Some(1.1738381489796619));
assert_eq!(sd.next(87.29), Some(0.7575645187045117));
```
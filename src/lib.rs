#![feature(external_doc)]
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;
use sma_rs::SMA;
use std::ops::Div;
#[doc(include = "../README.md")]
pub struct SD {
    period: u32,
    history: FixedQueue<f64>,
    sma: SMA,
    current_sma:f64
}


impl SD {
    pub fn new(period: u32) -> SD {
        Self {
            period,
            history: FixedQueue::new(period),
            sma: SMA::new(period),
            current_sma:0.0,
        }
    }

    pub fn get_current_sma(&self) -> f64 {
        return self.current_sma;
    }
}

impl Indicator<f64, Option<f64>> for SD {
    fn next(&mut self, input: f64) -> Option<f64> {
        self.current_sma = self.sma.next(input).unwrap_or(0.0);
        self.history.add(input);
        return if self.history.is_full() {
            let mut sd = 0.0_f64;
            for i in 0..self.history.size() {
                let val = self.history.at(i as i32).unwrap();
                sd = sd + ((val - self.current_sma).powf(2.0));
            }
            Some(sd.div(self.period as f64).sqrt())
        } else {
            None
        };

    }

    fn reset(&mut self) {
        self.history.clear();
        self.current_sma=0.0;
    }
}


#[cfg(test)]
mod tests {
    use crate::SD;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
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
    }
}

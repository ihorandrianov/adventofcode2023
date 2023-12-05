#[derive(Debug)]
pub struct RangeMap {
    ranges: Vec<Ranger>,
}

impl RangeMap {
    pub fn from(ranges: Vec<Ranger>) -> Self {
        Self { ranges }
    }
    pub fn transform(&self, number: u32) -> u32 {
        let mut res = number;
        for range in &self.ranges {
            if let Some(num) = range.eval(number) {
                res = num
            };
        }
        res
    }
}

#[derive(Debug)]
pub struct Ranger {
    from: usize,
    to: usize,
    diff: i64,
}

impl Ranger {
    pub fn new(from: Vec<i64>) -> Self {
        let start = from[1];
        let to = from[1] + from[2];
        let diff = from[0] - from[1];
        Self {
            from: start as usize,
            to: to as usize,
            diff,
        }
    }
    pub fn eval(&self, num: u32) -> Option<u32> {
        if num >= self.from as u32 && num < self.to as u32 {
            Some((num as i64 + self.diff) as u32)
        } else {
            None
        }
    }
}

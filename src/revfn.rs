pub struct RevFn {
    pub function: Vec<usize>,
}

impl RevFn {
    pub fn set(&mut self, x: usize, y: usize) {
        self.function[y] = x;
    }
    pub fn apprev(&self, y: usize) -> usize {
        return self.function[y];
    }

    pub fn appnorm(&self, x: usize) -> usize {
        let len = self.function.len();
        if x <= 0 {
            return len;
        }
        for (pos, val) in self.function.iter().enumerate() {
            // println!("{} {}", pos, val);
            if *val <= x {
                // println!("{} {}", val, pos);
                return pos;
            }
        };
        return 0;
    }

    pub fn clone(&self) -> Self {
        return RevFn { function: self.function.clone() };
    }
}

pub fn average(functions: Vec<RevFn>) -> Vec<f64> {
    let mut max = 0;
    for mut f in functions.iter() {
        let val = f.apprev(0);
        if val > max {
            max = val;
        }
    }
    // max = 10;
    let mut average = vec![0.0; max];
    for f in functions.iter() {
        let mut index = f.function.len() - 1;
        let mut value = f.function[index];
        let mut iter = 0;
        while index > 0 {
            while iter < value {
                average[iter] += value as f64;
                iter += 1;
            }
            index -= 1;
            value = f.function[index];
        }
    }


    for i in 0..max {
        average[i] /= functions.len() as f64;
    }

    return average;
}
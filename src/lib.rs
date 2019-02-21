extern crate quickersort;

pub mod statests;

pub struct MT {
    state : [u64; 624],
    index: u32
}

impl MT {
    pub fn new() -> MT {
        MT {state: [0;624], index: 0}
    }

    pub fn seed(self: &mut MT,  seed: u64) {
        self.state[0] = seed & 0xffffffff ;
        for i in 1..624 {
            let x = i as u64;
            let m :u64 = 1_812_433_253;
            self.state[i] = m.wrapping_mul((self.state[i-1] ^
                             (self.state[i-1] >> 30))) + x;
            self.state[i] = self.state[i] & 0xffffffff;
                            
        }
        self.index = 624
    } // end of seed

    pub fn get(self: &mut MT) -> u64 {
        let lower_mask : u64 = 0x7fffffff;
        let upper_mask : u64 = 0x80000000;
        let a : u64 = 0x9908b0df;
        
        let m = 624;
        let n = 397;
        let k : [u64; 2] = [0, a];
        if self.index >= 624 {
            //generate numbers
            if self.index == 625 {
                self.seed(5489);
            }
            for x in 0..m-n {
                let y = (self.state[x] & upper_mask) | (self.state[x+1 as usize] & lower_mask);
                self.state[x] = self.state[x+n] ^ (y >> 1) ^ k[(y&0x1) as usize];
            }
            let neg: i32 = ( n as i32 - m as i32);
            for x in m-n..m-1{
                let ix = x as i32;
                let y = (self.state[x] & upper_mask) | (self.state[x+1]  & lower_mask);
                self.state[x] = self.state[(ix+neg) as usize] ^  (y >> 1) ^ k[(y & 0x1) as usize];
            }
            let y = (self.state[m-1] & upper_mask) | (self.state[0] & lower_mask);
            self.state[m-1] = self.state[n-1] ^ (y >> 1) ^ k[(y&0x1) as usize];

            self.index = 0;
        }
        
        // Tempering
        let mut l:u64  = self.state[self.index as usize];
        self.index = self.index + 1;
        l = l ^ (l >> 11);
        l = l ^ ((l << 7) &  0x9d2c5680);
        l = l ^ ((l << 15) & 0xefc60000);
        l = l ^ (l >> 18);
        l
    }

    pub fn get_real(self: &mut MT) -> f32 {
        let g = self.get();
        (g as f32) * (1.0f32/4294967296.0f32)
    }




}

impl Iterator for MT {
    type Item =  u64;

    fn next(&mut self) -> Option<u64> {
        let k = self.get();
        Some(k)
    }
}

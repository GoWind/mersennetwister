mod statests {
    // Calculate the chi-squared value for the sample from the rng
    pub fn rngchi(v: Vec<f32>, b: u32) {
        let bins = bin(v, b);
        // now that we have 'binned' v into b bins 
        // calculate expected n*p(bin) for each bin
        
    }

    // split the vector v in bins such that items from the vector
    // are roughly distributed into the bins equitably
    pub fn bin(v: Vec<f32>, bins: u32) -> Vec<u32> {
        let mut binv = Vec::new();
        let mut bincount = Vec::with_capacity(bins as usize);
        let binsf32 = bins as f32;
        for i in 0..bins+1 {
            binv.push((i as f32) * (1.0f32/binsf32));
        }
        for item in v {
           let mut idx = bins;
           while item < binv[idx as usize]  {
                idx = idx -1;
           }
           bincount[idx as usize] += 1;
        }
        bincount
    }
}

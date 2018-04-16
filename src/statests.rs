mod statests {
    use quickersort;
    // Calculate the chi-squared value for the sample from the rng
    pub fn rngchi(v: &Vec<f32>, b: u32) {
        let bins = bin(v, b);
        // now that we have 'binned' v into b bins 
        // calculate expected n*p(bin) for each bin

    }

    // split the vector v in bins such that items from the vector
    // are roughly distributed into the bins equitably
    pub fn bin(v: &Vec<f32>, bins: u32) -> Vec<u32> {
        let mut binv = Vec::new();
        let  mut vcopy = v.clone();

        let mut bincount = Vec::with_capacity(bins as usize);
        quickersort::sort_floats(&mut vcopy);
        for i in 0..bins {
            bincount.insert(i as usize, 0);
        }
        //replace this by calculating the max(vcopy) - min(copy) / bins+1
        let d = (vcopy[vcopy.len()-1] - vcopy[0]) / (bins as f32);
        binv.push(vcopy[0]);
        for i in 1..bins+1 {
            binv.push(vcopy[0] + ((i as f32 * d)));
        }
        for item in v {
            let mut idx_max = 1;
            let mut binidx = 0;
            while item > &binv[idx_max] && idx_max <= binv.len()-1 {
                idx_max += 1;
                binidx += 1;
            }

            bincount[binidx] += 1;
            
        }
        bincount
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_some_stuff () {
            let test_items = vec![ 0.09411412,  0.60766515,  0.73813616,  0.73940241,  0.3727791 ,
            0.4703015 ,  0.58060895,  0.78641172,  0.45267021,  0.06347364,
            0.39489554,  0.83582679,  0.25423401,  0.18303462,  0.89189451,
            0.09696578,  0.53427369,  0.36446026,  0.42720965,  0.7935374 ,
            0.90780711,  0.11600202,  0.26208447,  0.95336705,  0.37059867,
            0.13294042,  0.19068664,  0.33125388,  0.57889386,  0.33317321,
            0.29831575,  0.95245595,  0.51019199,  0.32286373,  0.89663804,
            0.99358007,  0.82987749,  0.18297249,  0.58551784,  0.72953797,
            0.86942457,  0.92412221,  0.25850823,  0.89696456,  0.78296868,
            0.89571494,  0.45477669,  0.091698  ,  0.33123107,  0.47493998,
            0.17718802,  0.13962617,  0.1952651 ,  0.52003725,  0.34466757,
            0.04824041,  0.77790333,  0.52702591,  0.98519357,  0.76001192,
            0.07494921,  0.84988194,  0.63087364,  0.76641068,  0.6299559 ,
            0.4351151 ,  0.99263709,  0.44364432,  0.95347772,  0.5931436 ,
            0.50212558,  0.29064759,  0.00733029,  0.43789441,  0.05321575,
            0.28347446,  0.48313246,  0.19130598,  0.81299222,  0.91236247,
            0.5972047 ,  0.36516918,  0.43454668,  0.3356373 ,  0.94491055,
            0.95463197,  0.85726131,  0.71824116,  0.73977422,  0.07130504,
            0.57091088,  0.06633866,  0.02498385,  0.93918347,  0.43673544,
            0.18938621,  0.76750438,  0.75809562,  0.27087543,  0.73973458];
            let bins_count = bin(&test_items, 10);
            assert_eq!(bins_count, vec![11, 10,  7, 11, 11, 11,  3, 14,  7, 15]);
        }
    }
}

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;
use crate::dovecote::DoveCote;

/// Stores initial conditions for the simulation
pub(crate) struct InitCond {
    size: usize,
    comp: usize,
    rng: Pcg64Mcg,
    range: Uniform<usize>,
}

impl InitCond {
    pub fn new(size: usize, comp: usize) -> Self {
        let rng = Pcg64Mcg::from_entropy();
        let range = Uniform::new(0, size);

        return InitCond {
            size,
            comp,
            rng,
            range,
        };
    }

    pub fn run_sim(&mut self) -> DoveCote {
        let mut dovecote = DoveCote::new(self.size);
        while {
            let mut best_index = self.range.sample(&mut self.rng);

            if self.comp > 1 {
                let mut best_val = dovecote.get_hole(best_index);

                for _i in 1..self.comp {
                    let box_num = self.range.sample(&mut self.rng);
                    let val = dovecote.get_hole(box_num);
                    if val < best_val {
                        best_val = val;
                        best_index = box_num;
                    }
                }
            }

            dovecote.throw(best_index) // magia rustowa, jeśli zwróci false to pętla się kończy ( ten scope to tak naprawde expression )
        } {};
        return dovecote;
    }

    pub fn run_multiple(&mut self, times: usize) -> (f64, Vec<f64>, Vec<DoveCote>) {
        let mut results = Vec::new();
        let mut averages: Vec<f64> = vec![0.0; 5];
        // let mut empt_funcs = Vec::new();
        for _i in 0..times {
            let dovecote = self.run_sim();
            averages[0] += dovecote.first_collision as f64;
            averages[1] += dovecote.empty_boxes as f64;
            averages[2] += dovecote.all_one as f64;
            averages[3] += dovecote.all_two as f64;
            averages[4] += dovecote.max_doves as f64;

            // empt_funcs.push(dovecote.empty_boxes_fn.clone());
            results.push(dovecote);
        }

        for average in 0..averages.len() {
            averages[average] /= times as f64;
        }
        // let averagefn = revfn::average(empt_funcs);

        // returns tuple of size, averages, and results
        return (self.size as f64, averages, results);
    }

    // pub fn run_sim_optimized(&mut self) -> [usize;4] {
    //     let dovecote = self.run_sim();
    //     return [dovecote.first_collision, dovecote.empty_boxes, dovecote.all_one, dovecote.all_two]
    // }
    //
    // pub fn run_multiple_optimized(&mut self, times: usize) -> ([f64; 4], Vec<[usize; 4]>) {
    //     let mut results = Vec::new();
    //     let mut averages: [f64; 4] = [0.0; 4];
    //     // let mut empt_funcs = Vec::new();
    //     for _i in 0..times {
    //         let dovecote = self.run_sim_optimized();
    //         averages[0] += dovecote[0] as f64;
    //         averages[1] += dovecote[1] as f64;
    //         averages[2] += dovecote[2] as f64;
    //         averages[3] += dovecote[3] as f64;
    //
    //         // empt_funcs.push(dovecote.empty_boxes_fn.clone());
    //         results.push(dovecote);
    //     }
    //     averages[0] /= times as f64;
    //     averages[1] /= times as f64;
    //     averages[2] /= times as f64;
    //     averages[3] /= times as f64;
    //     // let averagefn = revfn::average(empt_funcs);
    //     return (averages, results);
    // }
}
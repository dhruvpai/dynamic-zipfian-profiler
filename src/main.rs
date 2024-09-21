use std::collections::HashMap;
use std::env;
use csv::ReaderBuilder;
use ordered_float::OrderedFloat;
use nalgebra::{DMatrix, DVector, Matrix3, Vector3};

#[allow(dead_code, non_snake_case)]

fn is_float(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

// enum to store datasets
pub enum data {
    num(Vec<f64>), // floats that need to be counted. may as well be string. example: keys
    string(Vec<String>), // string. example: moby dick
    ranked(Vec<f64>), // float that is already counted. example: earthquake dataset
    fileerror(String)
}

// csv -> data enum for usage in tester
fn process_csv(file_path: &str, column: usize, is_ranked: bool, is_str: bool) -> data {
    let mut builder = ReaderBuilder::new();
    builder.double_quote(false).comment(Some(b'*'));
    let result = builder.from_path(file_path);
    if result.is_err() {
        println!("Failed to read CSV. File path probably doesn't exist, or you don't have permissions.");
        std::process::exit(9);
    }

    let mut reader = result.unwrap();
    let mut float_data: Vec<f64> = Vec::new();
    let mut string_data: Vec<String> = Vec::new();
    println!("SETUP TO READ COMPLETE");

    let records = reader.records();
    for result in records {
        let record = match result {
            Ok(r) => r,
            Err(e) => return data::fileerror(e.to_string()),
        };
        if let Some(entry) = record.get(column) {
            if let Ok(number) = entry.parse::<f64>() {
                if number > 0.0 {
                    if is_str {
                        string_data.push(number.to_string());
                        //println!("{}", number.to_string());
                    }
                    else {
                        float_data.push(number);
                        //println!("{}", number.to_string());
                    }
                }
            }
            else {
                string_data.push(entry.to_string());
                //println!("{}", entry.to_string());
            }
        }
        else {
            println!("Error in file reading");
        }
    }
    println!("READ COMPLETE");
    let mut dataset = data::num(float_data.clone()); // this is a hack to make the compiler happy
    if float_data.len() == 0 && string_data.len() == 0 {
        println!("No data found in CSV file.");
        std::process::exit(9);
    }
    else if float_data.len() != 0 && is_ranked {
        println!("Pre-Counted Float data found in CSV file.");
        dataset = data::ranked(float_data);
    }
    else if float_data.len() != 0 && !is_ranked {
        println!("Uncounted Float data found in CSV file.");
        dataset = data::num(float_data);
    }
    else if string_data.len() != 0 {
        println!("String data found in CSV file.");
        dataset = data::string(string_data);
    }
    else {
        println!("ERROR");
    }
    dataset
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1]; // must be csv
    let column = (&args[2]).parse::<usize>().unwrap();
    let hyperparameter1 = (&args[3]).parse::<f64>().unwrap();
    let hyperparameter2 = (&args[4]).parse::<f64>().unwrap();

    let is_ranked = true; // FOR FLOAT BASED DATASETS: if the data needs to be counted to assign popularity ranks, select false. Else, assign true
    let is_str = false; // whether the data is string or not

    let dataset = process_csv(file_path, column, is_ranked, is_str);
    let zm = zm::new(1.0, 1.0, 1.0);
    let mut distribution = dist::new(1.0, 1.0, 1.0, 1.0, 1, 1.01, zm);
    distribution.zipf_tester(dataset, hyperparameter1, hyperparameter2);
}

impl data {
    fn length(&self) -> usize {
        match &self {
            data::num(dat) => {dat.len()}
            data::string(dat) => {dat.len()}
            data::ranked(dat) => {dat.len()}
            _ => {println!("Error in length: data type not recognized."); 0}
        }
    }

    // extracts empirical distribution from data
    fn get_samples(&self, start_index: usize, end_index: usize) -> Vec<f64> {
        let mut returned = Vec::new();
        match &self {
            data::num(_) => returned = self.get_counts(start_index, end_index),
            data::string(_) => returned = self.get_counts(start_index, end_index),
            data::ranked(dat) => returned = dat[start_index..end_index].to_vec(),
            _ => {println!("Error in get_samples: data type not recognized.");}
        }
        println!("made it out the get_samples match (GOOD)");
        returned.sort_by(|a, b| b.partial_cmp(a).unwrap());
        returned
    }

    // counts items and ranks by popularity
    fn get_counts(&self, start_index: usize, end_index: usize) -> Vec<f64> {
        let mut result = Vec::new();
        match &self {
            data::num(dat) => {
                let mut occurrences = HashMap::new();

                for item in dat[start_index..end_index].iter() {
                    *occurrences.entry(OrderedFloat(item.clone())).or_insert(0) += 1;
                }
                result = occurrences.into_iter().map(|(_, v)| v as f64 ).collect();
            }
            data::string(dat) => {
                let mut occurrences = HashMap::new();
                for item in dat[start_index..end_index].iter() {
                    *occurrences.entry(item.clone()).or_insert(0) += 1;
                }
                result = occurrences.into_iter().map(|(_, v)| v as f64 ).collect();
            }
            data::ranked(dat) => {result = dat.clone()}
            _ => {println!("Error in get_counts: data type not recognized.");}
        }
        result
    }
}

pub struct coherence {
    logrhist: Vec<f64>,
    Nhist: Vec<usize>,
    nhist: Vec<f64>,
    gammahist: Vec<f64>,
    coherencehist: Vec<bool>,
    qhist: Vec<f64>,
}

pub struct zm {
    gamma: f64,
    sbar: f64,
    q: f64,
}

pub struct dist {
    min: f64,
    max: f64,
    c: f64,
    n: f64,
    N: usize,
    alpha: f64,
    zm: zm,
}

impl dist {
    fn zipf_tester(&mut self, dataset: data, regularization_coefficient1: f64, regularization_coefficient2: f64) {
        //split dataset into 20 subsets: first 5%, first 10%, etc.
        let mut coherence = coherence::new();
        let mut drift = 0.0;
        //get length
        let len = dataset.length();

        let mut count = 0;
        for i in 1..=20 {
            println!();
            eprintln!("i: {}; samples length: {}", i, (i * len)/20);
            let samples = dataset.get_samples(0, (i * len)/20);
            self.N = samples.len();
            let initial_alpha = self.alpha;
            self.min = samples[self.N - 1];
            let initial_min = self.min;
            self.max = samples[0];
            let initial_max = self.max;
            let mut lambda = 0.001;
            let y = DVector::from_vec(samples.clone().iter().map(|sample| sample.log10()).collect());
            for _ in 0..100 {
                let f = DVector::from_fn(self.N, |i, _| self.f(i + 1));
                let r = &y - f;
                let J = DMatrix::from_fn(self.N, 3, |i, j| if j == 0 {self.df_da(i + 1)} else if j == 1 {self.df_dsm(i + 1)} else {self.df_dsM(i + 1)});
                let I = Matrix3::<f64>::identity();
                let regularization_coefficients_matrix = Matrix3::<f64>::from_diagonal(&Vector3::new(0.0, regularization_coefficient1, regularization_coefficient2));
                let delta = (J.transpose() * &J + regularization_coefficients_matrix + lambda * I).try_inverse().unwrap_or(Matrix3::zeros()) * (J.transpose() * &r + regularization_coefficients_matrix * Vector3::new(initial_alpha - self.alpha, initial_min - self.min, initial_max - self.max));
                if delta.norm() < 0.000001 {
                    break;
                }
                self.alpha += delta[0];
                let prev_min = self.min;
                self.min += delta[1];
                let prev_max = self.max;
                self.max += delta[2];
                let new_f = DVector::from_fn(self.N, |i, _| self.f(i + 1));
                let new_r = &y - new_f;
                if new_r.norm_squared() + regularization_coefficient1 * (self.min - initial_min).powf(2.0) + regularization_coefficient2 * (self.max - initial_max).powf(2.0) < r.norm_squared() + regularization_coefficient1 * (prev_min - initial_min).powf(2.0) + regularization_coefficient2 * (prev_max - initial_max).powf(2.0) {
                    lambda /= 10.0;
                }
                else {
                    lambda *= 10.0;
                    self.alpha -= delta[0];
                    self.min -= delta[1];
                    self.max -= delta[2];
                }
            }
            // print all parameters
            println!("Alpha: {}; Min: {}; Max: {}", self.alpha, self.min, self.max);
                self.update_zm();
                self.update_c();
                self.update_n();
                coherence.update_logrhist(self.logr());
                coherence.update_Nhist(self.N);
                coherence.update_nhist(self.n);
                coherence.update_gammahist(self.zm.gamma);
                coherence.update_qhist(self.zm.q);
            println!("Q: {}; n: {}", self.zm.q, self.n);
            if i > 1 {
                if ((coherence.qhist[i - 1] - coherence.qhist[(i - 1) - 1]) / (coherence.nhist[i - 1] - coherence.nhist[(i - 1) - 1]) <= 0.0) {
                    count += 1;
                    coherence.update_coherencehist(true);
                    println!("dQ/dn <= 0");
                }
                else {
                    coherence.update_coherencehist(false);
                    println!("dQ/dn > 0");
                }
            }
            drift += (self.logr()).powf(2.0);
            println!("S(k) = {}/((k + {})^{}); P(S) = 0 for S < {}, P(S) = {}/(S^{}) for {} <= S <= {}, and P(S) = 0 for S > {}; S(1) + ... + S({}) = {}; v = {}", self.zm.sbar, self.zm.q, self.zm.gamma, self.min, self.c, self.alpha, self.min, self.max, self.max, self.N, self.n, drift);
        }
        println!();
        println!("dQ/dn <= 0: {}/19", count);
    }

    fn update_c(&mut self) {
        self.c = (self.alpha - 1.0)/(self.min.powf(1.0 - self.alpha) - self.max.powf(1.0 - self.alpha));
    }

    fn update_zm(&mut self) {
        self.zm.update_gamma(self.alpha);
        self.zm.update_sbar(self.N, self.min);
        self.zm.update_q(self.N, self.min, self.max);
    }

    fn pdf(&self, x: f64) -> f64 {
        if x < self.min || x > self.max {
            return 0.0;
        }
        else {
            return self.c / x.powf(self.alpha);
        }
    }

    fn logr(&self) -> f64 {
        (self.min / self.max).log10()
    }

    fn f(&self, k: usize) -> f64 {
        let a = self.alpha;
        let N = self.N;
        let sm = self.min;
        let sM = self.max;
        (-1.0 / (a - 1.0)) * ((k as f64) + (N as f64) * (sm / sM).powf(a - 1.0)).log10() + (1.0 / (a - 1.0)) * (N as f64).log10() + sm.log10()
    }

    fn df_da(&self, k: usize) -> f64 {
        let a = self.alpha;
        let N = self.N;
        let sm = self.min;
        let sM = self.max;
        (1.0 / (a - 1.0).powf(2.0)) * ((k as f64) + (N as f64) * (sm / sM).powf(a - 1.0)).log10() + (-1.0 / (a - 1.0)) * (1.0 / (10 as f64).ln()) * (1.0 / ((k as f64) + (N as f64) * (sm / sM).powf(a - 1.0))) * ((N as f64) * (sm / sM).powf(a - 1.0) * (sm / sM).ln()) + (N as f64).log10() * (-1.0 / (a - 1.0).powf(2.0))
    }

    fn df_dsm(&self, k: usize) -> f64 {
        let a = self.alpha;
        let N = self.N;
        let sm = self.min;
        let sM = self.max;
        (-1.0 / (a - 1.0)) * (1.0 / (10 as f64).ln()) * (1.0 / ((k as f64) + (N as f64) * (sm / sM).powf(a - 1.0))) * ((N as f64) * (((a - 1.0) * sm.powf(a - 2.0)) / sM.powf(a - 1.0))) + (1.0 / (10 as f64).ln()) * (1.0 / sm)
    }

    fn df_dsM(&self, k: usize) -> f64 {
        let a = self.alpha;
        let N = self.N;
        let sm = self.min;
        let sM = self.max;
        (-1.0 / (a - 1.0)) * (1.0 / (10 as f64).ln()) * (1.0 / ((k as f64) + (N as f64) * (sm / sM).powf(a - 1.0))) * ((N as f64) * (sm.powf(a - 1.0) * (1.0 - a) / sM.powf(a)))
    }

    fn new(min: f64, max: f64, c: f64, n: f64, N: usize, alpha: f64, zm: zm) -> dist {
        dist {
            min: min,
            max: max,
            c: c,
            n: n,
            N: N,
            alpha: alpha,
            zm: zm,
        }
    }

    fn update_n(&mut self) {
        self.n = 0.0;
        for k in 1..=self.N {
            self.n += (10 as f64).powf(self.f(k));
        }
    }
}

impl zm {
    fn update_gamma(&mut self, alpha: f64) {
        self.gamma = 1.0 / (alpha - 1.0);
    }

    fn update_sbar(&mut self, N: usize, sm: f64) {
        self.sbar = (N as f64).powf(self.gamma) * sm
    }

    fn update_q(&mut self, N: usize, sm: f64, sM: f64) {
        self.q = (N as f64) * ((sm / sM).powf(1.0 / self.gamma))
    }

    fn new(gamma: f64, sbar: f64, q: f64) -> zm {
        zm {
            gamma: gamma,
            sbar: sbar,
            q: q,
        }
    }
}

impl coherence {
    fn update_logrhist(&mut self, logr: f64) {
        self.logrhist.push(logr);
    }

    fn update_Nhist(&mut self, N: usize) {
        self.Nhist.push(N);
    }

    fn update_nhist(&mut self, n: f64) {
        self.nhist.push(n);
    }

    fn update_gammahist(&mut self, gamma: f64) {
        self.gammahist.push(gamma);
    }

    fn update_coherencehist(&mut self, coherence: bool) {
        self.coherencehist.push(coherence);
    }

    fn update_qhist(&mut self, q: f64) {
        self.qhist.push(q);
    }

    fn new() -> coherence {
        coherence {
            logrhist: Vec::new(),
            Nhist: Vec::new(),
            nhist: Vec::new(),
            gammahist: Vec::new(),
            coherencehist: Vec::new(),
            qhist: Vec::new(),
        }
    }
}

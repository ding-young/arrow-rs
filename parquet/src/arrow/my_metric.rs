// use std::{fmt::Display, time::Duration};
// use once_cell::sync::Lazy;
// use std::sync::atomic::{AtomicU64, Ordering};

// #[derive(Default, Debug)]
// pub struct MyMetrics {
//     pub decode_time: Duration,
//     pub decompress_time: Duration,
//     pub parquet_decode_time: Duration,
//     pub parquet_metadata_time: Duration,
// }

// pub static MYMETRICS: Lazy<MyMetrics> = Lazy::new(MyMetrics::default);

// impl MyMetrics {
//     pub fn add_decompress_time(&mut self, duration: Duration) {
//         self.decompress_time += duration;
//     }

//     pub fn add_decode_time(&mut self, duration: Duration) {
//         self.decode_time += duration;
//     }
    
//     pub fn add_parquet_decode_time(&mut self, duration: Duration) {
//         self.parquet_decode_time += duration;
//     }
    
//     pub fn add_parquet_metadata_time(&mut self, duration: Duration) {
//         self.parquet_metadata_time += duration;
//     }
// }

impl Display for MyMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decode: {:?}\nDecompress: {:?}\nParquetDecode: {:?}\nMetadata: {:?}", self.decode_time, self.decompress_time, self.parquet_decode_time, self.parquet_metadata_time)
    }
}
use std::{
    fmt::Display,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use once_cell::sync::Lazy;

pub struct MyMetrics {
    decode_time: AtomicU64,
    decompress_time: AtomicU64,
    parquet_decode_time: AtomicU64,
    parquet_metadata_time: AtomicU64,
}

impl MyMetrics {
    pub fn add_decompress_time(&self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;
        self.decompress_time.fetch_add(nanos, Ordering::Relaxed);
    }

    pub fn add_decode_time(&self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;
        self.decode_time.fetch_add(nanos, Ordering::Relaxed);
    }

    pub fn add_parquet_decode_time(&self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;
        self.parquet_decode_time.fetch_add(nanos, Ordering::Relaxed);
    }

    pub fn add_parquet_metadata_time(&self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;
        self.parquet_metadata_time.fetch_add(nanos, Ordering::Relaxed);
    }

    pub fn get(&self) -> DisplayableMetrics {
        DisplayableMetrics {
            decode_time: Duration::from_nanos(self.decode_time.load(Ordering::Relaxed)),
            decompress_time: Duration::from_nanos(self.decompress_time.load(Ordering::Relaxed)),
            parquet_decode_time: Duration::from_nanos(self.parquet_decode_time.load(Ordering::Relaxed)),
            parquet_metadata_time: Duration::from_nanos(self.parquet_metadata_time.load(Ordering::Relaxed)),
        }
    }
}

impl Default for MyMetrics {
    fn default() -> Self {
        Self {
            decode_time: AtomicU64::new(0),
            decompress_time: AtomicU64::new(0),
            parquet_decode_time: AtomicU64::new(0),
            parquet_metadata_time: AtomicU64::new(0),
        }
    }
}

pub static MYMETRICS: Lazy<MyMetrics> = Lazy::new(MyMetrics::default);

pub struct DisplayableMetrics {
    pub decode_time: Duration,
    pub decompress_time: Duration,
    pub parquet_decode_time: Duration,
    pub parquet_metadata_time: Duration,
}

impl Display for DisplayableMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Decode: {:?}\nDecompress: {:?}\nParquetDecode: {:?}\nMetadata: {:?}",
            self.decode_time,
            self.decompress_time,
            self.parquet_decode_time,
            self.parquet_metadata_time,
        )
    }
}

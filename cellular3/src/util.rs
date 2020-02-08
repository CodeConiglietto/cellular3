use std::{
    path::{Path, PathBuf},
    sync::Mutex,
    time::SystemTime,
};

use lazy_static::lazy_static;
use rand::{RngCore, SeedableRng};
use walkdir::WalkDir;

pub fn collect_filenames<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut vec: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| {
            e.ok().and_then(|e| {
                if e.file_type().is_file() {
                    Some(e.path().to_owned())
                } else {
                    None
                }
            })
        })
        .collect();

    vec.sort();

    vec
}

lazy_static! {
    pub static ref RNG_SEED: Mutex<u128> =
        Mutex::new(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis());
}

#[cfg(target_pointer_width = "64")]
type DeterministicRngImpl = rand_pcg::Pcg64Mcg;

#[cfg(target_pointer_width = "32")]
type DeterministicRngImpl = rand_pcg::Pcg32;

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Only 32 and 64 bit systems supported");

pub struct DeterministicRng {
    rng: DeterministicRngImpl,
}

impl RngCore for DeterministicRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

impl SeedableRng for DeterministicRng {
    type Seed = <DeterministicRngImpl as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            rng: DeterministicRngImpl::from_seed(seed),
        }
    }
}

impl DeterministicRng {
    pub fn new() -> Self {
        let seed = *RNG_SEED.lock().unwrap();
        println!("Initializing RNG with seed {}", seed);
        Self::from_seed(seed.to_le_bytes())
    }
}

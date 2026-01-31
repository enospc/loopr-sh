use crate::{LooprError, LooprResult};

const REPO_ID_LENGTH: usize = 6;
const REPO_ID_ALPHABET: &str = "useandom26T198340PX75pxJACKVERYMINDBUSHWOLFGQZbfghjklqvwyzrict";

pub trait RandomSource {
    fn fill(&mut self, buf: &mut [u8]) -> LooprResult<()>;
}

pub struct OsRandom;

impl RandomSource for OsRandom {
    fn fill(&mut self, buf: &mut [u8]) -> LooprResult<()> {
        getrandom::fill(buf).map_err(|err| LooprError::new(format!("random bytes: {}", err)))?;
        Ok(())
    }
}

pub fn generate_nanoid(source: &mut dyn RandomSource, length: usize) -> LooprResult<String> {
    if length == 0 {
        return Err(LooprError::new("nanoid length must be positive"));
    }
    let alphabet = REPO_ID_ALPHABET.as_bytes();
    let mut output = String::with_capacity(length);
    let mut buf = vec![0u8; length];
    while output.len() < length {
        let needed = length - output.len();
        let take = needed.min(buf.len());
        source.fill(&mut buf[..take])?;
        for byte in &buf[..take] {
            let idx = (byte & 63) as usize;
            if idx >= alphabet.len() {
                continue;
            }
            output.push(alphabet[idx] as char);
            if output.len() == length {
                break;
            }
        }
    }
    Ok(output)
}

pub fn repo_id_length() -> usize {
    REPO_ID_LENGTH
}

pub fn repo_id_alphabet() -> &'static str {
    REPO_ID_ALPHABET
}

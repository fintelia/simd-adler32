pub mod avx2;
pub mod scalar;
pub mod sse2;

pub type Adler32Imp = fn(u16, u16, &[u8]) -> (u16, u16);

pub fn get_imp() -> Adler32Imp {
  avx2::get_imp().unwrap_or(scalar::update)
}

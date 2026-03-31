// NOTE: this might be redundant.

pub struct Octet
{
  data: u8
}

impl Octet
{
  pub fn new(byte: u8) -> Self { return Octet { data: byte }; }

  pub fn set_bit(&mut self, index: usize, boolean: bool)
  {
    let flag = 1 << index;
    if boolean { self.data |= flag; }
    else { self.data &= !flag; }
  }

  pub fn get_bit(&mut self, index: usize) -> bool
  {
    return (self.data >> index & 0x0000_0001) == 0x0000_0001;
  }
}

use crate::mac::MAC;

#[derive(Default)]
pub struct MACValidator
{
  last_mac: MAC,
}

impl MACValidator
{
  pub fn validate(&mut self, new_mac: MAC) -> Result<(), String>
  {
    // Extract to local variables.
    let last_mac = self.last_mac;

    // Update state.
    self.last_mac = new_mac;

    // Validate.
    if new_mac == last_mac { return Ok(()); }
    else
    { return Err(format!("MAC address changed. Old: {last_mac}, new: {new_mac}").to_string()); }
  }
}

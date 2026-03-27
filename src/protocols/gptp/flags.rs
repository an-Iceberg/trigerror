pub enum Flags
{
  AlternateMaster =       0b00000000_00000000,
  TwoStep =               0b00000001_00000000,
  Unicast =               0b00000010_00000000,
  PTPProfileSpecific1 =   0b00100000_00000000,
  PTPProfileSpecific2 =   0b01000000_00000000,
  Leap61 =                0b00000000_00000001,
  Leap59 =                0b00000000_00000010,
  CurrentUTCOffsetValid = 0b00000000_00000100,
  PTPTimescale =          0b00000000_00001000,
  TimeTraceable =         0b00000000_00010000,
  FrequencyTraceable =    0b00000000_00100000,
  Reserved,
}

// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod idl {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_FRAME_TYPE: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_FRAME_TYPE: i16 = 101;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_FRAME_TYPE: [FrameType; 4] = [
  FrameType::Message,
  FrameType::MessageAck,
  FrameType::OnLine,
  FrameType::OnLineAck,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FrameType(pub i16);
#[allow(non_upper_case_globals)]
impl FrameType {
  pub const Message: Self = Self(0);
  pub const MessageAck: Self = Self(1);
  pub const OnLine: Self = Self(100);
  pub const OnLineAck: Self = Self(101);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 101;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Message,
    Self::MessageAck,
    Self::OnLine,
    Self::OnLineAck,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Message => Some("Message"),
      Self::MessageAck => Some("MessageAck"),
      Self::OnLine => Some("OnLine"),
      Self::OnLineAck => Some("OnLineAck"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for FrameType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for FrameType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<i16>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for FrameType {
    type Output = FrameType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<i16>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for FrameType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = i16::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = i16::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for FrameType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for FrameType {}
// struct Int128, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Int128(pub [u8; 8]);
impl Default for Int128 { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl std::fmt::Debug for Int128 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Int128")
      .field("i1", &self.i1())
      .field("i2", &self.i2())
      .field("i3", &self.i3())
      .field("i4", &self.i4())
      .field("i5", &self.i5())
      .field("i6", &self.i6())
      .field("i7", &self.i7())
      .field("i8_", &self.i8_())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Int128 {}
impl flatbuffers::SafeSliceAccess for Int128 {}
impl<'a> flatbuffers::Follow<'a> for Int128 {
  type Inner = &'a Int128;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Int128>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Int128 {
  type Inner = &'a Int128;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Int128>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Int128 {
    type Output = Int128;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Int128 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Int128 {
    type Output = Int128;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Int128 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Int128 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Int128 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    i1: i8,
    i2: i8,
    i3: i8,
    i4: i8,
    i5: i8,
    i6: i8,
    i7: i8,
    i8_: i8,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_i1(i1);
    s.set_i2(i2);
    s.set_i3(i3);
    s.set_i4(i4);
    s.set_i5(i5);
    s.set_i6(i6);
    s.set_i7(i7);
    s.set_i8_(i8_);
    s
  }

  pub fn i1(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i1(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i2(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[1..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i2(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[1..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i3(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[2..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i3(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[2..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i4(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[3..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i4(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[3..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i5(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i5(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i6(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[5..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i6(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[5..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i7(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[6..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i7(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[6..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn i8_(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[7..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_i8_(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[7..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

}

// struct Timestamp, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Timestamp(pub [u8; 8]);
impl Default for Timestamp { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl std::fmt::Debug for Timestamp {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Timestamp")
      .field("ts", &self.ts())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Timestamp {}
impl flatbuffers::SafeSliceAccess for Timestamp {}
impl<'a> flatbuffers::Follow<'a> for Timestamp {
  type Inner = &'a Timestamp;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Timestamp>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Timestamp {
  type Inner = &'a Timestamp;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Timestamp>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Timestamp {
    type Output = Timestamp;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Timestamp as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Timestamp {
    type Output = Timestamp;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Timestamp as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Timestamp {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Timestamp {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    ts: i64,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_ts(ts);
    s
  }

  pub fn ts(&self) -> i64 {
    let mut mem = core::mem::MaybeUninit::<i64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_ts(&mut self, x: i64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i64 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<i64>(),
      );
    }
  }

}

// struct Header, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Header(pub [u8; 32]);
impl Default for Header { 
  fn default() -> Self { 
    Self([0; 32])
  }
}
impl std::fmt::Debug for Header {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Header")
      .field("len", &self.len())
      .field("type_", &self.type_())
      .field("version", &self.version())
      .field("from_id", &self.from_id())
      .field("to_id", &self.to_id())
      .field("forward_id", &self.forward_id())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Header {}
impl flatbuffers::SafeSliceAccess for Header {}
impl<'a> flatbuffers::Follow<'a> for Header {
  type Inner = &'a Header;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Header>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Header {
  type Inner = &'a Header;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Header>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Header {
    type Output = Header;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Header as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Header {
    type Output = Header;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Header as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Header {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Header {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    len: i32,
    type_: FrameType,
    version: i16,
    from_id: &Int128,
    to_id: &Int128,
    forward_id: &Int128,
  ) -> Self {
    let mut s = Self([0; 32]);
    s.set_len(len);
    s.set_type_(type_);
    s.set_version(version);
    s.set_from_id(&from_id);
    s.set_to_id(&to_id);
    s.set_forward_id(&forward_id);
    s
  }

  pub fn len(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<i32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_len(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn type_(&self) -> FrameType {
    let mut mem = core::mem::MaybeUninit::<FrameType>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<FrameType>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_type_(&mut self, x: FrameType) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const FrameType as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<FrameType>(),
      );
    }
  }

  pub fn version(&self) -> i16 {
    let mut mem = core::mem::MaybeUninit::<i16>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[6..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i16>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_version(&mut self, x: i16) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i16 as *const u8,
        self.0[6..].as_mut_ptr(),
        core::mem::size_of::<i16>(),
      );
    }
  }

  pub fn from_id(&self) -> &Int128 {
    unsafe { &*(self.0[8..].as_ptr() as *const Int128) }
  }

  pub fn set_from_id(&mut self, x: &Int128) {
    self.0[8..8+8].copy_from_slice(&x.0)
  }

  pub fn to_id(&self) -> &Int128 {
    unsafe { &*(self.0[16..].as_ptr() as *const Int128) }
  }

  pub fn set_to_id(&mut self, x: &Int128) {
    self.0[16..16+8].copy_from_slice(&x.0)
  }

  pub fn forward_id(&self) -> &Int128 {
    unsafe { &*(self.0[24..].as_ptr() as *const Int128) }
  }

  pub fn set_forward_id(&mut self, x: &Int128) {
    self.0[24..24+8].copy_from_slice(&x.0)
  }

}

pub enum FrameOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Frame<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Frame<'a> {
    type Inner = Frame<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Frame<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Frame { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args FrameArgs<'args>) -> flatbuffers::WIPOffset<Frame<'bldr>> {
      let mut builder = FrameBuilder::new(_fbb);
      if let Some(x) = args.bytes { builder.add_bytes(x); }
      if let Some(x) = args.header { builder.add_header(x); }
      builder.finish()
    }

    pub const VT_HEADER: flatbuffers::VOffsetT = 4;
    pub const VT_BYTES: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn header(&self) -> Option<&'a Header> {
    self._tab.get::<Header>(Frame::VT_HEADER, None)
  }
  #[inline]
  pub fn bytes(&self) -> Option<&'a [i8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i8>>>(Frame::VT_BYTES, None).map(|v| v.safe_slice())
  }
}

impl flatbuffers::Verifiable for Frame<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Header>(&"header", Self::VT_HEADER, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i8>>>(&"bytes", Self::VT_BYTES, false)?
     .finish();
    Ok(())
  }
}
pub struct FrameArgs<'a> {
    pub header: Option<&'a Header>,
    pub bytes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i8>>>,
}
impl<'a> Default for FrameArgs<'a> {
    #[inline]
    fn default() -> Self {
        FrameArgs {
            header: None,
            bytes: None,
        }
    }
}
pub struct FrameBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FrameBuilder<'a, 'b> {
  #[inline]
  pub fn add_header(&mut self, header: &Header) {
    self.fbb_.push_slot_always::<&Header>(Frame::VT_HEADER, header);
  }
  #[inline]
  pub fn add_bytes(&mut self, bytes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Frame::VT_BYTES, bytes);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FrameBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FrameBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Frame<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Frame<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Frame");
      ds.field("header", &self.header());
      ds.field("bytes", &self.bytes());
      ds.finish()
  }
}
pub enum MessageBodyOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct MessageBody<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MessageBody<'a> {
    type Inner = MessageBody<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> MessageBody<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        MessageBody { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageBodyArgs<'args>) -> flatbuffers::WIPOffset<MessageBody<'bldr>> {
      let mut builder = MessageBodyBuilder::new(_fbb);
      if let Some(x) = args.text { builder.add_text(x); }
      if let Some(x) = args.ts { builder.add_ts(x); }
      if let Some(x) = args.from_id { builder.add_from_id(x); }
      if let Some(x) = args.id { builder.add_id(x); }
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_FROM_ID: flatbuffers::VOffsetT = 6;
    pub const VT_TS: flatbuffers::VOffsetT = 8;
    pub const VT_TEXT: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn id(&self) -> Option<&'a Int128> {
    self._tab.get::<Int128>(MessageBody::VT_ID, None)
  }
  #[inline]
  pub fn from_id(&self) -> Option<&'a Int128> {
    self._tab.get::<Int128>(MessageBody::VT_FROM_ID, None)
  }
  #[inline]
  pub fn ts(&self) -> Option<&'a Timestamp> {
    self._tab.get::<Timestamp>(MessageBody::VT_TS, None)
  }
  #[inline]
  pub fn text(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MessageBody::VT_TEXT, None)
  }
}

impl flatbuffers::Verifiable for MessageBody<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Int128>(&"id", Self::VT_ID, false)?
     .visit_field::<Int128>(&"from_id", Self::VT_FROM_ID, false)?
     .visit_field::<Timestamp>(&"ts", Self::VT_TS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"text", Self::VT_TEXT, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageBodyArgs<'a> {
    pub id: Option<&'a Int128>,
    pub from_id: Option<&'a Int128>,
    pub ts: Option<&'a Timestamp>,
    pub text: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for MessageBodyArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageBodyArgs {
            id: None,
            from_id: None,
            ts: None,
            text: None,
        }
    }
}
pub struct MessageBodyBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBodyBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: &Int128) {
    self.fbb_.push_slot_always::<&Int128>(MessageBody::VT_ID, id);
  }
  #[inline]
  pub fn add_from_id(&mut self, from_id: &Int128) {
    self.fbb_.push_slot_always::<&Int128>(MessageBody::VT_FROM_ID, from_id);
  }
  #[inline]
  pub fn add_ts(&mut self, ts: &Timestamp) {
    self.fbb_.push_slot_always::<&Timestamp>(MessageBody::VT_TS, ts);
  }
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MessageBody::VT_TEXT, text);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBodyBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBodyBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MessageBody<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for MessageBody<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("MessageBody");
      ds.field("id", &self.id());
      ds.field("from_id", &self.from_id());
      ds.field("ts", &self.ts());
      ds.field("text", &self.text());
      ds.finish()
  }
}
pub enum MessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      if let Some(x) = args.body { builder.add_body(x); }
      if let Some(x) = args.header { builder.add_header(x); }
      builder.finish()
    }

    pub const VT_HEADER: flatbuffers::VOffsetT = 4;
    pub const VT_BODY: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn header(&self) -> Option<&'a Header> {
    self._tab.get::<Header>(Message::VT_HEADER, None)
  }
  #[inline]
  pub fn body(&self) -> Option<MessageBody<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<MessageBody>>(Message::VT_BODY, None)
  }
}

impl flatbuffers::Verifiable for Message<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Header>(&"header", Self::VT_HEADER, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<MessageBody>>(&"body", Self::VT_BODY, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageArgs<'a> {
    pub header: Option<&'a Header>,
    pub body: Option<flatbuffers::WIPOffset<MessageBody<'a>>>,
}
impl<'a> Default for MessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageArgs {
            header: None,
            body: None,
        }
    }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_header(&mut self, header: &Header) {
    self.fbb_.push_slot_always::<&Header>(Message::VT_HEADER, header);
  }
  #[inline]
  pub fn add_body(&mut self, body: flatbuffers::WIPOffset<MessageBody<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<MessageBody>>(Message::VT_BODY, body);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Message<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Message");
      ds.field("header", &self.header());
      ds.field("body", &self.body());
      ds.finish()
  }
}
pub enum MessageAckOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct MessageAck<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MessageAck<'a> {
    type Inner = MessageAck<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> MessageAck<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        MessageAck { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageAckArgs<'args>) -> flatbuffers::WIPOffset<MessageAck<'bldr>> {
      let mut builder = MessageAckBuilder::new(_fbb);
      if let Some(x) = args.ts { builder.add_ts(x); }
      if let Some(x) = args.id { builder.add_id(x); }
      if let Some(x) = args.header { builder.add_header(x); }
      builder.finish()
    }

    pub const VT_HEADER: flatbuffers::VOffsetT = 4;
    pub const VT_ID: flatbuffers::VOffsetT = 6;
    pub const VT_TS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn header(&self) -> Option<&'a Header> {
    self._tab.get::<Header>(MessageAck::VT_HEADER, None)
  }
  #[inline]
  pub fn id(&self) -> Option<&'a Int128> {
    self._tab.get::<Int128>(MessageAck::VT_ID, None)
  }
  #[inline]
  pub fn ts(&self) -> Option<&'a Timestamp> {
    self._tab.get::<Timestamp>(MessageAck::VT_TS, None)
  }
}

impl flatbuffers::Verifiable for MessageAck<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Header>(&"header", Self::VT_HEADER, false)?
     .visit_field::<Int128>(&"id", Self::VT_ID, false)?
     .visit_field::<Timestamp>(&"ts", Self::VT_TS, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageAckArgs<'a> {
    pub header: Option<&'a Header>,
    pub id: Option<&'a Int128>,
    pub ts: Option<&'a Timestamp>,
}
impl<'a> Default for MessageAckArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageAckArgs {
            header: None,
            id: None,
            ts: None,
        }
    }
}
pub struct MessageAckBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageAckBuilder<'a, 'b> {
  #[inline]
  pub fn add_header(&mut self, header: &Header) {
    self.fbb_.push_slot_always::<&Header>(MessageAck::VT_HEADER, header);
  }
  #[inline]
  pub fn add_id(&mut self, id: &Int128) {
    self.fbb_.push_slot_always::<&Int128>(MessageAck::VT_ID, id);
  }
  #[inline]
  pub fn add_ts(&mut self, ts: &Timestamp) {
    self.fbb_.push_slot_always::<&Timestamp>(MessageAck::VT_TS, ts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageAckBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageAckBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MessageAck<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for MessageAck<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("MessageAck");
      ds.field("header", &self.header());
      ds.field("id", &self.id());
      ds.field("ts", &self.ts());
      ds.finish()
  }
}
}  // pub mod idl


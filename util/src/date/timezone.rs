type Timezone = (i8, u8);

#[cfg(any(feature = "Shanghai"))]
pub const TIMEZONE: Timezone = (8, 0);


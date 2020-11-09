use florp_engine::WorldPos;

// -----------------------------------------------------------------------------
//     - Impl entity -
// -----------------------------------------------------------------------------
macro_rules! impl_entity {
    ($tile:tt, $pixel:expr) => {
        #[derive(Debug)]
        pub struct $tile(pub WorldPos);

        impl $tile {
            pub fn pixel(&self) -> char {
                $pixel
            }

            pub fn position(&self) -> WorldPos {
                self.0
            }
        }
    }
}

// -----------------------------------------------------------------------------
//     - Entities -
// -----------------------------------------------------------------------------
impl_entity!(Character, '@');
impl_entity!(Wall, '#');
impl_entity!(Gravel, '.');

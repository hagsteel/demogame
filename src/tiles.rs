use florp_engine::Point;

// -----------------------------------------------------------------------------
//     - Impl entity -
// -----------------------------------------------------------------------------
macro_rules! impl_entity {
    ($tile:tt, $pixel:expr) => {
        #[derive(Debug)]
        pub struct $tile(pub Point);

        impl $tile {
            pub fn pixel(&self) -> char {
                $pixel
            }

            pub fn position(&self) -> Point {
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

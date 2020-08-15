//! # Art
//!
//! A library for modeling artistic concepts.

// 导出内嵌模块的公共构造 相当于重新设计API的门面 或者给你一个我领土的地图目录
//pub use statements to re-export the items at the top level,

// Creating a useful public API structure is more of an art than a science, and you can iterate
// to find the API that works best for your users. Choosing pub use gives you flexibility in how you structure your crate internally and decouples that internal structure from what you present to your users.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;



pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
//    use crate::kinds::*; // NOTE 这里跟原来的差异 此处不是作为单独的crate来发布的 内嵌成员而已 crate==> super
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Green
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

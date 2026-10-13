//! The material of the workpiece: a short list (wood in three colours, metal, plastic with a colour of your choice) and
//! the thickness of the stock. It is what the 3D view shows and it is saved in the project.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialKind {
    WoodLight,
    WoodMedium,
    WoodDark,
    Metal,
    Plastic,
}

impl MaterialKind {
    pub const ALL: [MaterialKind; 5] = [MaterialKind::WoodLight, MaterialKind::WoodMedium, MaterialKind::WoodDark, MaterialKind::Metal, MaterialKind::Plastic];

    pub fn label(self) -> &'static str {
        match self {
            MaterialKind::WoodLight => "Wood, light",
            MaterialKind::WoodMedium => "Wood, medium",
            MaterialKind::WoodDark => "Wood, dark",
            MaterialKind::Metal => "Metal",
            MaterialKind::Plastic => "Plastic",
        }
    }

    /// The colour the material starts with.
    pub fn color(self) -> [u8; 3] {
        match self {
            MaterialKind::WoodLight => [0xe3, 0xc5, 0x8f],
            MaterialKind::WoodMedium => [0xb0, 0x7a, 0x45],
            MaterialKind::WoodDark => [0x5e, 0x3b, 0x24],
            MaterialKind::Metal => [0xa9, 0xb0, 0xb8],
            MaterialKind::Plastic => [0xf2, 0xf2, 0xf2],
        }
    }

    /// Only the colour of a plastic is yours to choose.
    pub fn colour_is_free(self) -> bool {
        self == MaterialKind::Plastic
    }
}

/// The workpiece: the material, its colour and the thickness of the stock (mm).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stock {
    pub kind: MaterialKind,
    pub color: [u8; 3],
    pub thickness: f64,
}

impl Default for Stock {
    fn default() -> Stock {
        Stock::of(MaterialKind::WoodLight)
    }
}

pub const MAX_THICKNESS: f64 = 500.0;

impl Stock {
    pub fn of(kind: MaterialKind) -> Stock {
        Stock { kind, color: kind.color(), thickness: 10.0 }
    }

    /// Choose another material: its own colour comes with it (a plastic keeps the colour it has).
    pub fn set_kind(&mut self, kind: MaterialKind) {
        if !(kind.colour_is_free() && self.kind.colour_is_free()) {
            self.color = kind.color();
        }
        self.kind = kind;
    }

    /// A stock made safe after it was read from a file.
    pub fn sane(mut self) -> Stock {
        if !self.thickness.is_finite() || self.thickness <= 0.0 {
            self.thickness = 10.0;
        }
        self.thickness = self.thickness.min(MAX_THICKNESS);
        if !self.kind.colour_is_free() {
            self.color = self.kind.color();
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_has_wood_in_three_colours_metal_and_plastic() {
        let wood = MaterialKind::ALL.iter().filter(|k| k.label().starts_with("Wood")).count();
        assert_eq!(wood, 3);
        let colours: std::collections::HashSet<_> = MaterialKind::ALL.iter().map(|k| k.color()).collect();
        assert_eq!(colours.len(), 5, "every material looks different");
    }

    #[test]
    fn only_a_plastic_takes_another_colour() {
        let mut s = Stock::of(MaterialKind::Plastic);
        s.color = [10, 200, 30];
        s.set_kind(MaterialKind::Plastic);
        assert_eq!(s.color, [10, 200, 30]);
        s.set_kind(MaterialKind::Metal);
        assert_eq!(s.color, MaterialKind::Metal.color());
        // A colour written into the file for a wood is ignored.
        let wood = Stock { kind: MaterialKind::WoodDark, color: [1, 2, 3], thickness: f64::NAN }.sane();
        assert_eq!((wood.color, wood.thickness), (MaterialKind::WoodDark.color(), 10.0));
        let plastic = Stock { kind: MaterialKind::Plastic, color: [1, 2, 3], thickness: 1e9 }.sane();
        assert_eq!((plastic.color, plastic.thickness), ([1, 2, 3], MAX_THICKNESS));
    }
}

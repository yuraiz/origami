use gtk::glib::bitflags::bitflags;

bitflags! {
    pub(crate) struct PositionFlags: u32 {
        const NONE = 0;
        const INSIDE = 0b000010000;

        const TOP = 0b00000001;
        const LEFT = 0b00000010;
        const RIGHT = 0b00000100;
        const BOTTOM = 0b000001000;

        const TOP_LEFT = Self::TOP.bits | Self::LEFT.bits;
        const TOP_RIGHT = Self::TOP.bits | Self::RIGHT.bits;
        const BOTTOM_RIGHT = Self::BOTTOM.bits | Self::RIGHT.bits;
        const BOTTOM_LEFT = Self::BOTTOM.bits | Self::LEFT.bits;

        const FULL_WIDTH = Self::LEFT.bits | Self::RIGHT.bits;
        const FULL_HEIGHT = Self::TOP.bits | Self::BOTTOM.bits;

        const FULL = Self::FULL_WIDTH.bits | Self::FULL_HEIGHT.bits;
    }
}

impl Default for PositionFlags {
    fn default() -> Self {
        Self::NONE
    }
}

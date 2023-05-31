//! [Paper Plane](https://github.com/paper-plane-developers/paper-plane) related set of gtk widgets that can be usable outside of it.

mod traits;

mod group;
mod loading_indicator;
mod shimmer_effect;
mod spoiler_overlay;

pub use group::*;
use gtk::prelude::StaticType;
pub use loading_indicator::LoadingIndicator;
pub use shimmer_effect::ShimmerEffect;
pub use spoiler_overlay::SpoilerOverlay;

/// Registers all library types.
///
/// Expected to be called in the main function
pub fn init() {
    AnimatedGroup::static_type();
    Group::static_type();
    LoadingIndicator::static_type();
    ShimmerEffect::static_type();
    SpoilerOverlay::static_type();
}

use bevy::asset::AssetDynamic;
use bevy::reflect::{TypeUuid, TypeUuidDynamic};
use bevy::asset::{Asset};

trait DerivedAsset: Asset { }

#[derive(Clone, Default, TypeUuid)]
#[uuid = "45e22445-c6f6-455c-9527-1bf9932193e3"]
struct SomeReflectedType { }

impl DerivedAsset for SomeReflectedType { } //< the trait bound `SomeReflectedType: Asset` is not satisfied [E0277] the trait `Asset` is not implemented for `SomeReflectedType`

fn main() {
    println!("Hello, world!");
}

use bevy_reflect::TypeUuid;

#[derive(Clone, Default, TypeUuid)]
#[uuid = "45e22445-c6f6-455c-9527-1bf9932193e3"]
struct SomeReflectedType { }

trait DerivedAsset: Asset_ { }
pub trait AssetDynamic_: Send + Sync + 'static {}
impl<T> AssetDynamic_ for T where T: Send + Sync + 'static {}
pub trait Asset_: TypeUuid + AssetDynamic_ {}
impl<T> Asset_ for T where T: TypeUuid + AssetDynamic_ {}

impl DerivedAsset for SomeReflectedType { } //< the trait bound `SomeReflectedType: Asset` is not satisfied [E0277] the trait `Asset` is not implemented for `SomeReflectedType`

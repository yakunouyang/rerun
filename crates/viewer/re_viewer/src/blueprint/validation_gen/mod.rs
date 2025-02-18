// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/blueprint_validation.rs
use super::validation::validate_component;
use re_entity_db::EntityDb;
pub use re_types::blueprint::components::ActiveTab;
pub use re_types::blueprint::components::ApplyLatestAt;
pub use re_types::blueprint::components::AutoLayout;
pub use re_types::blueprint::components::AutoViews;
pub use re_types::blueprint::components::BackgroundKind;
pub use re_types::blueprint::components::ColumnShare;
pub use re_types::blueprint::components::ComponentColumnSelector;
pub use re_types::blueprint::components::ContainerKind;
pub use re_types::blueprint::components::Corner2D;
pub use re_types::blueprint::components::Enabled;
pub use re_types::blueprint::components::FilterByRange;
pub use re_types::blueprint::components::FilterIsNotNull;
pub use re_types::blueprint::components::ForceDistance;
pub use re_types::blueprint::components::ForceIterations;
pub use re_types::blueprint::components::ForceStrength;
pub use re_types::blueprint::components::GridColumns;
pub use re_types::blueprint::components::GridSpacing;
pub use re_types::blueprint::components::IncludedContent;
pub use re_types::blueprint::components::Interactive;
pub use re_types::blueprint::components::LockRangeDuringZoom;
pub use re_types::blueprint::components::MapProvider;
pub use re_types::blueprint::components::NearClipPlane;
pub use re_types::blueprint::components::PanelState;
pub use re_types::blueprint::components::QueryExpression;
pub use re_types::blueprint::components::RootContainer;
pub use re_types::blueprint::components::RowShare;
pub use re_types::blueprint::components::SelectedColumns;
pub use re_types::blueprint::components::TensorDimensionIndexSlider;
pub use re_types::blueprint::components::TimelineName;
pub use re_types::blueprint::components::ViewClass;
pub use re_types::blueprint::components::ViewFit;
pub use re_types::blueprint::components::ViewMaximized;
pub use re_types::blueprint::components::ViewOrigin;
pub use re_types::blueprint::components::ViewerRecommendationHash;
pub use re_types::blueprint::components::VisibleTimeRange;
pub use re_types::blueprint::components::VisualBounds2D;
pub use re_types::blueprint::components::VisualizerOverrides;
pub use re_types::blueprint::components::ZoomLevel;

/// Because blueprints are both read and written the schema must match what
/// we expect to find or else we will run into all kinds of problems.

pub fn is_valid_blueprint(blueprint: &EntityDb) -> bool {
    validate_component::<ActiveTab>(blueprint)
        && validate_component::<ApplyLatestAt>(blueprint)
        && validate_component::<AutoLayout>(blueprint)
        && validate_component::<AutoViews>(blueprint)
        && validate_component::<BackgroundKind>(blueprint)
        && validate_component::<ColumnShare>(blueprint)
        && validate_component::<ComponentColumnSelector>(blueprint)
        && validate_component::<ContainerKind>(blueprint)
        && validate_component::<Corner2D>(blueprint)
        && validate_component::<Enabled>(blueprint)
        && validate_component::<FilterByRange>(blueprint)
        && validate_component::<FilterIsNotNull>(blueprint)
        && validate_component::<ForceDistance>(blueprint)
        && validate_component::<ForceIterations>(blueprint)
        && validate_component::<ForceStrength>(blueprint)
        && validate_component::<GridColumns>(blueprint)
        && validate_component::<GridSpacing>(blueprint)
        && validate_component::<IncludedContent>(blueprint)
        && validate_component::<Interactive>(blueprint)
        && validate_component::<LockRangeDuringZoom>(blueprint)
        && validate_component::<MapProvider>(blueprint)
        && validate_component::<NearClipPlane>(blueprint)
        && validate_component::<PanelState>(blueprint)
        && validate_component::<QueryExpression>(blueprint)
        && validate_component::<RootContainer>(blueprint)
        && validate_component::<RowShare>(blueprint)
        && validate_component::<SelectedColumns>(blueprint)
        && validate_component::<TensorDimensionIndexSlider>(blueprint)
        && validate_component::<TimelineName>(blueprint)
        && validate_component::<ViewClass>(blueprint)
        && validate_component::<ViewFit>(blueprint)
        && validate_component::<ViewMaximized>(blueprint)
        && validate_component::<ViewOrigin>(blueprint)
        && validate_component::<ViewerRecommendationHash>(blueprint)
        && validate_component::<VisibleTimeRange>(blueprint)
        && validate_component::<VisualBounds2D>(blueprint)
        && validate_component::<VisualizerOverrides>(blueprint)
        && validate_component::<ZoomLevel>(blueprint)
}

// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/panel_blueprint.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Shared state for the 3 collapsible panels.
#[derive(Clone, Debug, Default)]
pub struct PanelBlueprint {
    pub state: Option<crate::blueprint::components::PanelState>,
}

impl ::re_types_core::SizeBytes for PanelBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.state.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::PanelState>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.PanelBlueprintIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.PanelState".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.PanelBlueprintIndicator".into(),
            "rerun.blueprint.components.PanelState".into(),
        ]
    });

impl PanelBlueprint {
    /// The total number of components in the archetype: 0 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`PanelBlueprint`] [`::re_types_core::Archetype`]
pub type PanelBlueprintIndicator = ::re_types_core::GenericIndicatorComponent<PanelBlueprint>;

impl ::re_types_core::Archetype for PanelBlueprint {
    type Indicator = PanelBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.PanelBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Panel blueprint"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: PanelBlueprintIndicator = PanelBlueprintIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let state = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.PanelState")
        {
            <crate::blueprint::components::PanelState>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.PanelBlueprint#state")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { state })
    }
}

impl ::re_types_core::AsComponents for PanelBlueprint {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.state
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for PanelBlueprint {}

impl PanelBlueprint {
    /// Create a new `PanelBlueprint`.
    #[inline]
    pub fn new() -> Self {
        Self { state: None }
    }

    #[inline]
    pub fn with_state(
        mut self,
        state: impl Into<crate::blueprint::components::PanelState>,
    ) -> Self {
        self.state = Some(state.into());
        self
    }
}

// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/view_blueprint.fbs".

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

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: The description of a single view.
#[derive(Clone, Debug, Default)]
pub struct ViewBlueprint {
    /// The class of the view.
    pub class_identifier: Option<SerializedComponentBatch>,

    /// The name of the view.
    pub display_name: Option<SerializedComponentBatch>,

    /// The "anchor point" of this view.
    ///
    /// Defaults to the root path '/' if not specified.
    ///
    /// The transform at this path forms the reference point for all scene->world transforms in this view.
    /// I.e. the position of this entity path in space forms the origin of the coordinate system in this view.
    /// Furthermore, this is the primary indicator for heuristics on what entities we show in this view.
    pub space_origin: Option<SerializedComponentBatch>,

    /// Whether this view is visible.
    ///
    /// Defaults to true if not specified.
    pub visible: Option<SerializedComponentBatch>,
}

impl ViewBlueprint {
    /// Returns the [`ComponentDescriptor`] for [`Self::class_identifier`].
    #[inline]
    pub fn descriptor_class_identifier() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewBlueprint".into()),
            component_name: "rerun.blueprint.components.ViewClass".into(),
            archetype_field_name: Some("class_identifier".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::display_name`].
    #[inline]
    pub fn descriptor_display_name() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewBlueprint".into()),
            component_name: "rerun.components.Name".into(),
            archetype_field_name: Some("display_name".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::space_origin`].
    #[inline]
    pub fn descriptor_space_origin() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewBlueprint".into()),
            component_name: "rerun.blueprint.components.ViewOrigin".into(),
            archetype_field_name: Some("space_origin".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::visible`].
    #[inline]
    pub fn descriptor_visible() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewBlueprint".into()),
            component_name: "rerun.components.Visible".into(),
            archetype_field_name: Some("visible".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewBlueprint".into()),
            component_name: "rerun.blueprint.components.ViewBlueprintIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewBlueprint::descriptor_class_identifier()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewBlueprint::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ViewBlueprint::descriptor_display_name(),
            ViewBlueprint::descriptor_space_origin(),
            ViewBlueprint::descriptor_visible(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ViewBlueprint::descriptor_class_identifier(),
            ViewBlueprint::descriptor_indicator(),
            ViewBlueprint::descriptor_display_name(),
            ViewBlueprint::descriptor_space_origin(),
            ViewBlueprint::descriptor_visible(),
        ]
    });

impl ViewBlueprint {
    /// The total number of components in the archetype: 1 required, 1 recommended, 3 optional
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`ViewBlueprint`] [`::re_types_core::Archetype`]
pub type ViewBlueprintIndicator = ::re_types_core::GenericIndicatorComponent<ViewBlueprint>;

impl ::re_types_core::Archetype for ViewBlueprint {
    type Indicator = ViewBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ViewBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "View blueprint"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        ViewBlueprintIndicator::DEFAULT.serialized().unwrap()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let class_identifier = arrays_by_descr
            .get(&Self::descriptor_class_identifier())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_class_identifier())
            });
        let display_name = arrays_by_descr
            .get(&Self::descriptor_display_name())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_display_name())
            });
        let space_origin = arrays_by_descr
            .get(&Self::descriptor_space_origin())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_space_origin())
            });
        let visible = arrays_by_descr
            .get(&Self::descriptor_visible())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_visible()));
        Ok(Self {
            class_identifier,
            display_name,
            space_origin,
            visible,
        })
    }
}

impl ::re_types_core::AsComponents for ViewBlueprint {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.class_identifier.clone(),
            self.display_name.clone(),
            self.space_origin.clone(),
            self.visible.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ViewBlueprint {}

impl ViewBlueprint {
    /// Create a new `ViewBlueprint`.
    #[inline]
    pub fn new(class_identifier: impl Into<crate::blueprint::components::ViewClass>) -> Self {
        Self {
            class_identifier: try_serialize_field(
                Self::descriptor_class_identifier(),
                [class_identifier],
            ),
            display_name: None,
            space_origin: None,
            visible: None,
        }
    }

    /// Update only some specific fields of a `ViewBlueprint`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `ViewBlueprint`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            class_identifier: Some(SerializedComponentBatch::new(
                crate::blueprint::components::ViewClass::arrow_empty(),
                Self::descriptor_class_identifier(),
            )),
            display_name: Some(SerializedComponentBatch::new(
                crate::components::Name::arrow_empty(),
                Self::descriptor_display_name(),
            )),
            space_origin: Some(SerializedComponentBatch::new(
                crate::blueprint::components::ViewOrigin::arrow_empty(),
                Self::descriptor_space_origin(),
            )),
            visible: Some(SerializedComponentBatch::new(
                crate::components::Visible::arrow_empty(),
                Self::descriptor_visible(),
            )),
        }
    }

    /// The class of the view.
    #[inline]
    pub fn with_class_identifier(
        mut self,
        class_identifier: impl Into<crate::blueprint::components::ViewClass>,
    ) -> Self {
        self.class_identifier =
            try_serialize_field(Self::descriptor_class_identifier(), [class_identifier]);
        self
    }

    /// The name of the view.
    #[inline]
    pub fn with_display_name(mut self, display_name: impl Into<crate::components::Name>) -> Self {
        self.display_name = try_serialize_field(Self::descriptor_display_name(), [display_name]);
        self
    }

    /// The "anchor point" of this view.
    ///
    /// Defaults to the root path '/' if not specified.
    ///
    /// The transform at this path forms the reference point for all scene->world transforms in this view.
    /// I.e. the position of this entity path in space forms the origin of the coordinate system in this view.
    /// Furthermore, this is the primary indicator for heuristics on what entities we show in this view.
    #[inline]
    pub fn with_space_origin(
        mut self,
        space_origin: impl Into<crate::blueprint::components::ViewOrigin>,
    ) -> Self {
        self.space_origin = try_serialize_field(Self::descriptor_space_origin(), [space_origin]);
        self
    }

    /// Whether this view is visible.
    ///
    /// Defaults to true if not specified.
    #[inline]
    pub fn with_visible(mut self, visible: impl Into<crate::components::Visible>) -> Self {
        self.visible = try_serialize_field(Self::descriptor_visible(), [visible]);
        self
    }
}

impl ::re_byte_size::SizeBytes for ViewBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.class_identifier.heap_size_bytes()
            + self.display_name.heap_size_bytes()
            + self.space_origin.heap_size_bytes()
            + self.visible.heap_size_bytes()
    }
}

// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/view_coordinates.fbs".

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
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: How we interpret the coordinate system of an entity/space.
///
/// For instance: What is "up"? What does the Z axis mean?
///
/// The three coordinates are always ordered as [x, y, z].
///
/// For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
/// down, and the Z axis points forward.
///
/// Make sure that this archetype is logged at or above the origin entity path of your 3D views.
///
/// ⚠ [Rerun does not yet support left-handed coordinate systems](https://github.com/rerun-io/rerun/issues/5032).
///
/// ## Example
///
/// ### View coordinates for adjusting the eye camera
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_view_coordinates").spawn()?;
///
///     rec.log_static("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP())?; // Set an up-axis
///     rec.log(
///         "world/xyz",
///         &rerun::Arrows3D::from_vectors(
///             [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], //
///         )
///         .with_colors([[255, 0, 0], [0, 255, 0], [0, 0, 255]]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/1200w.png">
///   <img src="https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
#[repr(transparent)]
pub struct ViewCoordinates {
    /// The directions of the [x, y, z] axes.
    pub xyz: Option<SerializedComponentBatch>,
}

impl ViewCoordinates {
    /// Returns the [`ComponentDescriptor`] for [`Self::xyz`].
    #[inline]
    pub fn descriptor_xyz() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.ViewCoordinates".into()),
            component_name: "rerun.components.ViewCoordinates".into(),
            archetype_field_name: Some("xyz".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.ViewCoordinates".into()),
            component_name: "rerun.components.ViewCoordinatesIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewCoordinates::descriptor_xyz()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewCoordinates::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ViewCoordinates::descriptor_xyz(),
            ViewCoordinates::descriptor_indicator(),
        ]
    });

impl ViewCoordinates {
    /// The total number of components in the archetype: 1 required, 1 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`ViewCoordinates`] [`::re_types_core::Archetype`]
pub type ViewCoordinatesIndicator = ::re_types_core::GenericIndicatorComponent<ViewCoordinates>;

impl ::re_types_core::Archetype for ViewCoordinates {
    type Indicator = ViewCoordinatesIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.ViewCoordinates".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "View coordinates"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ViewCoordinatesIndicator = ViewCoordinatesIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
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
        let xyz = arrays_by_descr
            .get(&Self::descriptor_xyz())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_xyz()));
        Ok(Self { xyz })
    }
}

impl ::re_types_core::AsComponents for ViewCoordinates {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [Self::indicator().serialized(), self.xyz.clone()]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ViewCoordinates {}

impl ViewCoordinates {
    /// Create a new `ViewCoordinates`.
    #[inline]
    pub fn new(xyz: impl Into<crate::components::ViewCoordinates>) -> Self {
        Self {
            xyz: try_serialize_field(Self::descriptor_xyz(), [xyz]),
        }
    }

    /// Update only some specific fields of a `ViewCoordinates`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `ViewCoordinates`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            xyz: Some(SerializedComponentBatch::new(
                crate::components::ViewCoordinates::arrow_empty(),
                Self::descriptor_xyz(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [self
            .xyz
            .map(|xyz| xyz.partitioned(_lengths.clone()))
            .transpose()?];
        let indicator_column =
            ::re_types_core::indicator_column::<Self>(_lengths.into_iter().count())?;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// Helper to partition the component data into unit-length sub-batches.
    ///
    /// This is semantically similar to calling [`Self::columns`] with `std::iter::take(1).repeat(n)`,
    /// where `n` is automatically guessed.
    #[inline]
    pub fn columns_of_unit_batches(
        self,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>> {
        let len_xyz = self.xyz.as_ref().map(|b| b.array.len());
        let len = None.or(len_xyz).unwrap_or(0);
        self.columns(std::iter::repeat(1).take(len))
    }

    /// The directions of the [x, y, z] axes.
    #[inline]
    pub fn with_xyz(mut self, xyz: impl Into<crate::components::ViewCoordinates>) -> Self {
        self.xyz = try_serialize_field(Self::descriptor_xyz(), [xyz]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ViewCoordinates`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_xyz`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_xyz(
        mut self,
        xyz: impl IntoIterator<Item = impl Into<crate::components::ViewCoordinates>>,
    ) -> Self {
        self.xyz = try_serialize_field(Self::descriptor_xyz(), xyz);
        self
    }
}

impl ::re_byte_size::SizeBytes for ViewCoordinates {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.xyz.heap_size_bytes()
    }
}

# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/marker_size.fbs".

# You can extend this class by creating a "MarkerSizeExt" class in "marker_size_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["MarkerSize", "MarkerSizeBatch", "MarkerSizeType"]


class MarkerSize(datatypes.Float32, ComponentMixin):
    """**Component**: Radius of a marker of a point in e.g. a 2D plot, measured in UI points."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of MarkerSizeExt in marker_size_ext.py

    # Note: there are no fields here because MarkerSize delegates to datatypes.Float32
    pass


class MarkerSizeType(datatypes.Float32Type):
    _TYPE_NAME: str = "rerun.components.MarkerSize"


class MarkerSizeBatch(datatypes.Float32Batch, ComponentBatchMixin):
    _ARROW_TYPE = MarkerSizeType()


# This is patched in late to avoid circular dependencies.
MarkerSize._BATCH_TYPE = MarkerSizeBatch  # type: ignore[assignment]

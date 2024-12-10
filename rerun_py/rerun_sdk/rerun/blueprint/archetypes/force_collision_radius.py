# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_collision_radius.fbs".

# You can extend this class by creating a "ForceCollisionRadiusExt" class in "force_collision_radius_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ... import datatypes
from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["ForceCollisionRadius"]


@define(str=False, repr=False, init=False)
class ForceCollisionRadius(Archetype):
    """**Archetype**: Resolves collisions between the bounding spheres, according to the radius of the nodes."""

    def __init__(
        self: Any,
        *,
        enabled: datatypes.BoolLike | None = None,
        strength: datatypes.Float64Like | None = None,
        iterations: datatypes.UInt64Like | None = None,
    ):
        """
        Create a new instance of the ForceCollisionRadius archetype.

        Parameters
        ----------
        enabled:
            Whether the force is enabled.
        strength:
            The strength of the force.
        iterations:
            Specifies how often this force should be applied per iteration.

            Increasing this parameter can lead to better results at the cost of longer computation time.

        """

        # You can define your own __init__ function as a member of ForceCollisionRadiusExt in force_collision_radius_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(enabled=enabled, strength=strength, iterations=iterations)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            enabled=None,  # type: ignore[arg-type]
            strength=None,  # type: ignore[arg-type]
            iterations=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> ForceCollisionRadius:
        """Produce an empty ForceCollisionRadius, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    enabled: blueprint_components.EnabledBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.EnabledBatch._optional,  # type: ignore[misc]
    )
    # Whether the force is enabled.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    strength: blueprint_components.ForceStrengthBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.ForceStrengthBatch._optional,  # type: ignore[misc]
    )
    # The strength of the force.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    iterations: blueprint_components.ForceIterationsBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.ForceIterationsBatch._optional,  # type: ignore[misc]
    )
    # Specifies how often this force should be applied per iteration.
    #
    # Increasing this parameter can lead to better results at the cost of longer computation time.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]

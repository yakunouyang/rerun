# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_position.fbs".

# You can extend this class by creating a "ForcePositionExt" class in "force_position_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ... import components, datatypes
from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["ForcePosition"]


@define(str=False, repr=False, init=False)
class ForcePosition(Archetype):
    """**Archetype**: Similar to gravity, this force pulls nodes towards a specific position."""

    def __init__(
        self: Any,
        *,
        enabled: datatypes.BoolLike | None = None,
        strength: datatypes.Float64Like | None = None,
        position: datatypes.Vec2DLike | None = None,
    ):
        """
        Create a new instance of the ForcePosition archetype.

        Parameters
        ----------
        enabled:
            Whether the force is enabled.
        strength:
            The strength of the force.
        position:
            The position where the nodes should be pulled towards.

        """

        # You can define your own __init__ function as a member of ForcePositionExt in force_position_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(enabled=enabled, strength=strength, position=position)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            enabled=None,  # type: ignore[arg-type]
            strength=None,  # type: ignore[arg-type]
            position=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> ForcePosition:
        """Produce an empty ForcePosition, bypassing `__init__`."""
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

    position: components.Position2DBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.Position2DBatch._optional,  # type: ignore[misc]
    )
    # The position where the nodes should be pulled towards.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]

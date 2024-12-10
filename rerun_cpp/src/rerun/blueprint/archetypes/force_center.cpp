// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_center.fbs".

#include "force_center.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<blueprint::archetypes::ForceCenter>::serialize(
        const blueprint::archetypes::ForceCenter& archetype
    ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(3);

        if (archetype.enabled.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.enabled.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.ForceCenter",
                    "enabled",
                    "rerun.blueprint.components.Enabled"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.strength.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.strength.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.ForceCenter",
                    "strength",
                    "rerun.blueprint.components.ForceStrength"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = ForceCenter::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun

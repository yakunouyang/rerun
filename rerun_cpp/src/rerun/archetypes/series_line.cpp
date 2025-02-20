// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/series_line.fbs".

#include "series_line.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    SeriesLine SeriesLine::clear_fields() {
        auto archetype = SeriesLine();
        archetype.color =
            ComponentBatch::empty<rerun::components::Color>(Descriptor_color).value_or_throw();
        archetype.width = ComponentBatch::empty<rerun::components::StrokeWidth>(Descriptor_width)
                              .value_or_throw();
        archetype.name =
            ComponentBatch::empty<rerun::components::Name>(Descriptor_name).value_or_throw();
        archetype.visible_series =
            ComponentBatch::empty<rerun::components::SeriesVisible>(Descriptor_visible_series)
                .value_or_throw();
        archetype.aggregation_policy = ComponentBatch::empty<rerun::components::AggregationPolicy>(
                                           Descriptor_aggregation_policy
        )
                                           .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> SeriesLine::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(6);
        if (color.has_value()) {
            columns.push_back(color.value().partitioned(lengths_).value_or_throw());
        }
        if (width.has_value()) {
            columns.push_back(width.value().partitioned(lengths_).value_or_throw());
        }
        if (name.has_value()) {
            columns.push_back(name.value().partitioned(lengths_).value_or_throw());
        }
        if (visible_series.has_value()) {
            columns.push_back(visible_series.value().partitioned(lengths_).value_or_throw());
        }
        if (aggregation_policy.has_value()) {
            columns.push_back(aggregation_policy.value().partitioned(lengths_).value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<SeriesLine>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> SeriesLine::columns() {
        if (color.has_value()) {
            return columns(std::vector<uint32_t>(color.value().length(), 1));
        }
        if (width.has_value()) {
            return columns(std::vector<uint32_t>(width.value().length(), 1));
        }
        if (name.has_value()) {
            return columns(std::vector<uint32_t>(name.value().length(), 1));
        }
        if (visible_series.has_value()) {
            return columns(std::vector<uint32_t>(visible_series.value().length(), 1));
        }
        if (aggregation_policy.has_value()) {
            return columns(std::vector<uint32_t>(aggregation_policy.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<Collection<ComponentBatch>> AsComponents<archetypes::SeriesLine>::as_batches(
        const archetypes::SeriesLine& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(6);

        if (archetype.color.has_value()) {
            cells.push_back(archetype.color.value());
        }
        if (archetype.width.has_value()) {
            cells.push_back(archetype.width.value());
        }
        if (archetype.name.has_value()) {
            cells.push_back(archetype.name.value());
        }
        if (archetype.visible_series.has_value()) {
            cells.push_back(archetype.visible_series.value());
        }
        if (archetype.aggregation_policy.has_value()) {
            cells.push_back(archetype.aggregation_policy.value());
        }
        {
            auto result = ComponentBatch::from_indicator<SeriesLine>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return rerun::take_ownership(std::move(cells));
    }
} // namespace rerun

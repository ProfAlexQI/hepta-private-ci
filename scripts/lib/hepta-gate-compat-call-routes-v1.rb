# frozen_string_literal: true

module HeptaGateCompatCallRoutes
  module_function

  def decode(routes, symbols)
    return routes unless routes.is_a?(Array)
    routes.to_h do |source_index, target_index|
      [symbols.fetch(source_index), symbols.fetch(target_index)]
    end
  end

  def verify!(registry)
    symbols = registry.fetch("call_route_symbols")
    raise "family call-route symbol table contains duplicates" unless symbols.uniq.length ==
      symbols.length
    registry.fetch("families").each do |family|
      family.fetch("call_routes").each_value do |routes|
        raise "family call-route encoding drifted" unless routes.all? do |source, target|
          source.is_a?(Integer) && target.is_a?(Integer) &&
            source.between?(0, symbols.length - 1) && target.between?(0, symbols.length - 1)
        end
      end
    end
  end
end

local std = require("std")

local move = require("lib.move")

local cylinder_builder = {}

local function place_block()
    if not turtle.placeDown() then
        print("Warning: Could not place block")
    end
end

--- Builds a cylinder with the specified parameters
--- @param height number
--- @param diameter number
--- @param filled boolean
function cylinder_builder.build(height, diameter, filled)
    print("Building cylinder:")
    print("Height: " .. height)
    print("Diameter: " .. diameter)
    print("Filled: " .. (filled and "yes" or "no"))

    for layer = 1, height do
        turtle.up()

        print("Building layer " .. layer .. " of " .. height)

        move.traverse_circle(diameter, filled, place_block)
    end

    print("Cylinder construction complete!")
end

local function main(...)
    local args = { ... }

    if #args < 3 then
        print("Usage: cylinder_builder <height> <diameter> <filled>")
        print("  height: Number of layers high")
        print("  diameter: Diameter of the cylinder")
        print("  filled: true for filled cylinder, false for hollow")
        return
    end

    local height = tonumber(args[1])
    local diameter = tonumber(args[2])
    local filled_str = args[3]

    if height == nil then
        print("Invalid height: " .. args[1])
        return
    end

    if diameter == nil then
        print("Invalid diameter: " .. args[2])
        return
    end

    if height <= 0 then
        print("Height must be positive")
        return
    end

    if diameter <= 0 then
        print("Diameter must be positive")
        return
    end

    local filled = false
    if filled_str == "true" then
        filled = true
    elseif filled_str == "false" then
        filled = false
    else
        print("Invalid filled value: " .. filled_str .. " (must be 'true' or 'false')")
        return
    end

    cylinder_builder.build(height, diameter, filled)
end

if std.is_main() then
    main(...)
end

return cylinder_builder

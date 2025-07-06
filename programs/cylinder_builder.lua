local std = require("std")

local inventory = require("lib.inventory")
local move = require("lib.move")

local cylinder_builder = {}

--- @param building_item string
local function place_block(building_item)
    if turtle.getItemCount() == 0 then
        if building_item and inventory.select_item(building_item) then
            print("Switched to new stack of " .. building_item)
        else
            print("Error: No more " .. (building_item or "building materials") .. " available")
            return
        end
    end

    if not turtle.placeDown() then
        print("Warning: Could not place block")
    end
end

--- Builds a cylinder with the specified parameters
--- @param height number
--- @param diameter number
--- @param filled boolean
function cylinder_builder.build(height, diameter, filled)
    turtle.select(1)
    local building_item = nil
    local item_detail = turtle.getItemDetail()
    if item_detail then
        building_item = item_detail.name
        print("Using building material: " .. building_item)
    else
        print("Error: No building materials found in slot 1")
        return
    end

    print("Building cylinder:")
    print("Height: " .. height)
    print("Diameter: " .. diameter)
    print("Filled: " .. (filled and "yes" or "no"))

    for layer = 1, height do
        turtle.up()

        print("Building layer " .. layer .. " of " .. height)

        move.traverse_circle(diameter, filled, function()
            place_block(building_item)
        end)
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

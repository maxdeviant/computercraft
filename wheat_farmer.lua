local std = require("std")
local inventory = require("lib.inventory")
local move = require("lib.move")

local wheat = "minecraft:wheat"
local wheat_seeds = "minecraft:wheat_seeds"

local function is_above_wheat()
    local has_block, data = turtle.inspectDown()
    return has_block and data.name == wheat
end

local function is_wheat_grown()
    local has_block, data = turtle.inspectDown()
    if not has_block then
        return false
    end

    if data.name ~= wheat then
        return false
    end

    return data.state.age == 7
end

local function till_soil()
    std.times(2, function()
        turtle.digDown()
    end)
end

local function plant_wheat()
    if not inventory.select_item(wheat_seeds) then
        return false
    end

    turtle.placeDown()

    return true
end

local function harvest()
    turtle.digDown()
end

local function main()
    local size = tonumber(arg[1])
    if size == nil then
        print("Invalid size")
        return
    end

    print("Starting wheat farmer")
    print("Size: " .. size .. "x" .. size)

    while true do
        move.traverse_plane(size, size, function()
            if is_wheat_grown() then
                harvest()
                plant_wheat()
            elseif is_above_wheat() then
                -- Skip over it.
            else
                till_soil()
                plant_wheat()
            end
        end)
    end
end

main()

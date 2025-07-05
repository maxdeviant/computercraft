local inventory = require("lib.inventory")

local log_kind = "minecraft:birch_log"
local sapling_kind = "minecraft:birch_sapling"
local track_kind = "minecraft:cobbled_deepslate"
local tree_spacing = 2

local function plant_sapling(sapling)
    local has_block, data = turtle.inspect()
    if not has_block or data.name ~= sapling then
        if inventory.select_item(sapling) then
            turtle.place()
        end
    end
end

local function move_to_ground()
    while true do
        local has_block, data = turtle.inspectDown()
        if has_block then
            if data.name == track_kind then
                break
            else
                turtle.digDown()
            end
        end

        turtle.down()
    end
end

local function chop_tree()
    print("Chopping tree...")

    while true do
        local _, data = turtle.inspect()
        if data.name ~= log_kind then
            break
        end

        turtle.dig()
        turtle.digUp()
        turtle.up()
    end

    move_to_ground()
    plant_sapling(sapling_kind)
end

local function move_to_next_tree()
    print("Moving to next tree...")

    turtle.turnRight()

    for i = 1, tree_spacing do
        turtle.forward()
        turtle.suck()

        local has_block, data = turtle.inspectDown()
        if has_block and data.name ~= track_kind then
            print("Turning around...")

            turtle.turnLeft()
            turtle.turnLeft()
            turtle.forward()

            turtle.turnLeft()

            return
        end
    end

    turtle.turnLeft()
end

local function main()
    while true do
        chop_tree()
        move_to_next_tree()
    end
end

main()

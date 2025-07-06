local inventory = require("lib.inventory")

local torch = "minecraft:torch"
local tunnel_width = 3
local tunnel_height = 3
local torch_spacing = 5

local function place_torch(height)
    if not inventory.select_item(torch) then
        return false
    end

    for _ = 1, height - 1 do
        turtle.up()
    end

    turtle.place()

    for _ = 1, height - 1 do
        turtle.down()
    end

    return true
end

--- Mines a column of the specified height in front of the turtle.
---
--- Will return down to the starting location, once finished.
local function mine_column(height)
    for _ = 1, height - 1 do
        turtle.dig()
        turtle.digUp()
        turtle.up()
    end

    turtle.dig()

    for _ = 1, height do
        turtle.down()
    end
end

local function mine_row(row)
    mine_column(tunnel_height)
    turtle.forward()
    turtle.turnLeft()

    for i = 1, tunnel_width - 1 do
        mine_column(tunnel_height)

        if i < tunnel_width - 1 then
            turtle.forward()
        end
    end

    if row % torch_spacing == 0 then
        place_torch(2)
    end

    turtle.turnRight()
    turtle.turnRight()

    for _ = 1, tunnel_width - 1 do
        turtle.forward()
    end

    turtle.turnLeft()
end

local function main()
    print("Starting tunnel miner")

    local row = 0
    while true do
        if inventory.has_item(torch) then
            mine_row(row)
            row = row + 1
        else
            print("Out of torches")
            sleep(10)
        end
    end
end

main()

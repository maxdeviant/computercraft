local move = require("lib.move")

local function mine_shaft(depth, width, height)
    for _ = 1, depth do
        turtle.digDown()
        turtle.down()
        move.traverse_plane(width, height, turtle.dig)
    end
end

local function main(...)
    local args = { ... }
    local depth = args[1]
    local width = args[2]
    local height = args[3] or width

    print("Starting shaft miner")
    print("Depth: " .. depth)
    print("Size: " .. width .. "x" .. height)

    mine_shaft(tonumber(depth), tonumber(width), tonumber(height))
end

main(...)

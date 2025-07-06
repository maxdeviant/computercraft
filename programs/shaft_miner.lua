local move = require("lib.move")

local function mine_shaft(depth, size)
    for _ = 1, depth do
        turtle.digDown()
        turtle.down()
        move.traverse_plane(size, size, turtle.dig)
    end
end

local function main(...)
    local args = { ... }
    local depth = args[1]
    local size = args[2]

    print("Starting shaft miner")
    print("Depth: " .. depth)
    print("Size: " .. size .. "x" .. size)

    mine_shaft(tonumber(depth), tonumber(size))
end

main(...)

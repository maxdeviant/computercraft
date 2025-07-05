local function mine_shaft_layer(width, height)
    for x = 1,width do
        for _ = 1,height - 1 do
            turtle.dig()
            turtle.forward()
        end

        if x < width then
            if x % 2 == 0 then
                turtle.turnLeft()
                turtle.dig()
                turtle.forward()
                turtle.turnLeft()
            else
                turtle.turnRight()
                turtle.dig()
                turtle.forward()
                turtle.turnRight()
            end
        end
    end

    turtle.turnRight()
    turtle.turnRight()
end

local function mine_shaft(depth, size)
    for _ = 1,depth do
        turtle.digDown()
        turtle.down()
        mine_shaft_layer(size, size)
    end
end

local function main()
    local depth = arg[1]
    local size = arg[2]

    print("Starting shaft miner")
    print("Depth: " .. depth)
    print("Size: " .. size .. "x" .. size)

    mine_shaft(tonumber(depth), tonumber(size))
end

main()

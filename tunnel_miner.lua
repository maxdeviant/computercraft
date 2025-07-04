torch = "minecraft:torch"
tunnel_width = 3
tunnel_height = 3
torch_spacing = 5

function has_item(item)
    for i = 1,16 do
        it = turtle.getItemDetail(i)
        if it and it.name == item then
            return true
        end
    end

    return false
end

function select_item(item)
    for i = 1,16 do
        turtle.select(i)
        it = turtle.getItemDetail()
        if it and it.name == item then
            return true
        end
    end

    return false
end

function place_torch(height)
    if not select_item(torch) then
        return false
    end

    for _ = 1,height - 1 do
        turtle.up()
    end

    turtle.place()

    for _ = 1,height - 1 do
        turtle.down()
    end

    return true
end

-- Mines a column of the specified height in front of the turtle.
--
-- Will return down to the starting location, once finished.
function mine_column(height)
    for _ = 1,height - 1 do
        turtle.dig()
        turtle.digUp()
        turtle.up()
    end

    turtle.dig()

    for _ = 1,height do
        turtle.down()
    end
end

function mine_row(row)
    mine_column(tunnel_height)
    turtle.forward()
    turtle.turnLeft()

    for i = 1,tunnel_width - 1 do
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

    for _ = 1,tunnel_width - 1 do
        turtle.forward()
    end

    turtle.turnLeft()
end

function main()
    print("Starting tunnel miner")

    local row = 0
    while true do
        if has_item(torch) then
            mine_row(row)
            row = row + 1
        else
            print("Out of torches")
            sleep(10)
        end
    end
end

main()

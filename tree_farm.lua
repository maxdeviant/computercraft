log_kind = "minecraft:birch_log"
sapling_kind = "minecraft:birch_sapling"
track_kind = "minecraft:cobbled_deepslate"
tree_spacing = 2

function chop_tree()
    print("Chopping tree...")

    while true do
        _, data = turtle.inspect()
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

function plant_sapling(sapling)
    has_block, data = turtle.inspect()
    if not has_block or data.name ~= sapling then
        if select_item(sapling) then
            turtle.place()
        end
    end
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

function move_to_ground()
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

function move_to_next_tree()
    print("Moving to next tree...")

    turtle.turnRight()

    for i = 1,tree_spacing do
        turtle.forward()
        turtle.suck()

        has_block, data = turtle.inspectDown()
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

function main()
    while true do
        chop_tree()
        move_to_next_tree()
    end
end

main()

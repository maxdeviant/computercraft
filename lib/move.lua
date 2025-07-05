local move = {}

--- Moves the turtle forward by *n* blocks.
---
--- @param n number
function move.forward(n)
    for _ = 1, n do
        turtle.forward()
    end
end

--- Moves the turtle backwards by *n* blocks.
---
--- @param n number
function move.back(n)
    for _ = 1, n do
        turtle.back()
    end
end

--- Moves the turtle up by *n* blocks.
---
--- @param n number
function move.up(n)
    for _ = 1, n do
        turtle.up()
    end
end

--- Moves the turtle down by *n* blocks.
---
--- @param n number
function move.down(n)
    for _ = 1, n do
        turtle.down()
    end
end

--- Traverses a plane with the specified width and height, invoking the provided action at each block.
---
--- @param width number
--- @param height number
--- @param action fun(): nil
function move.traverse_plane(width, height, action)
    for x = 1, width do
        for _ = 1, height - 1 do
            action()
            turtle.forward()
        end

        if x < width then
            if x % 2 == 0 then
                turtle.turnLeft()
                action()
                turtle.forward()
                turtle.turnLeft()
            else
                turtle.turnRight()
                action()
                turtle.forward()
                turtle.turnRight()
            end
        end
    end

    turtle.turnRight()
    turtle.turnRight()
end

return move

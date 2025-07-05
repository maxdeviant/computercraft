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

return move

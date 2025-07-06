local std = require("std")

local move = {}

--- A cardinal direction.
local Direction = {
    NORTH = 0,
    EAST = 1,
    SOUTH = 2,
    WEST = 3,
}

--- Moves the turtle forward by *n* blocks.
---
--- @param n number
function move.forward(n)
    std.times(n, turtle.forward)
end

--- Moves the turtle backwards by *n* blocks.
---
--- @param n number
function move.back(n)
    std.times(n, turtle.back)
end

--- Moves the turtle up by *n* blocks.
---
--- @param n number
function move.up(n)
    std.times(n, turtle.up)
end

--- Moves the turtle down by *n* blocks.
---
--- @param n number
function move.down(n)
    std.times(n, turtle.down)
end

--- Turns the turtle 90º to the left *n* times.
---
--- @param n number
function move.turn_left(n)
    std.times(n, turtle.turnLeft)
end

--- Turns the turtle 90º to the right *n* times.
---
--- @param n number
function move.turn_right(n)
    std.times(n, turtle.turnRight)
end

--- Turns the turtle 180º.
function move.turn_around()
    move.turn_right(2)
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

--- Traverses a circle with the specified diameter, invoking the provided action at each block.
---
--- @param diameter number (must be odd)
--- @param filled boolean Whether to fill the circle (true) or just traverse the outline (false)
--- @param action fun(): nil
function move.traverse_circle(diameter, filled, action)
    local radius = (diameter - 1) / 2
    local current_x = 0
    local current_y = 0
    local current_facing = Direction.NORTH

    -- Helper function to turn turtle to face a specific direction
    local function face_direction(target_facing)
        if current_facing == target_facing then
            return
        end

        -- Calculate shortest rotation
        local right_turns = (target_facing - current_facing) % 4
        local left_turns = (current_facing - target_facing) % 4

        if right_turns <= left_turns then
            move.turn_right(right_turns)
        else
            move.turn_left(left_turns)
        end

        current_facing = target_facing
    end

    -- Helper function to move turtle to a specific position
    local function move_to_position(target_x, target_y)
        local dx = target_x - current_x
        local dy = target_y - current_y

        -- Move horizontally first
        if dx > 0 then
            face_direction(Direction.EAST)
            move.forward(dx)
        elseif dx < 0 then
            face_direction(Direction.WEST)
            move.forward(-dx)
        end

        -- Move vertically
        if dy > 0 then
            face_direction(Direction.NORTH)
            move.forward(dy)
        elseif dy < 0 then
            face_direction(Direction.SOUTH)
            move.forward(-dy)
        end

        current_x = target_x
        current_y = target_y
    end

    -- Generate all circle points
    local points = {}
    for x = -radius, radius do
        for y = -radius, radius do
            local distance = math.sqrt(x * x + y * y)
            local should_include = false

            if filled then
                should_include = distance <= radius
            else
                should_include = math.abs(distance - radius) <= 0.5
            end

            if should_include then
                local angle = math.atan(x, y) -- atan(x, y) for clockwise from north
                angle = (angle + 2 * math.pi) % (2 * math.pi) -- Normalize to 0-2π
                table.insert(points, { x = x, y = y, angle = angle })
            end
        end
    end

    -- Sort points by angle (clockwise from north)
    table.sort(points, function(a, b)
        return a.angle < b.angle
    end)

    -- Execute action at each point
    for ix = 1, #points do
        local point = points[ix]
        move_to_position(point.x, point.y)
        action()
    end

    -- Return to center
    move_to_position(0, 0)
    face_direction(Direction.NORTH)
end

return move

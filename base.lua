#!/usr/bin/env lua
local curses = require("curses")

local filename = arg[1] or "untitled.txt"
local lines = {""}
local cy, cx = 1, 1
local mode = "NORMAL"
local status_message = "Ready"

local stdscr = curses.init()
curses.echo(false)
curses.raw(true)
curses.curs_set(1)
stdscr:keypad(true)
stdscr:nodelay(true)

local function render()
    stdscr:erase()
    local height, width = stdscr:getmaxyx()
    local text_height = height - 2

    for row = 1, text_height do
        if row <= #lines then
            stdscr:mvaddstr(row - 1, 0, string.sub(lines[row], 1, width))
        end
    end

    local status = string.format(" Base | %s | MODE: %s | Line: %d/%d Col: %d | %s ", 
        filename, mode, cy, #lines, cx, status_message)
    
    stdscr:attron(curses.A_REVERSE)
    stdscr:mvaddstr(height - 2, 0, string.format("%-" .. width .. "s", status))
    stdscr:attroff(curses.A_REVERSE)

    local cmd_bar = " Interface Shell Mode | Waiting for Rust Command Engine "
    stdscr:mvaddstr(height - 1, 0, string.format("%-" .. width .. "s", cmd_bar))

    stdscr:move(math.min(cy, text_height) - 1, math.min(cx, width) - 1)
    stdscr:refresh()
end

local function handle_key_event(ch)
    print(string.format('{"event": "key", "code": %d, "mode": "%s", "cy": %d, "cx": %d}', ch, mode, cy, cx))
    io.flush()
end

local running = true
while running do
    render()

    local ch = stdscr:getch()
    if ch and ch ~= -1 then
        if ch == 17 then
            running = false
        else
            handle_key_event(ch)
        end
    end
end

curses.done()

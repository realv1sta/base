#!/usr/bin/env lua

local filename = arg[1] or "untitled.txt"
local lines = {""}
local cy, cx = 1, 1
local mode = "NORMAL"
local status_message = "Base Terminal Ready"
local cmd_input = ""

os.execute("stty raw -echo")

local function clear_screen()
    io.write("\27[2J\27[H")
end

local function move_cursor(r, c)
    io.write(string.format("\27[%d;%dH", r, c))
end

local function get_terminal_size()
    local handle = io.popen("stty size")
    local result = handle:read("*a")
    handle:close()
    local h, w = result:match("(%d+)%s+(%d+)")
    return tonumber(h) or 24, tonumber(w) or 80
end

local function render()
    clear_screen()
    local height, width = get_terminal_size()
    local text_height = height - 2

    for row = 1, text_height do
        move_cursor(row, 1)
        if row <= #lines then
            io.write(string.sub(lines[row], 1, width))
        end
    end

    move_cursor(height - 1, 1)
    local status = string.format(" Base | %s | MODE: %s | Pos: %d:%d | %s ", 
        filename, mode, cy, cx, status_message)
    io.write(string.format("\27[7m%-" .. width .. "s\27[0m", status))

    move_cursor(height, 1)
    local cmd_bar = ""
    if mode == "COMMAND" then
        cmd_bar = "BASE > " .. cmd_input
    elseif mode == "NORMAL" then
        cmd_bar = " [:] Cmd Bar | :sv Save | :sp Ship | :ex Exit | :dis Discard | i Insert "
    else
        cmd_bar = " ESC: Normal | Typing... | Base Autosave Active "
    end
    io.write(string.format("%-" .. width .. "s", cmd_bar))

    if mode == "COMMAND" then
        move_cursor(height, math.min(#cmd_input + 8, width))
    else
        move_cursor(math.min(cy, text_height), math.min(cx, width))
    end
    
    io.flush()
end

local function send_command_payload(cmd_type, payload)
    os.execute(string.format('printf \'{"event": "%s", "cmd": "%s", "filename": "%s"}\\n\'', cmd_type, payload, filename))
end

local function execute_base_command(cmd)
    local trimmed = cmd:match("^%s*(.-)%s*$")
    
    if trimmed == "sv" then
        status_message = "Saving file..."
        send_command_payload("base_cmd", "save")
    elseif trimmed == "ex" then
        send_command_payload("base_cmd", "exit")
        return false
    elseif trimmed == "sp" then
        status_message = "Saving and exiting..."
        send_command_payload("base_cmd", "save_and_exit")
        return false
    elseif trimmed == "dis" then
        send_command_payload("base_cmd", "force_exit")
        return false
    elseif trimmed == "clear" then
        lines = {""}
        cy, cx = 1, 1
        status_message = "Buffer cleared"
        send_command_payload("base_cmd", "clear_buffer")
    else
        status_message = "Base Cmd Not Found: " .. trimmed
    end
    return true
end

local function handle_key_input(ch)
    if mode == "COMMAND" then
        if ch == 27 then
            mode = "NORMAL"
            cmd_input = ""
        elseif ch == 10 or ch == 13 then
            local should_continue = execute_base_command(cmd_input)
            cmd_input = ""
            mode = "NORMAL"
            return should_continue
        elseif ch == 127 or ch == 8 then
            if #cmd_input > 0 then
                cmd_input = string.sub(cmd_input, 1, #cmd_input - 1)
            else
                mode = "NORMAL"
            end
        elseif ch >= 32 and ch <= 126 then
            cmd_input = cmd_input .. string.char(ch)
        end
    elseif mode == "NORMAL" then
        if ch == string.byte(":") then
            mode = "COMMAND"
            cmd_input = ""
        elseif ch == string.byte("i") then
            mode = "INSERT"
        else
            os.execute(string.format('printf \'{"event": "key", "code": %d, "mode": "%s", "cy": %d, "cx": %d}\\n\'', ch, mode, cy, cx))
        end
    elseif mode == "INSERT" then
        if ch == 27 then
            mode = "NORMAL"
        else
            os.execute(string.format('printf \'{"event": "key", "code": %d, "mode": "%s", "cy": %d, "cx": %d}\\n\'', ch, mode, cy, cx))
        end
    end
    return true
end

local running = true
while running do
    render()

    local char = io.read(1)
    if char then
        local ch = string.byte(char)
        running = handle_key_input(ch)
    end
end

os.execute("stty sane")
clear_screen()

---@meta

---@alias ccTweaked.os.locale
---| '"ingame"' # The current world time
---| '"utc"' # Get the hour of the day in UTC time
---| '"local"' # Get the hour of the day in the timezone that the server is located in

---@class ccTweaked.os.dateTable
---@field year integer year number
---@field yday integer day of the year
---@field wday integer day of the week
---@field month integer month of the year
---@field day integer day of the month
---@field hour integer hour of the day
---@field min integer Minute of the hour
---@field sec integer Seconds of the minute
---@field isdst boolean If Daylight Saving Time is in effect

---@alias ccTweaked.os.ASCII number An [ASCII code](https://www.rapidtables.com/code/text/ascii-table.html) that corresponds to a character

---@alias ccTweaked.os.event
---| '"alarm"' # Fired when an alarm started with `os.setAlarm()` completes.<hr/>[Official Documentation](https://tweaked.cc/event/alarm.html)
---| '"char"' # Fired when a key is typed on the keyboard. Only captures characters, not control keys.<hr/>[Official Documentation](https://tweaked.cc/event/char.html)
---| '"computer_command"' # Fired when the `/computercraft queue` command is run for the current computer.<hr/>[Official Documentation](https://tweaked.cc/event/computer_command.html)
---| '"disk"' # Fired when a disk is inserted into an adjacent or networked disk drive.<hr/>[Official Documentation](https://tweaked.cc/event/disk.html)
---| '"disk_eject"' # Fired when a disk is removed from an adjacent or networked disk drive.<hr/>[Official Documentation](https://tweaked.cc/event/disk_eject.html)
---| '"http_check"' # Fired when a URL check completes.<hr/>[Official Documentation](https://tweaked.cc/event/http_check.html)
---| '"http_failure"' # Fired when an HTTP request fails.<hr/>[Official Documentation](https://tweaked.cc/event/http_failure.html)
---| '"http_success"' # Fired when an HTTP request succeeds.<hr/>[Official Documentation](https://tweaked.cc/event/http_success.html)
---| '"key"' # Fired when any key is pressed while the terminal is focused. For text input, use a `char` event.<hr/>[Official Documentation](https://tweaked.cc/event/key.html)
---| '"key_up"' # Fired when a key is released (or the terminal is closed while a key was pressed).<hr/>[Official Documentation](https://tweaked.cc/event/key_up.html)
---| '"modem_message"' # Fired when a message is received on an open channel on any modem.<hr/>[Official Documentation](https://tweaked.cc/event/modem_message.html)
---| '"monitor_resize"' # Fired when an adjacent or networked monitor is resized.<hr/>[Official Documentation](https://tweaked.cc/event/monitor_resize.html)
---| '"monitor_touch"' # Fired when an adjacent or networked **advanced** monitor is right-clicked.<hr/>[Official Documentation](https://tweaked.cc/event/monitor_touch.html)
---| '"mouse_click"' # Fired when an advanced/pocket computer is clicked with a mouse.<hr/>[Official Documentation](https://tweaked.cc/event/mouse_click.html)
---| '"mouse_drag"' # Fired when the mouse is moved while a mouse button is being held.<hr/>[Official Documentation](https://tweaked.cc/event/mouse_drag.html)
---| '"mouse_scroll"' # Fired when a mouse wheel is scrolled in the terminal.<hr/>[Official Documentation](https://tweaked.cc/event/mouse_scroll.html)
---| '"mouse_up"' # Fired when a mouse button is released or leaves the terminal.<hr/>[Official Documentation](https://tweaked.cc/event/mouse_up.html)
---| '"paste"' # Fired when text is pasted into the computer.<hr/>[Official Documentation](https://tweaked.cc/event/paste.html)
---| '"peripheral"' # Fired when a peripheral is attached to a side or a modem.<hr/>[Official Documentation](https://tweaked.cc/event/peripheral.html)
---| '"peripheral_detach"' # Fired when a peripheral is removed from a side or modem.<hr/>[Official Documentation](https://tweaked.cc/event/peripheral_detach.html)
---| '"rednet_message"' # Fired when a message is received over rednet.<hr/>[Official Documentation](https://tweaked.cc/event/rednet_message.html)
---| '"redstone"' # Fired when a redstone input on a side is changed.<hr/>[Official Documentation](https://tweaked.cc/event/redstone.html)
---| '"speaker_audio_empty"' # Fired when a speaker's audio has ended.<hr/>[Official Documentation](https://tweaked.cc/event/speaker_audio_empty.html)
---| '"task_complete"' # Fired when an asynchronous completes.<hr/>[Official Documentation](https://tweaked.cc/event/task_complete.html)
---| '"term_resize"' # Fired when the main terminal is resized.<hr/>[Official Documentation](https://tweaked.cc/event/term_resize.html)
---| '"terminate"' # Fired when `CTRL + T` is held down.<hr/>[Official Documentation](https://tweaked.cc/event/terminate.html)
---| '"timer"' # Fired when a timer started with `os.startTimer()` completes.<hr/>[Official Documentation](https://tweaked.cc/event/timer.html)
---| '"turtle_inventory"' # Fired when a turtle's inventory changes.<hr/>[Official Documentation](https://tweaked.cc/event/turtle_inventory.html)
---| '"websocket_closed"' # Fired when a websocket connection is closed.<hr/>[Official Documentation](https://tweaked.cc/event/websocket_closed.html)
---| '"websocket_failure"' # Fired when a websocket connection fails.<hr/>[Official Documentation](https://tweaked.cc/event/websocket_failure.html)
---| '"websocket_message"' # Fired when a websocket message is received.<hr/>[Official Documentation](https://tweaked.cc/event/websocket_message.html)
---| '"websocket_success"' # Fired when a websocket connection succeeds.<hr/>[Official Documentation](https://tweaked.cc/event/websocket_success.html)
---| '"file_transfer"' # Fired when a file is dragged and dropped on an open computer.<hr/>[Official Documentation](https://tweaked.cc/event/file_transfer.html)
---| string

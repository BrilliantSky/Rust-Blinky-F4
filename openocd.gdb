# Not used by the VS Code Debugger, only `cargo run`
target remote :3333
set print asm-demangle on
set print pretty on
load
# monitor tpiu config internal itm.txt uart off 48000000
# monitor itm port 0 on
# break main
# continue
# monitor reset halt

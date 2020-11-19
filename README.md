BlinkyF4/144
============

This project is an example of how to blink an LED on the STM32F429ZI Nucleo144 board.
Debugging is pre-configured for Visual Studio Code on native Linux using the cortex-debug extension.

Based on the discovery project.


Requirements:
-------------
* [cortex-debug plugin](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug)
* [cortex-debug svd plugin](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug-dp-stm32f4)
* [rust support plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
* OpenOCD (other tools may work, see cortex-debug supported debug probes and modify launch.json accordingly).
* STM32F429ZIT6 on Nucleo144 board (this includes 8MHz HSE input and an LED on PB7). Other F4 processors or boards may work with some minor changes.


Build
-----

At the command line:
```
cargo build
```

In vs code: run the default build task (ctrl+shift+B by default).


Debug - Command-line test
-------------------------

In one terminal, start OpenOCD:
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
```
This connects to the board and runs a GDB server. If the connection is successful,
you'll see the ST-link LED flashing green and red.

In another terminal, launch the debug session:
```
cargo run
```
This starts up GDB and connects to the target using ```target remote :3333```

Type ```continue``` to start the program, or type ```next``` to step line by line.
(Note: I have to use ```si``` once to get past the first instruction before using ```next``` otherwise it runs without stopping.)


Debug - VS Code
---------------

Click the green 'run' button under the cortex-debug pane (F5 by default).
The program will be downloaded and start at the reset handler. You can set
a breakpoint somewhere in main or just let it run by pressing F5 again.

Note: Make sure the OpenOCD server is not running as the cortex-debug plugin will start it automatically.


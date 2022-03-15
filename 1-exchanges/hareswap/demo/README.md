# Demos

Make sure to build first by running `./build.sh` in the root folder (up one directory from here)

* Simple Swap
  * Request a buy of 200 "M" tokens for some amount of T tokens (...100 T is quoted)
  * Run: `./simple_swap.sh`
  * It logs a whole bunch and is well commented.  Dig in!

## Other Details

* `baseline.sh` - sets up accounts and things not specific to HareSwap but required for the demos.  
                  Includes hardcoded values instead of parsing resim out. It may break in the 
                  future.
* `logging.sh` - makes the text output a little prettier

## Limitations

These have only been tested on Linux, but good chance they work on macOS too.
On Windows you could use WSL, but it will not work with Powershell (sorry)

GFX Clock
=========

This is a project to control a Gra & Afch NCS314-8C nixie tube v2.2 clock 
with a Raspberry Pi 4B attached through the GRA & AFCH Raspberry Pi Arduino
Shield Adapter ASTRPA v2.1.

The intent is to be able to use the clock to display the time. Also it is fun to use
the clock to display other things, so this project is arranged to be modular and
show other data like wind speed/direction, temperature, pressure, light quality,
and various other things that look cool given the 9 tubes, 6 indicators, and 6 LEDs
this hardware has to offer.

This project is experimental. If you run this project, it might kill you, fry your
equipment, tubes, or completely wipe your brain, so definitely use at your own risk!

Early development notes:

So far I've found very little in terms of documentation about how to drive
the different parts of the clock. The CLITool at https://github.com/afch/NixieClockRaspberryPi
is what I am mainly using for reference. I'm also referencing https://github.com/afch/NixeTubesShieldNCS314_8C
but it is difficult to tell how the Pi to Arduino adapter maps to the GPIO pins.

I'm developing on a Mac, but I set up a Docker image to do the cross-compilation
after failing to find a solid armv7-unknown-linux-gnueabihf toolchain. If this is being compiled on linux,
it might be easier to build locally.

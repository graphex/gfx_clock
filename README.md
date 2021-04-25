GFX Clock
=========

This is a personal project to control my Gra & Afch NCS314-8C v2.2 clock 
with my Raspberry Pi 4B attached through the GRA & AFCH Raspberry Pi Arduino
Shield Adapter ASTRPA v2.1

I am not particularly competent in Rust, GPIO, SPI, I2C, electricity, nixie tubes,
high voltage, or really anything else resembling the necessary knowledge required
to make this project work. If you run this project, it might kill you, fry your
equipment, tubes, or completely wipe your brain, so definitely use at your own risk!

So far I've found very little in terms of documentation about how to drive
the different parts of the clock. The CLITool at https://github.com/afch/NixieClockRaspberryPi
is what I am mainly using for reference. I'm also referencing https://github.com/afch/NixeTubesShieldNCS314_8C
but it is difficult to tell how the Pi to Arduino adapter maps to the GPIO pins.

I'm developing on a Mac, but I set up a Docker image to do the cross-compilation
after failing to find a solid armv7-unknown-linux-gnueabihf toolchain.
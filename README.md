# rustyboy

A Game Boy emulator written in Rust.

## Status

**IMPORTANT**: This emulator is in a very early stage, so no games can be
played yet.

* **CPU**: There are a 3 opcodes missing and the timings of the instructions
  are yet to be tested.
* **MMU**: Only MBC0 ROMs are supported.
* **Interrupts**: Not being handled.
* **Timer**: Only some memory read and write code is done.
* **GPU**: Only some memory read and write code is done, no display.
* **Input**: 0% done, no input is processed.
* **APU**: 0% done, no sound.

## Building

You need to have installed the latest nightly build of Rust, which includes
Cargo.

To build the project run:

```
cargo build
```

## Running

Once the project is built:

```
target/rustyboy /path/to/rom
```

## FAQ

### What games does it run?

None at the moment.

### When will I be able to run games?

This is a side project, so I don't know.

### Your emulator sucks! Where can I find a decent one?

Mimey is a work in progress. In the meantime [let me help you find
one][emulators].

### Why do you want to build yet another Game Boy emulator?

One of my dreams since I first used an emulator was to create one by myself, I
enjoy programming and I think this will be a great learning experience. My goal
is being able to run [Pokémon Red and Blue][red-blue].

Mi [first attempt][mimey] at creating an emulator was a couple of years ago,
but I lost my motivation because using TDD to test a CPU is a little boring.
Now that the emulator has an [active fork][mimey-jojo], I want to start from
scratch with a more relaxed approach.

## Acknowledgements

* [Nintendo][nintendo] for creating the Game Boy.
* [Satoshi Tariji][satoshi-tariji] for creating Pokémon, my favorite Game Boy
  game.
* [eljojo][eljojo] for being the new maintainer of my abandoned emulator.
* [sprocketnes][sprocketnes] for convincing me that Rust is a good language for
  emulator programming.
* The great tutorial of [Imran Nazar][imran-nazar] gave me the motivation to
  start writing [Mimey][mimey], my now abandoned first attempt at creating a
  Game Boy emulator.
* The [Pandocs][pandocs] have really useful information.

## License

MIT.

[emulators]: http://bit.ly/14hVUqL
[red-blue]: http://bulbapedia.bulbagarden.net/wiki/Pokémon_Red_and_Blue_Versions
[nintendo]: http://nintendo.com
[satoshi-tariji]: http://en.wikipedia.org/wiki/Satoshi_Tajiri
[imran-nazar]: http://imrannazar.com/GameBoy-Emulation-in-JavaScript
[pandocs]: http://problemkaputt.de/pandocs.htm
[eljojo]: https://github.com/eljojo
[sprocketnes]: https://github.com/pcwalton/sprocketnes
[mimey]: https://github.com/janogonzalez/mimey
[mimey-jojo]: https://github.com/eljojo/mimey

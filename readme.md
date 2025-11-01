# memutil

this is a small utility to make memory reading easier.

## setup

download a pre-build binary from the releases tab.

alternatively, clone the repository, and run `cargo build --release`.

## patterns

for the read, watch and find commands you can enter a pattern.

these can be either an address, an ida pattern, or a pattern and a pointer chain.

example:

- address: `0x7FFF12345678`
- pattern: `48 83 3D ? ? ? ? 00 0F @3/8`
- pointer chain: `48 83 3D ? ? ? ? 00 0F @3/8 -> 0x210 -> 0x520`

the `@3/8` part in the pattern reads the instruction pointer offset in a `lea` instruction.
the 3 is the offset to the rip offset, the 8 is the instruction size (here 8 bytes).

for pointer chains, the last pointer will be read as an offset.
so here it is assumed to read the address of some data in the pattern,
then read a pointer at 0x210, and finally some data at offset 0x520.

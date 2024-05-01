This is a lib that generates data using the wave function colapse algoritm.
Its mainly intended to be used with games and has been optimized to be very fast.
Given some tile data that can be generated with TileLoader this algorithm can generate map data.
You can run examples/simple.rs to generate a very basic pattern:

┘┌┘┌┼┼┐┌┼─┘│└┐│││┌─┘│ └┘│ └┐└┘  ││ └┘└─┼
┌┼─┘└┘│└┘┌┐│┌┘│└┘│┌┐└─┐┌┼─┐└───┐││ ┌───┘
┘│  ┌─┘ ┌┘└┼┘┌┼─┐└┘└┐┌┼┼┼┐│┌┐ ┌┼┼┼┐│┌───
─┼┐ │  ┌┼─┐└─┼┼┐│  ┌┘└┼┼┘│││└─┼┼┘│││└┐ ┌
┐└┘ └──┼┘ └┐ └┘└┼┐┌┼─┐││┌┼┘│  ││ └┼┘┌┘ └
└┐  ┌┐┌┘┌──┼┐┌┐ └┘└┘┌┼┼┼┘└┐└┐┌┼┼──┘ │ ┌┐
 │┌─┼┼┼─┼┐ └┼┘└─┐┌┐┌┼┘│└──┼─┼┼┼┘ ┌┐ │ └┘
┐│└─┼┘└┐││  └┐ ┌┘└┼┘└┐│ ┌┐└─┼┼┘  └┼┐│  ┌
││┌┐└──┘└┘┌──┘┌┘  └┐ ││ │└─┐│└┐  ┌┘│└──┘
│└┼┘┌┐  ┌─┼┐┌─┼┐┌──┼┐└┼┐│ ┌┘│┌┘ ┌┼─┼┐┌─┐

Features I want to implement:
The ability to pann and colapse new cells.
Being able to provide a map with some colapsed cells and a script to randomly generate these from some conditions.
A weight system using tickets too influence tile selection when colapsing.
The ability to manually edit tiles and recolape neighboring cells.

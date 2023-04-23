# NanoCatPlanetSolution
Solution to the NanoCatPlanet challenge from Aliyun CTF 2023

`cargo run --release -- gui`

This tool was how I solved this challenge, in the GUI, go along increasing offset_x until you get to all lines vertical - offset_x = 5809.
Then increase offset_y until you get the watermark - offset_y = 26.

You can then use the CLI to recover the flag.
`cargo run --release -- cli --orig-img planet_orig.png --watermarked-img planet_fixed.png --output-img flag.png --dx 5809 --dy 26`


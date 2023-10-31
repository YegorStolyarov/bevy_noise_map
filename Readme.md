This is example project, showing how to use Perlin Noise for procedural generation of terrain from 2d tiles.
For different noises explore [Noise repo](https://github.com/Razaekel/noise-rs).

All map-building logic located in ```main.rs``` for easier exploration.
(While running this project, you can move camera with WASD)

I didn't find any for bevy, so I implemented [this solution](https://forum.unity.com/threads/how-to-make-biomes-in-2d-perlin-noise-tilemaps.1157411/#post-7425869) found on Unity forums.

For showcase, I used simple color tiles, feel free to use any tilesets you want, and custom tiling function.

| Version       | Bevy Version |
|---------------|--------------|
| 0.1 (current) | 0.11         |
| -             | -            |

### Level generated with this example:
![Generated level][ex image]

### Same approach level generation in my pet project:
![Live level][ex gif]
<i>For terrain tiles I used terrace inverted noise with threshold for ground\water tiles.
```rust
pub fn terrain_noisemap(seed: u32) -> NoiseMap {
    let perlin = Perlin::new(seed);

    let terrace_inverted = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0)
        .invert_terraces(true);

    PlaneMapBuilder::new(terrace_inverted)
        .set_size(NOISEMAP_SIZE.0, NOISEMAP_SIZE.1)
        .build()
}
```
For flora I used hybridmulti perlin noice</i>
```rust
pub fn flora_noisemap(seed: u32) -> NoiseMap {
    let hybrid_multi = HybridMulti::<Perlin>::new(seed);
    PlaneMapBuilder::new(hybrid_multi)
        .set_size(NOISEMAP_SIZE.0, NOISEMAP_SIZE.1)
        .build()
}
```
Also, I implemented transition detection in resulting noise to place transition tiles correctly,
but this problem lies outside of topic a bit.

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[ex image]: /images/example.png
[ex gif]:   /images/noisemap_preview.gif



# Learning rust, one program at the time!

1. [words](https://github.com/folkol/words), split stdin on words
2. [align](./align), center lines on regex
3. [count](./count), keeps counting, like perpetual seq!
4. [distinct](./distinct), uniq -- but keeps order
5. [(reservoir) sample](./sample), "Algorithm R"
6. [hist](./hist), histogram of numbers on stdin
7. [rand](./rand), generate random numbers
8. [stats](./stats), calculate descriptive statistics for numbers on stdin
9. [prefix](./prefix), prefixes [argument] to all lines on stdin, with placeholders
10. [lorem](./lorem), generates dummy text
11. [pace](./pace), paces std-stdout flow
12. [dbg](./dbg), (pipe)-debugger, like `pv`, but with samples
13. [pick](./pick), pick (and optionally permute) columns
14. [ascii](./ascii), print ascii table -- or convert "queries"
15. [groupby](./groupby), groups lines by field 1
16. [cumsum](./cumsum), cumulative sum
17. [tld](./tld), top-level domains (country)
18. [ends](./ends), "ends", head and tail in one pass
19. [jwtdec](./jwtdec), decodes JWT and prints header and payload
20. [base64url](./base64url), base64 -- but for base64 URL encodings
21. [urldecode](./urldecode), url decoder
22. [urlencode](./urlencode), url encoder
23. [rule](./rule), ported ruler to rust (and renamed it rule)
24. [clock](https://github.com/folkol/misc/tree/master/pixel-poking/clock), ascii-clock
25. [magnifying-glass](https://github.com/folkol/misc/tree/master/pixel-poking/magnifying-glass), old-school magnifying-glass effect
26. [llen](./llen), line lengths
27. [ddiff](./ddiff), ddiff (date-difference)
28. [scatter](./scatter), scatter-plot of numbers from stdin
29. [stars](https://github.com/folkol/misc/tree/master/pixel-poking/stars), screensaver-like stars
30. [gen-strs](./gen-strs), generate strings matching regex
31. [news](./news), read and display news feeds (RSS)
32. [word-chain](https://github.com/folkol/misc/tree/master/pixel-poking/word-chain), http://codekata.com/kata/kata19-word-chains/
33. [pulsating-heart](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/pulsating-heart), animated heart shape using Bevy
34. [hex2dec](./hex2num), convert #HEXCOL to decimals (and fractions)
35. [snake](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/snake), did some snake thing in Bevy
36. [particle-system](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/particle-system), small particle system
37. [animated-shader](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/animated-shader), 2d rectangle with custom fragment shader
38. [mini_gl_fb-still-image](https://github.com/folkol/misc/tree/master/pixel-poking/pixel-poker/mini_gl_fb-still-image), drawing random walk to a mini_gl_fb buffer
39. [mini_gl_fb-animation](https://github.com/folkol/misc/tree/master/pixel-poking/pixel-poker/mini_gl_fb-animation), drawing random walk -- animated
40. [bouncing_pixel](https://github.com/folkol/misc/tree/master/pixel-poking/pixel-poker/bouncing_pixel), bounce a pixel (with trail) around with winit + Pixel
41. [animated-sine-wav](https://github.com/folkol/tutorials/tree/master/nannou-simple-window), animated sine wave with nannou
42. [mandelbrot](https://github.com/folkol/misc/tree/master/pixel-poking/pixel-poker/mandelbrot), drew the mandelbrot fractal with minifb. Best so far!
43. [map-range](./map-range), map-range shell filter
44. [breakthrough](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/breakthrough), break-out-ish
45. [metaballs](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/metaballs), a green metaball!
46. [rainbow](https://github.com/folkol/misc/tree/master/pixel-poking/rainbow), spectrum-to-cie-to-rgb mapping
47. [raycaster](https://github.com/folkol/misc/tree/master/pixel-poking/raycaster), wolfenstein-style raycaster
48. [ripples](https://github.com/folkol/misc/tree/master/pixel-poking/ripples), some sine-ripples
49. [filter-speed-comparison](https://github.com/folkol/misc/tree/master/pixel-poking/filter-performance), comparing basic unix filter in various languages
50. [interactive-fractal](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/interactive-fractal), zoomable mandelbrot
51. [bevy-crash-course-tutorial](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/bevy-crash-course), followed some tutorial. Physics + sounds.
52. [shooter-poc](https://github.com/folkol/misc/tree/master/pixel-poking/bunch-o-bevy-apps/shooter), some basic shooter mechanics PoC
53. [ued](./ued), micro-ed, for editing a single line in a file. WIP.
54. [beep](./beep), generating some noise with rodio
55. [rotating-cube](https://github.com/folkol/misc/tree/master/pixel-poking/rotating-cube), old-school rotating cube!
56. [interer-sort](https://github.com/folkol/misc/tree/master/integer-sort), integer-sort and criterion-comparison
57. [voronoi](https://github.com/folkol/misc/tree/master/bunch-o-bevy-apps), animated voronoi tesselation
58. [sirenpinski-triangle](https://github.com/folkol/misc/tree/master/pixel-poking/sierpinski-triangle), animated sierpinksi triangle
59. [rgb-palette](https://github.com/folkol/misc/tree/master/pixel-poking/rgb-palette), (0)rgb colors in bit order
60. [merge](./merge), merge already sorted files, like sort -m
61. [bgrep](./bgrep), binary search text file for line beginning with pattern
62. [num-to-bin](./num-to-bin), parse number from ascii to binary
63. [bin-to-num](./bin-to-num), convert binary to ascii. Only LE u32 for now.
64. [roll](https://github.com/folkol/apl-inspired-filters/tree/master/roll), bunch of unix filters inspired by [APL](https://aplwiki.com/wiki/Mnemonics)
65. [ceiling](https://github.com/folkol/apl-inspired-filters/tree/master/upstile), added monadic upstile (ceiling) and started on input-parsing as lib
66. [maximum](https://github.com/folkol/apl-inspired-filters/tree/master/upstile), added dyadic upstile (maximum), no scalar extension yet
67. [plus](https://github.com/folkol/apl-inspired-filters/tree/master/plus), added plus, but with monadic to be inc rather than conjugate
68. [lcm](https://github.com/folkol/apl-inspired-filters/tree/master/lcm), added lcm, least common multiplier
69. [bang](https://github.com/folkol/apl-inspired-filters/tree/master/bang), added bang/exclamation mark (factorial vs binomial)
70. [deal](https://github.com/folkol/apl-inspired-filters/tree/master/deal), added dyadic roll (deal)
71. [decode](https://github.com/folkol/apl-inspired-filters/tree/master/decode), added decode
72. [multiply](https://github.com/folkol/apl-inspired-filters/tree/master/multiply), added multiply
73. [divide](https://github.com/folkol/apl-inspired-filters/tree/master/divide), added divide
74. [encode](https://github.com/folkol/apl-inspired-filters/tree/master/encode), added encode
75. [circle](https://github.com/folkol/apl-inspired-filters/tree/master/circle), added monadic circle
76. [circle](https://github.com/folkol/apl-inspired-filters/tree/master/circle), added dyadic circle
77. [equal](https://github.com/folkol/apl-inspired-filters/tree/master/equal), added equal
78. [maze-generator](https://github.com/folkol/misc/tree/master/pixel-poking/maze-generator), maze-generator
79. [not](https://github.com/folkol/apl-inspired-filters/tree/master/not), added monadic not
80. [not](https://github.com/folkol/apl-inspired-filters/tree/master/not), added dyadic not
81. [power](https://github.com/folkol/apl-inspired-filters/tree/master/power), added power (e** / x \** y)
82. [drunken-bishop](https://github.com/folkol/misc/tree/master/drunken-bishop/), (animated) drunken bishop
83. [casey-circle](https://github.com/folkol/misc/tree/master/casey-circle/), ported Casey's circle drawing algo
84. [casey-circle](https://github.com/folkol/misc/tree/master/pixel-poking/casey-circle/), Casey's Circle, animated
85. [actix-proto](https://github.com/folkol/misc/tree/master/web-server-benchmarks/json_vs_protobuf/actix_web), Protobuf vs JSON in Actix Web
86. [ping-server-comparison](https://github.com/folkol/misc/tree/master/web-server-benchmarks/ping-server/), ping server comparison
87. [bresenham+unsafe](https://github.com/folkol/misc/tree/master/pixel-poking/bresenham/), bresenham + unsafe static mutation
88. [rust-collections-performance](https://github.com/folkol/misc/tree/master/rust-collections-performance/), some collection comparions + criterion
89. [macro-rules](https://github.com/folkol/misc/tree/master/rust-collections-performance/), tried macro_rules
90. [perlin-noise](https://github.com/folkol/misc/tree/master/pixel-poking/perlin/), 2d perlin noise
91. [matrix](https://github.com/folkol/misc/tree/master/matrix/), matrix-like animation (ascii)
92. [drop](https://github.com/folkol/apl-inspired-filters/tree/master/drop), drop
93. [floor](https://github.com/folkol/apl-inspired-filters/tree/master/floor), floor
94. [ge](https://github.com/folkol/apl-inspired-filters/tree/master/ge), ge
95. [residue](https://github.com/folkol/apl-inspired-filters/tree/master/residue), residue

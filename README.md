# discauterizer
Small program designed to make videos sendable on discord using ffmpeg to get them small enough
It essentially an ffmpeg wrapper because im lazy as fuck and cant be fucked calculating the bitrates anymore

Discord + Cauterize, real genius name I know
```
Usage: discauterizer.exe [OPTIONS] --input <FILE>

Options:
  -i, --input <FILE>
  -s, --size <SIZE>      [default: 500]
  -b, --bake <BAKE>        [default: 1]
  -l, --leeway <LEEWAY>  [default: 50]
  -o, --output <FILE>    [default: output.mp4]
  -h, --help             Print help
  -V, --version          Print version
```

`--input` is the only required argument, it is the path to the video file you want to compress

`--size` is the target size in megabytes, the program will try to get as close to this as possible

`--bake` whether or not to bake in a subtitle stream pulled from the source video

`--leeway` is a reduction to the target size, the program will try to get the video to be `size - leeway` MB this is because I'm too lazy to make the program get the video to be exactly the target size, its only a config because if you really need a specific size. To disable this just set it to 0

`--output` is the path to the output file, defaults to `output.mp4`


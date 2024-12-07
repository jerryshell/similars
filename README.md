# similaRS

_similaRS_ is a tool for calculating image similarity written in Rust 🦀

## How to use

```bash
cargo install --git https://github.com/jerryshell/similars
```

```
Usage: similars_cli [OPTIONS] --image-x-path <IMAGE_X_PATH> --image-y-path <IMAGE_Y_PATH>

Options:
  -x, --image-x-path <IMAGE_X_PATH>
  -y, --image-y-path <IMAGE_Y_PATH>
      --width <HAMMING_WIDTH>        [default: 8]
      --height <HAMMING_HEIGHT>      [default: 8]
  -d, --show-distance                By default, the output is the percentile of similarity, or the Hamming distance if --show-distance is explicitly specified.
  -h, --help                         Print help
  -V, --version                      Print version
```

## Example

By default, the output is the percentile of similarity

```bash
similars_cli -x example_img1.jpg -y example_img2.jpg
# 95.3125
```

If `-d` is explicitly specified, the output is Hamming distance

```bash
similars_cli -d -x example_img1.jpg -y example_img2.jpg
# 3
```

## References

- [https://www.ruanyifeng.com/blog/2011/07/principle_of_similar_image_search.html](https://www.ruanyifeng.com/blog/2011/07/principle_of_similar_image_search.html)

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)

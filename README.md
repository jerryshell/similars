# similaRS

similaRS is a tool for calculating picture similarity. Written in Rust.

## Usage

```
$ ./similars_cli --help
similars_cli 0.1.0

USAGE:
    similars_cli [OPTIONS] --image-x-path <IMAGE_X_PATH> --image-y-path <IMAGE_Y_PATH>

OPTIONS:
    -d, --distance-flag
            By default, the output is a percentile of similarity, and if this parameter is
            explicitly specified, the output is hamming distance.

    -h, --hamming-height <HAMMING_HEIGHT>
            [default: 8]

        --help
            Print help information

    -V, --version
            Print version information

    -w, --hamming-width <HAMMING_WIDTH>
            [default: 8]

    -x, --image-x-path <IMAGE_X_PATH>
            

    -y, --image-y-path <IMAGE_Y_PATH>
```

## Example

By default, the output is a percentile of similarity.

```
$ ./similars_cli -x example_img1.jpg -y example_img2.jpg
87.5
```

If `-d` parameter is explicitly specified, the output is hamming distance.

```
$ ./similars_cli -d -x example_img1.jpg -y example_img2.jpg
8
```

## References

* [https://www.ruanyifeng.com/blog/2011/07/principle_of_similar_image_search.html](https://www.ruanyifeng.com/blog/2011/07/principle_of_similar_image_search.html)

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)

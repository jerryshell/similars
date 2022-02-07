# similaRS

similaRS is a tool for calculating picture similarity. Written in Rust.

## Usage

```
$ ./similars --help
similars 0.1.0
github.com/jerryshell/similars

USAGE:
    similars [OPTIONS] --image-x-path <IMAGE_X_PATH> --image-y-path <IMAGE_Y_PATH>

OPTIONS:
    -h, --convert-height <CONVERT_HEIGHT>    [default: 8]
        --help                               Print help information
    -V, --version                            Print version information
    -w, --convert-width <CONVERT_WIDTH>      [default: 8]
    -x, --image-x-path <IMAGE_X_PATH>        
    -y, --image-y-path <IMAGE_Y_PATH>
```

## Example

```
$ ./similars -x example_img1.jpg -y example_img2.jpg
8
```

```
$ ./similars -x example_img1.jpg -y example_img1.jpg
0
```

## References

* [https://www.ruanyifeng.com/blog/2011/07/principle_of_similar_image_search.html](https://www.ruanyifeng.com/blog/2011/07/principle_of_similar_image_search.html)

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)

# SVG to PNG
This repository was created to convert SVG files to PNG or to create a Dynamic Image type compatible with the [image crate](https://docs.rs/image/latest/image).

## Dependency
```
[dependencies]
image = { version = "0.24.7"}
rsvg = "0.4.0"
cairo-rs = "0.4.1"
```

You must use version 0.4.1 or higher of cairo-rs. This is because the version of cairo-rs used in rsvg is 0.4.1, and if the version changes, the type changes, which can cause problems.

## How to use

https://github.com/jinseok9338/svg_to_png/assets/27854958/e2e3451e-8d92-43e1-8f15-aee097afca34


```
let handler = get_svg_handler(Path::new(TEST_FILE_PATH));
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        println!("{:?}", handler);
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        println!("{:?}", surface);
        let _ = save_png_to_path(&Path::new("./src/assets/example.png"), &surface.unwrap());
```
As above, you can create a Handle with get_svg_handler, create an ImageSurface with render_image, and then save the PNG file using save_png_to_path.

```
let handler = get_svg_handler(Path::new(TEST_FILE_PATH));
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        let image = make_surface_into_dynamic_image(&mut surface.unwrap());
        assert_eq!(image.is_ok(), true);
```
Or you can call make_surface_into_dynamic_image to create a DynamicImage type. → After this, you can use the image crate to proceed with webp or image resizing.

1. Tested Multiple svg files for stability
2. Tested to see if it can be converted into webp


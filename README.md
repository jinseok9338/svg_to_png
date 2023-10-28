# SVG to PNG
이 리포는 svg 파일을 png 로 바꾸어 저장 하거나 [image crate](https://docs.rs/image/latest/image) 에 호환되는 Dynamic Image 타입을 만들기 위해서 생성한 리포 입니다. 

## Dependency

```rust
[dependencies]
image = { version = "0.24.7"}
rsvg = "0.4.0"
cairo-rs = "0.4.1"
```

cairo-rs 의 버전은 0.4.1 버전이상을 사용해야 합니다. 이는 rsvg 에서 사용하는 cairo-rs 버전이 0.4.1 이라 버전이 바뀌면 타입이 변하기 때문에 문제가 됩니다. 

## 사용방법




https://github.com/jinseok9338/svg_to_png/assets/27854958/e2e3451e-8d92-43e1-8f15-aee097afca34


[xxo3uv.mp4]([https://github.com/jinseok9338/svg_to_png/assets/27854958/e2e3451e-8d92-43e1-8f15-aee097afca34])

```rust

let handler = get_svg_handler(Path::new(TEST_FILE_PATH));
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        println!("{:?}", handler);
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        println!("{:?}", surface);
        let _ = save_png_to_path(&Path::new("./src/assets/example.png"), &surface.unwrap());
```

위와 같이 `get_svg_handler` 로 `Handle` 을 만든후에 `render_image` 를 통해서 `ImageSurface` 를 만든후 `save_png_to_path` 를 이용하여 png 파일을 저장 하면 된다.

```rust
let handler = get_svg_handler(Path::new(TEST_FILE_PATH));
        assert_eq!(handler.is_ok(), true);
        let handler = handler.unwrap();
        let surface = render_image(handler.width, handler.height, handler.handle, None);
        assert_eq!(surface.is_ok(), true);
        let image = make_surface_into_dynamic_image(&mut surface.unwrap());
        assert_eq!(image.is_ok(), true);
```

혹은 make_surface_into_dynamic_image 를 호출 하여서 DynamicImage 타입을 만들 수도 있다. → 이후  image crate 를 이용하여 webp 혹은 이미지 리사이징 등을 진행 할 수 있다.

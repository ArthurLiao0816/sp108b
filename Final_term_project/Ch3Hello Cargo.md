# [Hello Cargo](README.md#rust-learning)
---

## process

### 1.  `cd` to target directory within Microsoft PowerShell

### 2.  enter `cargo --version` to make sure `cargo` is available

### 3.  enter `cargo new hello_cargo` to create a cargo directory

### 4.  `cd` to `hello_cargo` and `ls` it
* you'll see...<br><br>
![cargo_screenshot_creat_file](Picture/cargo_screenshot_creat_file.png)<br><br>
* this step creates one directory and two files in `hello_cargo` directory

### 5. Check out `Cargo.toml`
* you'll see...<br><br>
    ```
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    authors = ["ArthurLiao0816 <ArthurLiao0816@yahoo.com>"]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    ```

### 6.  `cd` to `src` directory and run the file inside it
* result<br><br>
![cargo_screenshot_compile_src_main](Picture/cargo_screenshot_compile_src_main.png)

## [References](References.md#Ch3.)
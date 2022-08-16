# Pokemon Indexer

A simple Rust program to download all of the **15,000** pokemon images on [pkmncards](https://pkmncards.com/?s=).

<p align="center">
    <img src="./pokemon.png" />
</p>

There is about **252** pages on website at the time of creating of this PR every page has about **60** image. the output of the program should be in a folder called `output` which should include the all the pages represented as directories and every directoy has the 60 images.

```bash
--ouptut
  |
  ------- page (idx 1..=252)
            |
            -----image (idx 1..=60).png

```

# How it works?

I am using **tokio** to be able to spen up multiple asynchronous tasks. Due to the network limitation I am running about `5` tasks for `5` page and then another `60` tasks for each page this is about `5 * 60 = 360` image running at the same time. Becuase we have 252 page it will take about 51 iterations to download all of the images.

This process might take some time. You can consider running the program as a **backgorund process** using **[pm2](https://pm2.keymetrics.io/)**.

# How to run?

You can pass the following arguments
| Argument | desc| type | default | note|
| --------- | ----| ----- | ------- | ----|
| page (-p) | Number of pages to process at the same time | `u8` | `5` | |
| json (-j)| Save all images urls into a json file| `bool` | `flase` | Required to be able to run the web server|
| save-images (-s)| Save all fetched images| `bool` | `flase` | |
| addr | server binding address| `socketAddr` | `127.0.0.1:8080` | |

### examples

```bash
# Only output a json file with all of the urls
[exe] -p 12 -j
# Process 20 pages in parallel ans save all of the output images
[exe] -p 20 -s
# Run the server on a different port
[exe] --addr 127.0.0.1:3000
# Get all commands
[exe] -h
```

```bash

USAGE:
    pokemon-cards.exe [OPTIONS]

OPTIONS:
        --addr <ADDR>      [default: 127.0.0.1:8080]
    -h, --help             Print help information
    -j, --json
    -p, --pages <PAGES>    [default: 5]
    -s, --save-images
    -V, --version          Print version information
```

## Run with one of the previous arguments

1. build the binary to your target platform using

   ```bash
   cargo run --release
   ```

2. Run the program using one of these two ways

   a.

   ```bash
    /target/release/pokemon-cards #.exe for windows
   ```

   b. Use **pm2**. You will need to have NodeJS installed and then you can install pm2

   ```bash
   npm install -g pm2
   ```

   Check your installation by running

   ```bash
   pm2 --version
   ```

   Run the binary as background script. All the configs to run the program is found in `ecosystem.config.js` file

   ```bash
    pm2 start ecosystem.config.js
   ```

   To check the logs of the programs run

   ```bash
    pm2 log
   ```

   To stop or delete or restart the program use

   ```bash
   pm2 stop|delete|restart ecosystem.config.js
   ```

# API

`GET /api/v1/pokemon`

| Query     | type  | default | desc                |
| --------- | ----- | ------- | ------------------- |
| page      | `i32` | `1`     | current page nubmer |
| page_size | `i32` | `10`    | Items perpage       |

### Example

```
GET /api/v1/pokemon?page=1&page_size=20
```

response

```json
{
  "urls": [
    /*Array of all the urls*/
  ],
  "count": 14400,
  "num_of_pages": 1200
}
```

## Example website

<video src="./example.mp4" atuoplay />

## To Build Program

1. Clone this repository with ```git clone```.

2. Build project with ```cargo build```.

### To Encrypt an Image
Run ``cargo run encrypt [input image path] [output image path] [key 1] [key 2]``. 

(eg. ```cargo run encrypt InputImages/Forest.jpg ImageBin/combined.png 0.1 0.3```)

### To Decrypt an Image
Run ``cargo run decrypt [input image path] [output image path] [key 1] [key 2]``. 

(eg. ```cargo run decrypt ImageBin/combined.png ImageBin/decrypted.png 0.1 0.3```)

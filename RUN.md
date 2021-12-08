## To Build Program

1. Clone this repository with ```git clone```.

2. Build project with ```cargo build```.

### To Encrypt an Image
Run ``cargo run encrypt [input image path] [output image path]``. 

(eg. ```cargo run encrypt InputImages/Forest.jpg ImageBin/combined.png```)

### To Decrypt an Image
Run ``cargo run decrypt [input image path] [output image path]``. 

(eg. ```cargo run decrypt ImageBin/combined.png ImageBin/decrypted.png```)

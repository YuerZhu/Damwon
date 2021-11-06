Group Name: Damwon

Names (netID): 
Phyllis Wang (pjwang3), 
Mohammad Zayyad (mzayy2), 
David Zhu (yuerzhu2)

Project: Image Encryption and Decryption

Introduction:

Our project is to program a way to encrypt and decrypt images in Rust. 

Our goal would be to make it so that a user is able to submit an image to be encrypted, or to send in an encrypted image to be decrypted, outputting the original image. We chose this project because it would be an interesting introduction to the idea of encryption on computers, despite our own program not being as sophisticated as existing encryptors. We hope that this project will help provide us with a deeper understanding of the process of encryption as well. Furthermore, this project could be a cool way for people to send “secret” encrypted images to one another and be able to decrypt it.

System Overview and Roadmap: (To be implemented in this general order)

Structures:

\t  Pixel, containing rgb values
  
  Function to determine the correct cipher, possibly by comparing differences of adjacent pixels, or other methods
  
Encryption: 

  Read and store pixels
  
  Divide pixels into multiple rows, columns, or 2d arrays
  
  Encrypt rgb values according to use-selected cipher
  
  Piece together the pixels and output image
  
    
Decryption:

  Read and store pixels
  
  Set up multiple threads to change and output each pixel from a provided cipher
  
  Compare different outputs to determine the best decryption
  
  Piece together the pixels and output image 
  

Possible Challenges:

References: 

Add-ons:

Conduct different ciphers on different sections of image

Test among a set of different ciphers for the correct cipher

Imbed information of encryption cipher into rgb values so that it acts as a public key for the receiver to select the correct cipher to decrypt the image 

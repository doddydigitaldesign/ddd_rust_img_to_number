# ddd_rust_img_to_number

A simple CLI tool for reducing an image to a number. This can be used for seeding PRNGs, generating unique identifiers and so on, since the outputted numbers are quite unique.

### Algorithm

1. Open the image file
2. Reduce each pixel to a number
3. Sum over the pixels of the image

find ./static/images -name '*.png' -or -name '*.jpg' | xargs -I {} cwebp {} -o {}.webp

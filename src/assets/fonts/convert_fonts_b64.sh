shopt -s nullglob

for file in ./*
do
    base64 "$file" > "$file.b64"
done

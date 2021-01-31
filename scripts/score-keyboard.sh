cargo +nightly build --release --out-dir=./target/publish -Z unstable-options

clear

./target/publish/genetic_keyboard.exe \
    -t ./data/0.05-123.result.txt \
    -k ./data/keyboard.json \
    -p 0 \
    -c 0 \
    -g 0 \
    -m 0 \
    --results-count 100 \
    --repeats-count 0

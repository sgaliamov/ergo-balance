cargo +nightly build --release --out-dir=./target/publish -Z unstable-options

clear

./target/publish/genetic_keyboard.exe \
    -t ./data/0.1-29.result.txt \
    -k ./data/keyboard.json \
    -p 0 \
    -c 0 \
    -g 0 \
    -m 1 \
    --results-count 100 \
    --repeats-count 0

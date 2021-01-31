cargo +nightly build --release --out-dir=./target/publish -Z unstable-options

clear

./target/publish/genetic_keyboard.exe \
    -t ./data/0.1-29.result.txt \
    -k ./data/keyboard.json \
    -p 200 \
    -c 5 \
    -g 10000 \
    -m 4 \
    --results-count 20 \
    --repeats-count 100

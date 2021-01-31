cargo +nightly build --release --out-dir=./target/publish -Z unstable-options

clear

./target/publish/genetic_keyboard.exe \
    -t ./data/samples/0.05-123.result.txt \
    -k ./data/keyboard.json \
    -p 500 \
    -c 5 \
    -g 10000 \
    -m 4 \
    --results-count 30 \
    --repeats-count 100

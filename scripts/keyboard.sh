cargo +nightly build --release --out-dir=./target/publish -Z unstable-options

clear

./target/publish/genetic_keyboard.exe \
    -t ./data/samples/0.05-367.result.txt \
    -k ./data/keyboard.json \
    -p 500 \
    -c 5 \
    -g 10000 \
    -m 5 \
    --results-count 35 \
    --repeats-count 150

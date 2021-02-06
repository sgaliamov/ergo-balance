cargo +nightly build --release --out-dir=./target/publish -Z unstable-options

clear

./target/publish/genetic_keyboard.exe \
    -t ./data/samples/0.01-10272.result.txt \
    -k ./data/keyboard.json \
    -p 600 \
    -c 4 \
    -g 10000 \
    -m 5 \
    --results-count 30 \
    --repeats-count 100

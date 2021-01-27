clear

./target/publish/genetic_keyboard.exe \
    -k ./data/keyboard.dev.json \
    -p 100 \
    -c 4 \
    -g 10000 \
    -m 4 \
    --results-count 20 \
    --repeats-count 100

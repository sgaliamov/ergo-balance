clear

./target/publish/genetic_letters.exe \
    -d ./data/digraphs.json \
    -p 1000 \
    -c 20 \
    -g 10000 \
    -m 4 \
    -l 15 \
    --frozen-left ser \
    --results-count 20 \
    --repeats-count 500

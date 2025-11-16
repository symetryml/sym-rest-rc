

export SML_SK="YOUR_SK for your user"
export SML_CONFIG_FILE=etc/rc.conf

#./target/debug/sym-rest-rc --file=./etc/rc.conf

./target/debug/sym-rest-rc create --type=cpu --name test001 --hist

# Use Rest API to learn
./target/debug/sym-rest-rc --config=etc/rc.conf learn \
    --project=test-rust \
    --file=/datasets/c/IrisFiles/Iris_rtlm.csv \
    --types="C,C,C,C,B,B,B,B,B,B,B,B,B,B,B"

# Use Web Socket to learn:
./target/debug/sym-rest-rc --config=etc/rc.conf learn \
    --project=test-rust \
    --file=./dem-websocat/iris2rows.csv \
    --types="C,C,C,C,B,B,B,B,B,B,B,B,B,B,B" \
    --use-ws

# build model
./target/debug/sym-rest-rc build --config=./etc/rc.conf \
  --project=test-rust --name=model1 --type=lda --targets="12" --inputs="0,1,2,3"

./target/debug/sym-rest-rc build --config=./etc/rc.conf \
  --project=test-rust --name=model2 --type=lda --target-names="Iris_setosa" \
  --input-names="sepal_length,sepal_width,petal_length,petal_width"


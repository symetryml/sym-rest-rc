

export SML_SK="YOUR_SK for your user"
export SML_CONFIG_FILE=etc/rc.conf

#./target/debug/sym-rest-rc --file=./etc/rc.conf

./sym-rest-rc project create --type=cpu --name test001 --hist

# Use Rest API to learn
./sym-rest-rc --config=etc/rc.conf learn \
    --project=test001 \
    --file=/datasets/c/IrisFiles/Iris_rtlm.csv \
    --types="C,C,C,C,B,B,B,B,B,B,B,B,B,B,B"

# Use Web Socket to learn:
./sym-rest-rc --config=etc/rc.conf learn \
    --project=test001 \
    --file=./dem-websocat/iris2rows.csv \
    --types="C,C,C,C,B,B,B,B,B,B,B,B,B,B,B" \
    --use-ws

# build model
./sym-rest-rc model build --config=./etc/rc.conf \
  --project=test001 --name=model1 --type=lda --targets="12" --inputs="0,1,2,3"

# Check model build job (replace --id=15 with good id
./sym-rest-rc job --id=15

./sym-rest-rc model build --config=./etc/rc.conf \
  --project=test001 --name=model2 --type=lda --target-names="Iris_setosa" \
  --input-names="sepal_length,sepal_width,petal_length,petal_width"

# Make prediction
./sym-rest-rc model predict --project=test001 --model=model1 --file="./iris2rows.csv"

# delete model
./sym-rest-rc model delete --project=test001 --model=model1

# delete project
./sym-rest-rc project delete --project=test001



#
# It is assumed that this script is run from the sym-rest-rc folder
#

export SML_SK="YOUR_SK for your user"
export SML_CONFIG_FILE=etc/rc.conf

#./target/debug/sym-rest-rc --file=./etc/rc.conf

./sym-rest-rc project create --type=cpu --name test001 --hist

# Use Rest API to learn
./sym-rest-rc --config=etc/rc.conf learn \
    --project=test001 \
    --file=./data/Iris_rtlm.csv \
    --types="C,C,C,C,B,B,B,B,B,B,B,B,B,B,B"

# Use Web Socket to learn:
./sym-rest-rc --config=etc/rc.conf learn \
    --project=test001 \
    --file=./data/iris2rows.csv \
    --types="C,C,C,C,B,B,B,B,B,B,B,B,B,B,B" \
    --use-ws

# build model
./sym-rest-rc model build --config=./etc/rc.conf \
  --project=test001 --name=model1 --type=lda --targets="12" --inputs="0,1,2,3"

# build model with autoselect #1
./sym-rest-rc model autoselect \
    --project=test001 \
    --task=binary_classifier \
    --model=as1 \
    --val-file=./data/Iris_rtlm.csv \
    --target-names="Iris_setosa" \
    --input-names="sepal_length,sepal_width,petal_length,petal_width,sepal_lengt_b1,sepal_lengt_b2,sepal_width_b1,sepal_width_b2,petal_length_b1,petal_length_b2,petal_width_b1,petal_width_b2" \
    --params='autoselect_grid_type=autoselect_grid_type_normal,selector_type=selector_type_fw_bw,selector_max_iterations=5,autoselect_thread_pool_size=4,sml_model_assessment_type=auc'

# build model with autoselect #2
./sym-rest-rc model autoselect \
    --project=test001 \
    --task=binary_classifier \
    --model=as2 \
    --val-file=./data/Iris_rtlm.csv \
    --targets="13" \
    --inputs="0,1,2,3,4,5,6,7,8,9,10,11,12" \
    --params='autoselect_grid_type=autoselect_grid_type_normal,selector_type=selector_type_fw_bw,selector_max_iterations=5,autoselect_thread_pool_size=4,sml_model_assessment_type=auc'


# Check model build job (replace --id=15 with good id
./sym-rest-rc job --id=12

./sym-rest-rc model build --config=./etc/rc.conf \
  --project=test001 --name=model2 --type=lda --target-names="Iris_setosa" \
  --input-names="sepal_length,sepal_width,petal_length,petal_width"

# Make prediction
./sym-rest-rc model predict --project=test001 --model=as1 --file="./data/iris2rows.csv"

# Project Info
./sym-rest-rc project info --project=test001

# Models Info
./sym-rest-rc model info --model=as1 --project=test001
./sym-rest-rc model info --model=model1 --project=test001

# delete model
./sym-rest-rc model delete --project=test001 --model=model1
./sym-rest-rc model delete --project=test001 --model=as1

# delete project
./sym-rest-rc project delete --project=test001



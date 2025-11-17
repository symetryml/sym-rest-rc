


# Useful environment Variables

`export SML_CONFIG_FILE` so that we do not need to use `--config` with every commands:
```
export SML_CONFIG_FILE=./etc/rc.conf
export SML_SK="YOUR_SK for your user"
```

# Project

## Create a Project
This create a new project.
```
./sym-rest-rc project create --name=project1 --params="enable_histogram=true" --type=cpu
```

## Project Information

Returns information about a project.
```
./sym-rest-rc project info --name=project1
```

## Delete a Project

```
./sym-rest-rc project delete --project=project1
```

# Learn / Push Data to a SymetryProject

This learn or push data to a project, use web socket
```
./sym-rest-rc learn --use-ws=true --file="./iris2rows.csv" --types=C,C,C,C,B,B,B,B,B,B,B,B,B,B,B
```

# Model

## Build a Model

this create a new machine learning model, use rest API (asynchronous - returns job id)

```
./sym-rest-rc model build --project=project1 --name=model1 --type=hba --targets="13" --inputs="0,1,2,3" --params="param1=somevalue,param2=somevalue2"
./sym-rest-rc model build --project=project1 --name=model2 --type=hba --target-names="Iris_setosa" --input-names="sepal_length,sepal_width,petal_length,petal_width" --params="param1=somevalue,param2=somevalue2"
```

## Auto Select Model
```
./sym-rest-rc model autoselect \
    --project=nhtest \
    --task=binary_classifier \
    --model=as1 \
    --val-file=/datasets/c/IrisFiles/Iris_rtlm.csv \
    --target-names="Iris_setosa" \
    --input-names="sepal_length,sepal_width,petal_length,petal_width,sepal_lengt_b1,sepal_lengt_b2,sepal_width_b1,sepal_width_b2,petal_length_b1,petal_length_b2,petal_width_b1,petal_width_b2" \
    --params='autoselect_grid_type=autoselect_grid_type_normal,selector_type=selector_type_fw_bw,selector_max_iterations=5,autoselect_thread_pool_size=4,sml_model_assessment_type=auc'

./sym-rest-rc model autoselect \
    --project=nhtest \
    --task=binary_classifier \
    --model=as2 \
    --val-file=/datasets/c/IrisFiles/Iris_rtlm.csv \
    --targets="13" \
    --inputs="0,1,2,3,4,5,6,7,8,9,10,11,12" \
    --params='autoselect_grid_type=autoselect_grid_type_normal,selector_type=selector_type_fw_bw,selector_max_iterations=5,autoselect_thread_pool_size=4,sml_model_assessment_type=auc'

```

## Make Predictions

This make prediction with a model

### Predictions Using REST API with JSON dataframe
```
./sym-rest-rc model predict --project=test-rust --model=model2 --df='{"attributeNames":["sepal_length","sepal_width","petal_length","petal_width"],"data":[["4.3","3","1.1","0.1"]],"attributeTypes":["C","C","C","C"]}'
```

### Predictions Using REST API with CSV file
```
./sym-rest-rc model predict --project=test-rust --model=model2 --file="./iris2rows.csv"
```

### Predictions Using WebSocket with CSV file
```
./sym-rest-rc model predict --project=test-rust --model=model2 --file="./iris2rows.csv" --use-ws
```

## Create EVT Wrapper TBD
```
NOT IMPLEMENTED
url:
POST /{cid}/projects/{pid}/{modelid}/evtwrapper
body:
map<string,string>

./sym-rest-rc model evtwrapper --project=test-rust --model=model5  --params="param1=somevalue,param2=somevalue2"
```

## Delete EVT Wrapper TBD
```
NOT IMPLEMENTED
url:
DELETE /{cid}/projects/{pid}/{modelid}/evtwrapper
./sym-rest-rc model evtwrapper --delete --project=test-rust --model=model5
```
## Model Info
```
./sym-rest-rc model info --name=model2 --project=test-rust
```


## Delete model
```
./sym-rest-rc model delete --name=model2 --project=test-rust
```

# Job Status
check the status of an asynchronous job (like building a model)
```
./sym-rest-rc job --id=8
```



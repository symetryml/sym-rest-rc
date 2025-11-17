


# Useful environment Variables

export SML_CONFIG_FILE so that we do not need to use `--config` with every commands:
```
export SML_CONFIG_FILE=./etc/rc.conf
export SML_SK="YOUR_SK for your user"
```

# Project

## Create a Project
This create a new project, use rest API
```
./sym-rest-rc project create --name=project1 --params="enable_histogram=true" --type=cpu
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

## Make Predictions

This make prediction with a model

### Predictions Using REST API with JSON dataframe
```
./sym-rest-rc predict --project=test-rust --model=model2 --df='{"attributeNames":["sepal_length","sepal_width","petal_length","petal_width"],"data":[["4.3","3","1.1","0.1"]],"attributeTypes":["C","C","C","C"]}'
```

### Predictions Using REST API with CSV file
```
./sym-rest-rc predict --project=test-rust --model=model2 --file="./iris2rows.csv"
```

### Predictions Using WebSocket with CSV file
```
./sym-rest-rc model predict --project=test-rust --model=model2 --file="./iris2rows.csv" --use-ws
```
## Auto Select Model TBD
```
NOT IMPLEMENTED

url:
/{cid}/projects/{pid}/autoSelect
body: Map with 2 keys
Map{"dataframe"=DataFrame, "mlcontext"=MLContext}

./sym-rest-rc model autoselect --project=test-rust --model=model5 --file="./iris2rows.csv" --targets="13" --inputs="0,1,2,3" --params="param1=somevalue,param2=somevalue2" --df='{"attributeNames":["sepal_length","sepal_width","petal_length","petal_width"],"data":[["4.3","3","1.1","0.1"]],"attributeTypes":["C","C","C","C"]}'
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



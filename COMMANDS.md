
# This command load the configuration file
# All properties needs to be available via some 'static' method

# export SML_CONFIG_FILE so that we do not need to use --config with every command
export SML_CONFIG_FILE=./etc/rc.conf
export SML_SK="YOUR_SK for your user"

# This create a new project, use rest API
create --name=project1 --params="enable_histogram=true" --type=cpu

# this learn or push data to a project, use web socket
learn --usews=true --file="/datasets/c/IrisFile/Iris_rtlm.csv" --types=C,C,C,C,B,B,B,B,B,B,B,B,B,B,B

# this create a new machine learning model, use rest API (asynchronous - returns job id)
build --project=project1 --name=model1 --type=hba --targets="13" --inputs="0,1,2,3" --params="param1=somevalue,param2=somevalue2"
build --project=project1 --name=model2 --type=hba --target-names="Iris_setosa" --input-names="sepal_length,sepal_width,petal_length,petal_width" --params="param1=somevalue,param2=somevalue2"

# check the status of an asynchronous job (like build)
job --id=8

# this make prediction with a model
# using REST API with JSON dataframe
predict --project=test-rust --model=model2 --df='{"attributeNames":["sepal_length","sepal_width","petal_length","petal_width"],"data":[["4.3","3","1.1","0.1"]],"attributeTypes":["C","C","C","C"]}'

# using REST API with CSV file
predict --project=test-rust --model=model2 --file="./dem-websocat/iris2rows.csv"

# using WebSocket with CSV file
predict --project=test-rust --model=model2 --file="./dem-websocat/iris2rows.csv" --use-ws



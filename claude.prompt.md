================================================
================================================

Let's now implement the 'model info' command. It will be like that

model info --project='project name' --model='model id'

Cannonical URL

GET http://{host}:{port}/symetry/rest/{cid}/projects/{pid}/models/{modelid}

================================================
================================================

Let's now implement the 'project info' command. It will be like that

project info --project='project name'

Cannonical URL

GET http://{host}:{port}/symetry/rest/{cid}/projects/{pid}/info



================================================
================================================
1)
Let's now implement the 'model autoselect' call. It will have the following parameters

--project='the project name'
--model='the model name to create'
--val-file='file to use for validation data'
--val-df='dataframe to use for validation data'
--inputs='comma separated list of attribute id of inputs'
--targets='comma separated list of attributeid of targets'
--input-names='comma separated list of attributes names for input'
--target-names='comma separated list of attributes names for targets'
--params="param1=somevalue,param2=somevalue2"

2) the canonical url:

/symetry/rest/{cid}/projects/{pid}/autoSelect?task=binary_classifier&modelid=autoselect-model1

3) request body: it is a Map with 2 keys "dataframe" and "mlcontext"

Map{"dataframe"=DataFrame, "mlcontext"=MLContext}

3.1) the "dataframe" contains the json dataframe of either the --val-file or --val-df parameters
3.2) the "mlcontext" is explained at point (4).

4) command line parameter mapping for ml context:

4..1) the --targets parameter needs to be mapped to the MLContext.targets array
4.2) the --inputs parameter needs to be mapped to the MLContext.inputAttributes array
4.3) the --input-names parameter needs to be mapped to the MLContext.inputAttributeNames array
4.4) the --target-names parameter needs to be mapped to the MLContext.targetAttributeNames array
4.5) the --params needs to be mapped to the MLContext.extraParameters hashmap


5) example:

./sym-rest-rc model autoselect --task=binary_classifier --project=test-rust --model=model5 --val-file="./iris2rows.csv" --targets="13" --inputs="0,1,2,3" --params="param1=somevalue,param2=somevalue2" --val-df='{"attributeNames":["sepal_length","sepal_width","petal_length","petal_width","sepal_lengt_b1","sepal_lengt_b2","sepal_width_b1","sepal_width_b2","petal_length_b1","petal_length_b2","petal_width_b1","petal_width_b2","Iris_setosa","Iris_versicolor","Iris_virginica"],"data":[["4.3","3","1.1","0.1","1","0","0","1","1","0","1","0","1","0","0"],["4.8","3","1.4","0.1","1","0","0","1","1","0","1","0","1","0","0"]],"attributeTypes":["C","C","C","C","B","B","B","B","B","B","B","B","B","B","B"],"errorHandling":1}' \
--params='autoselect_grid_type=autoselect_grid_type_normal,selector_type=selector_type_fw_bw,selector_max_iterations=5,autoselect_thread_pool_size=4,sml_model_assessment_type=auc'


================================================
================================================


let's implement the build command

1) This command use the REST API.
2) see @dem-curl/dem-curl-examples/ex-0001-iris.sh step 6: for example of request body

# 6. build one LDA model
dem_curl \
"POST" \
"/symetry/rest/c1/projects/iris/build" \
"algo=lda&modelid=lda1&svdreduce=false" \
'{"targets":[12],"inputAttributes":[0,1,2,3],"extraParameters":{"matrix_use_pseudoinv":"false","sml_rcond_use":"false","sml_rcond_tolerance":"0.00000000000001"}}' \
"$CUST" \
"$SECRETKEY" \
"$BASE"


3) build command has 2 form: one that use id for targets and inputs and one that use the attibutes names for inputs and target

build --name=model1 --type=hba --targets="13" --inputs="0,1,2,3" --params="param1=somevalue,param2=somevalue2"
build --name=model2 --type=hba --targetNames="Iris_setosa" --input-namess="sepal_length,sepal_width,petal_length,petal_width" --params="param1=somevalue,param2=somevalue2"

4) The body of the request is a MLContext object, in java it is defined like this:

public class MLContext {
	ArrayList<String> targets = new ArrayList<String>();
	ArrayList<String> inputAttributes = new ArrayList<String>();
	ArrayList<String> inputAttributeNames = new ArrayList<String>();
	ArrayList<String> targetAttributeNames = new ArrayList<String>();
	HashMap<String, String> extraParameters = new HashMap<String, String>();
}

5) command line parameter mapping:
5.1) the --targets parameter needs to be mapped to the MLContext.targets array
5.2) the --inputs parameter needs to be mapped to the MLContext.inputAttributes array 
5.3) the --input-names parameter needs to be mapped to the MLContext.inputAttributeNames array
5.4) the --target-names parameter needs to be mapped to the MLContext.targetAttributeNames array
5.5) the --params needs to be mapped to the MLContext.extraParameters hashmap



================================================
================================================

Okay, let's implement the learn with websocket command. So this is the example of the URL as well as the request body. Obviously make sure to use the authentication scheme as used with create project. Please create a learn_rest.rs file in commands folder.


learn --file ./dem-websocat/iris2rows.csv --types "C,C,C,C,B,B,B,B,B,B,B,B,B,B,B" --project=test-rust --use_ws

URL:

ws://{host}:{config.port}/symetry/ws/learn

body:

{length of header},{headers}{body}

Please look at the method wslearn() in the @dem_websocat/dem_websocat.sh to understand what {headers} and body are.

This is an example of a message: customer id is part of the headers, and the project name is part of "extraKeys":

147,{"headers":["2025-11-16 20:11:50;294669000","QIrbsrJcSCNAYVFjcjkHMw==","5+38zLMsXJlRj56cwn4S07vmdPkSArnYRhm/MldCWng=","c1"],"extraKeys":["rtest1"]}{"attributeNames":["sepal_length","sepal_width","petal_length","petal_width","sepal_lengt_b1","sepal_lengt_b2","sepal_width_b1","sepal_width_b2","petal_length_b1","petal_length_b2","petal_width_b1","petal_width_b2","Iris_setosa","Iris_versicolor","Iris_virginica"],"data":[["4.3","3","1.1","0.1","1","0","0","1","1","0","1","0","1","0","0"],["4.8","3","1.4","0.1","1","0","0","1","1","0","1","0","1","0","0"]],"attributeTypes":["C","C","C","C","B","B","B","B","B","B","B","B","B","B","B"],"errorHandling":1}


================================================
================================================
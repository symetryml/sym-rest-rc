
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
const fs = require('fs');

//Interesting question on the fastest way to concat to arrays
//https://stackoverflow.com/a/44087401/2588566

//First part

var readFast = function(callback){

    var nodes = []
    fs.readFile("input.txt", "utf8", function(err, data){
	lines = data.split("\n");
	for(var i =0; i < lines.length; i++){
	    line = lines[i].replace("(", "").replace(")", "").replace(/,/g, "").split(" ");
	    if (line.filter(x => x == '->')){
		nodes.push( ...line.slice(2) ); // Unpack and push
	    }
	    nodes.push(line[0]);
	}
	callback(nodes);
    });
    
}


var root = function(nodes){
    for(let i in nodes){
	if(nodes.filter(x => x === nodes[i]).length == 1 && nodes[i].length){
	    console.log("Solution to part 1: ", nodes[i]);
	    return nodes[i];
	}
    }
}

readFast(root);

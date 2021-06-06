## sample with Graphviz

```shell
dot -Txdot_json -oimport.json ./resources/go-package.gv.txt
cat import.json | ./target/debug/dot-json2cypher-rs
```
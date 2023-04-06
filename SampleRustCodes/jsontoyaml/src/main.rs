use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs::File;
use std::io::Write;

fn main() {
    // assume `json_value` contains the JSON string as a serde_json::Value
    let json_str = r#"{"config": {"kube_config": {"apiVersion": "v1", "clusters": [{"cluster": {"certificate-authority-data": "=", "server": "ip"}, "name": "kubernetes"}], "contexts": [{"context": {"cluster": "kubernetes", "user": "robin-admin"}, "name": "robin-admin"}, {"context": {"cluster": "kubernetes", "user": "robin-user"}, "name": "robin-user"}], "current-context": "robin-admin", "kind": "Config", "preferences": {}, "users": [{"name": "robin-admin", "user": {"token": "-----"}}, {"name": "robin-user", "user": {"exec": {"apiVersion": "client.authentication.k8s.io/v1beta1", "args": null, "command": "robin-auth", "env": null, "provideClusterInfo": false}}}]}, "type": "KUBE_CONFIG"}, "name": "telx131", "elem_ref": null, "description": ""}"#;
    let yaml_value: YamlValue = serde_json::from_str(json_str).unwrap();

    // convert to a YAML value
    //let yaml_value: YamlValue = serde_yaml::from_value(json_value).unwrap();

    // open the file and write the YAML value to it
    let mut file = File::create("output.yaml").unwrap();
    file.write_all(serde_yaml::to_string(&yaml_value).unwrap().as_bytes()).unwrap();
}
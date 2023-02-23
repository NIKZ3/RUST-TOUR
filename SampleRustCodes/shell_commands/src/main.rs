use std::process::Command;

fn main() {
    // Run a shell command and capture the output

    // command parsing
    let kube_cmd = "kubectl     get   pods    ".split(' ');
    let mut kube_cmd_args: Vec<String> = Vec::new();
    for arg in kube_cmd {
        if arg != "" && arg != "kubectl" {
            kube_cmd_args.push(arg.to_string());
        }
        }
    
    //setup kubeconfig
    // open file kubeconfigs/<elem_uid>/config -> put the request json in it
    let kubeconfig = format!("/root/test/config");
    kube_cmd_args.push("--kubeconfig".to_string());
    kube_cmd_args.push(kubeconfig.to_string());
    println!("{:#?}", kube_cmd_args);

    let output = Command::new("kubectl")
                     .args(&kube_cmd_args)
                     .output()
                     .expect("failed to execute command");

    // Store the output in a string
    let output_string = String::from_utf8_lossy(&output.stdout).to_string();

    // Print the output
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
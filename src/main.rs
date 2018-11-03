#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate i3ipc;

use std::collections::BTreeMap;
use std::{fs, env};
use i3ipc::{I3Connection, I3EventListener, Subscription};
use i3ipc::reply::NodeType;
use i3ipc::event::{Event, WindowEventInfo, inner::WindowChange};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(rename = "default-gamma")]
    default_gamma: BTreeMap<String, f32>,
    #[serde(rename = "window")]
    windows: Vec<WindowConfig>
}

#[derive(Debug, Deserialize)]
struct WindowConfig {
    title: String,
    gamma: BTreeMap<String, f32>
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let config_filename = args.get(1).expect("configuration file name not provided");
    let config_file = fs::read_to_string(config_filename).expect("cannot open configuration");
    let config: Config = toml::from_str(&config_file).expect("cannot parse configuration");

    let mut connection = I3Connection::connect().expect("cannot connect to i3");

    let mut listener = I3EventListener::connect().expect("cannot listen for i3 events");
    listener.subscribe(&[Subscription::Window]).expect("cannot subscribe to i3 events");

    for event in listener.listen() {
        match event.expect("cannot receive i3 events") {
            Event::WindowEvent(WindowEventInfo { change: WindowChange::Focus, .. }) => {
                let mut tree = &connection.get_tree().expect("cannot request i3 node tree");
                let mut output = None;
                while !tree.focused {
                    if tree.nodetype == NodeType::Output {
                        output = tree.name.as_ref();
                    }

                    for node in tree.nodes.iter().chain(tree.floating_nodes.iter()) {
                        if node.id == tree.focus[0] {
                            tree = &node;
                            break
                        }
                    }
                }
                if !tree.focused { continue }
                let output = output.expect("window does not belong to any output");
                println!("output {output} switched to window {title:?}",
                         output=output, title=tree.name);

                let mut found = false;
                for window in config.windows.iter() {
                    if tree.name.as_ref().map(|x| x.as_str()) == Some(window.title.as_ref()) {
                        if window.gamma.contains_key(output) {
                            let gamma = window.gamma[output];

                            println!("output {output} set gamma to {gamma} for {title}",
                                output=output, gamma=gamma, title=window.title);
                            connection
                                .run_command(&format!(
                                    "exec xrandr --output {output} \
                                                 --gamma {gamma}:{gamma}:{gamma}",
                                    output=output, gamma=gamma))
                                .expect("cannot run xrandr");
                            found = true;
                            break
                        }
                    }
                }

                if !found && config.default_gamma.contains_key(output) {
                    let gamma = config.default_gamma[output];

                    println!("output {output} set gamma to {gamma} by default",
                        output=output, gamma=gamma);
                    connection
                        .run_command(&format!(
                            "exec xrandr --output {output} --gamma {gamma}:{gamma}:{gamma}",
                            output=output, gamma=gamma))
                        .expect("cannot run xrandr");
                }
            }

            _ => ()
        }
    }
}

pub struct PrinterColumns {
    pub columns: Vec<PrinterColumn>,
}

pub struct PrinterColumn {
    pub name: String,
    pub data_type: String,
    pub json_path: String,
    pub description: String,
    pub format: String,
    pub priority: u8,
}

impl PrinterColumns {
    pub fn new() -> Self {
        PrinterColumns { columns: vec![] }
    }
    pub fn pods() -> PrinterColumns {
        PrinterColumns {
            columns: vec![
                PrinterColumn {
                    name: String::from("Namespace"),
                    data_type: String::from("String"),
                    json_path: String::from("$.metadata.namespace"),
                    description: String::from("Namespace of the Pod"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Name"),
                    data_type: String::from("String"),
                    json_path: String::from("$.metadata.name"),
                    description: String::from("Name of the Pod"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Ready"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.containerStatuses[*].ready"),
                    description: String::from("Readiness of the Pod's containers"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Status"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.phase"),
                    description: String::from("Status of the Pod"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Restarts"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.containerStatuses[*].restartCount"),
                    description: String::from("Number of restarts of the Pod's containers"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Age"),
                    data_type: String::from("String"),
                    json_path: String::from("$.metadata.creationTimestamp"),
                    description: String::from(
                        "Human readable duration since start time of the Pod",
                    ),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("IP"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.podIps[*]"),
                    description: String::from("IP addresses of the Pod's containers"),
                    format: String::from("String"),
                    priority: 1,
                },
                PrinterColumn {
                    name: String::from("Node"),
                    data_type: String::from("String"),
                    json_path: String::from("$.spec.nodeName"),
                    description: String::from("Node name where the Pod is running"),
                    format: String::from("String"),
                    priority: 1,
                },
                PrinterColumn {
                    name: String::from("Nominated Node"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.nominatedNodeName"),
                    description: String::from(
                        "Name of Node that is nominated for the Pod to run on",
                    ),
                    format: String::from("String"),
                    priority: 1,
                },
                PrinterColumn {
                    name: String::from("Readiness Gates"),
                    data_type: String::from("String"),
                    json_path: String::from("$spec.readinessGates[*].conditionType"),
                    description: String::from("Condition Gates for the Pod"),
                    format: String::from("String"),
                    priority: 1,
                },
            ],
        }
    }

    pub fn deploys() -> PrinterColumns {
        PrinterColumns {
            columns: vec![
                PrinterColumn {
                    name: String::from("Namespace"),
                    data_type: String::from("String"),
                    json_path: String::from("$.metadata.namespace"),
                    description: String::from("Namespace of the Deployment"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Name"),
                    data_type: String::from("String"),
                    json_path: String::from("$.metadata.name"),
                    description: String::from("Name of the Deployment"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Ready"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.availableReplicas"),
                    description: String::from("Readiness of the Deployment's Pods"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Up-To-Date"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.updatedReplicas"),
                    description: String::from("Number of up-to-date pods in the Deployment"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Available"),
                    data_type: String::from("String"),
                    json_path: String::from("$.status.availableReplicas"),
                    description: String::from("Number of available Pod's in the Deployment"),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Age"),
                    data_type: String::from("String"),
                    json_path: String::from("$.metadata.creationTimestamp"),
                    description: String::from(
                        "Human readable duration since creation time of the Deployment",
                    ),
                    format: String::from("String"),
                    priority: 0,
                },
                PrinterColumn {
                    name: String::from("Containers"),
                    data_type: String::from("String"),
                    json_path: String::from("$.spec.template.spec.containers[*].name"),
                    description: String::from("Name of containers that the Pods create"),
                    format: String::from("String"),
                    priority: 1,
                },
                PrinterColumn {
                    name: String::from("Images"),
                    data_type: String::from("String"),
                    json_path: String::from("$.spec.template.spec.containers[*].image"),
                    description: String::from("Images that the containers in the Deployment use"),
                    format: String::from("String"),
                    priority: 1,
                },
                PrinterColumn {
                    name: String::from("Selector"),
                    data_type: String::from("String"),
                    json_path: String::from("$.spec.selector.matchLabels[*]"),
                    description: String::from(
                        "Selectors that the Deployment uses to determine which Pods belong to the Deployment",
                    ),
                    format: String::from("String"),
                    priority: 1,
                },
            ],
        }
    }
}

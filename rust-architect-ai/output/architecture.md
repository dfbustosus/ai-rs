sequenceDiagram
    participant U as User
    participant A as Args
    participant M as Main
    participant F as Run Function
    participant L as logger::init
    participant CPR as config::get_api_key
    participant SC as project_scanner::scan_project
    participant DG as diagram_generator::DiagramGenerator
    participant OC as openai_client::OpenAIClient
    participant GD as diagram_generator::generate_diagram
    
    U->>M: Run application
    M->>L: logger::init()
    M->>A: Parse command-line Args
    A-->>M: Args
    M->>F: run().await
    F->>A: Validate Args
    A-->>F: Validated Args
    F->>CPR: config::get_api_key()
    CPR-->>F: API Key
    F->>OC: OpenAIClient::new(API Key)
    OC-->>F: Client Instance
    F->>DG: DiagramGenerator::new(Client)
    DG-->>F: DiagramGenerator Instance
    F->>SC: scan_project(&args.project_path)
    SC-->>F: Project Context String
    F->>GD: generate_diagram(Project Context, Diagram Type, Function Name)
    GD-->>F: Diagram String
    F->>M: Save Diagram to File
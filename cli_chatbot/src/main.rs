use groq::*;

fn main() {
    // Create a system message
    let system_message = Message::new("You are a helpful AI Assistant".to_string(), Role::System);

    // Initialize the model
    let mut model = GroqModel::build(GroqModelType::Llama3_8b8192).unwrap();

    // change chat configs for the model
    model.chat_options.messages.push(system_message);
    model.chat_options.temperature = Some(0.5);

    

    loop {
        println!("[USER]: ");
        
        let mut user_content = String::new();
        std::io::stdin().read_line(&mut user_content).unwrap();
        if user_content.trim() == "[STOP]".to_string() { break; }

        println!("--------------------------------------------------");

        let response = model.invoke(
            Message {
                content: user_content,
                role: Role::User
            }
        ).unwrap();

        println!("[ASSISTANT]: {}", response.content);
        println!("--------------------------------------------------")
    }
    

}

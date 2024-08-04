// main.rs
//use rust_sled_crud_lib::db::sled_db::SledDb;
//use rust_sled_crud_lib::models::conversation::Conversation;
//use rust_sled_crud_lib::db::dd_object::DbObject;
use rust_sled_crud_lib::models::parser::parse_conversation_data;
use rust_sled_crud_lib::controller::conversation::ConversationController;
//use std::error::Error;
//use uuid::Uuid;
//use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_string = r#"
    {
  "id": "ee6a03b5-d17f-42e4-a5b5-65a099c250dc",
  "title": "Escape File Path Rust",
  "create_time": 1720335757.046148,
  "update_time": 1720335895.590312,
  "mapping": {
    "99bbb25d-d020-4bf8-87a3-01c14f3575a5": {
      "id": "99bbb25d-d020-4bf8-87a3-01c14f3575a5",
      "message": {
        "id": "99bbb25d-d020-4bf8-87a3-01c14f3575a5",
        "author": {
          "role": "system",
          "name": null,
          "metadata": {}
        },
        "create_time": null,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            ""
          ]
        },
        "status": "finished_successfully",
        "end_turn": true,
        "weight": 0,
        "metadata": {
          "is_visually_hidden_from_conversation": true
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "aaa18177-4607-4e8f-ad33-c85e66cdfad7",
      "children": [
        "aaa2d189-e728-41f5-b670-4faccaa113d9"
      ]
    },
    "aaa18177-4607-4e8f-ad33-c85e66cdfad7": {
      "id": "aaa18177-4607-4e8f-ad33-c85e66cdfad7",
      "message": null,
      "parent": null,
      "children": [
        "99bbb25d-d020-4bf8-87a3-01c14f3575a5"
      ]
    },
    "aaa2d189-e728-41f5-b670-4faccaa113d9": {
      "id": "aaa2d189-e728-41f5-b670-4faccaa113d9",
      "message": {
        "id": "aaa2d189-e728-41f5-b670-4faccaa113d9",
        "author": {
          "role": "user",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335757.053241,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            "Escape correctly the following file so that Rust string understands it:\n\n```\n    let test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n```\n"
          ]
        },
        "status": "finished_successfully",
        "end_turn": null,
        "weight": 1,
        "metadata": {
          "request_id": "89f5f84c081181fd-IAD",
          "message_source": null,
          "timestamp_": "absolute",
          "message_type": null
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "99bbb25d-d020-4bf8-87a3-01c14f3575a5",
      "children": [
        "f79c556c-8fe1-4ba8-a660-fc06e2e91c5e"
      ]
    },
    "f79c556c-8fe1-4ba8-a660-fc06e2e91c5e": {
      "id": "f79c556c-8fe1-4ba8-a660-fc06e2e91c5e",
      "message": {
        "id": "f79c556c-8fe1-4ba8-a660-fc06e2e91c5e",
        "author": {
          "role": "assistant",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335764.159342,
        "update_time": null,
        "content": {
          "content_type": "model_editable_context",
          "model_set_context": "1. [2024-05-12]. The user has participated in many workshops provided by the Miner's Cyber Security Club at UTEP, involving the club president and guests from the army.\n\n2. [2024-05-14]. The user is interested in pursuing a PhD and has a bachelor's degree in Computer Science.\n\n3. [2024-05-14]. They are considering studying in Switzerland for their PhD.\n\n4. [2024-05-30]. The user's GitHub username is 'sergio-com-ai'.\n\n5. [2024-05-31]. The user is working with Node.js, Express, and Supertest for testing APIs.\n\n6. [2024-06-02]. The user is interested in growing their LinkedIn brand and penetrating the Hispanic and US markets.\n\n7. [2024-06-06]. The user is preparing for the AWS Solutions Architect Professional exam, focusing on case scenarios and deep architectures.\n\n8. [2024-06-07]. The user is working with Templater for Obsidian.\n\n9. [2024-06-07]. The user is using ALSA with PipeWire for audio on their system.\n\n10. [2024-06-12]. The user is enhancing dataview tables in Obsidian for job applications and interviews, adding specific statuses with corresponding emojis. They have a new status called 'Ghosted' and removed 'In Progress' as a status.\n\n11. [2024-06-15]. User is applying for a Master's in Artificial Intelligence at UTEP and needs a recommendation letter from Professor Ward.\n\n12. [2024-06-15]. User has spent the last five years in the DC area working with startups and large companies.\n\n13. [2024-06-15]. User is considering traveling to San Francisco for networking due to being without a job for about a year after not getting clearances in DC.\n\n14. [2024-06-15]. User is mid-senior to almost director level, has a BS, and a struggling startup on the cloud.\n\n15. [2024-06-15]. User is in a complex relationship where marriage was planned for mutual benefit, but the partner is in love with another man and seems to be withdrawing.\n\n16. [2024-06-19]. The user prefers their diary entries to reflect their unique personality and identity as a 'mexa, mexa, mexicano' who sometimes sings and expresses themselves with more flair.\n\n17. [2024-06-19]. The user is from Chihuahua, talks with a strong northern Mexican accent, often eating letters and yelling a lot, especially due to the heat. They also emphasize that they are actually crossing into the United States, as they were born there, despite identifying strongly as Mexican.\n\n18. [2024-06-26]. User wants a more advanced workout plan and because it's Monday, the focus will be on the chest as well. They are optimizing their physique for a beach body and seek an advanced 5-day training plan.\n\n19. [2024-07-04]. User previously interviewed Professor Ward about AI and has a strong interest in learning about AI in-depth. They have worked on AI since undergraduate, taking AI courses, and interviewing Dr. Ward for another class about AI. They have been doing AI-oriented work and have a passion for the field.\n\n20. [2024-07-06]. User likes the physique of Arnold Schwarzenegger and prefers to train legs often, especially with barbell compound exercises at the beginning of the routine to kick off the workout.",
          "repository": null,
          "repo_summary": null
        },
        "status": "finished_successfully",
        "end_turn": null,
        "weight": 1,
        "metadata": {
          "message_type": null,
          "model_slug": "gpt-4o",
          "default_model_slug": "gpt-4o",
          "parent_id": "aaa2d189-e728-41f5-b670-4faccaa113d9",
          "request_id": "89f5f84c081181fd-IAD",
          "timestamp_": "absolute",
          "model_switcher_deny": []
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "aaa2d189-e728-41f5-b670-4faccaa113d9",
      "children": [
        "fc642c34-554c-48ed-b72d-104f122aa965"
      ]
    },
    "fc642c34-554c-48ed-b72d-104f122aa965": {
      "id": "fc642c34-554c-48ed-b72d-104f122aa965",
      "message": {
        "id": "fc642c34-554c-48ed-b72d-104f122aa965",
        "author": {
          "role": "assistant",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335764.159451,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            "To properly escape the file path so that Rust understands it, you need to escape the backslashes. In Rust, backslashes are escape characters, so each backslash in the path needs to be doubled. Here's the correctly escaped file path:\n\n```rust\nlet test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n```\n\nIn this case, since the original path does not contain any backslashes, it remains the same when used as a string in Rust. If you had backslashes in the path (e.g., a Windows path), you would need to escape them like this:\n\n```rust\nlet test_path = \"C:\\\\Users\\\\adminlenovo\\\\Data\\\\Programming\\\\rust-srt-extractor\\\\1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n```\n\nFor your specific example on a Unix-based system, no changes are needed beyond ensuring the path is within double quotes."
          ]
        },
        "status": "finished_successfully",
        "end_turn": true,
        "weight": 1,
        "metadata": {
          "citations": [],
          "gizmo_id": null,
          "message_type": null,
          "model_slug": "gpt-4o",
          "default_model_slug": "gpt-4o",
          "pad": "A",
          "parent_id": "f79c556c-8fe1-4ba8-a660-fc06e2e91c5e",
          "finish_details": {
            "type": "stop",
            "stop_tokens": [
              200002
            ]
          },
          "is_complete": true,
          "request_id": "89f5f84c081181fd-IAD",
          "timestamp_": "absolute",
          "model_switcher_deny": []
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "f79c556c-8fe1-4ba8-a660-fc06e2e91c5e",
      "children": [
        "aaa220ac-5483-47cb-8a88-4006e2d28e0e"
      ]
    },
    "aaa220ac-5483-47cb-8a88-4006e2d28e0e": {
      "id": "aaa220ac-5483-47cb-8a88-4006e2d28e0e",
      "message": {
        "id": "aaa220ac-5483-47cb-8a88-4006e2d28e0e",
        "author": {
          "role": "user",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335788.835813,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            "It's having issues with the spaces (using linux btw)\n"
          ]
        },
        "status": "finished_successfully",
        "end_turn": null,
        "weight": 1,
        "metadata": {
          "request_id": "89f5f912c89d81fd-IAD",
          "message_source": null,
          "timestamp_": "absolute",
          "message_type": null
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "fc642c34-554c-48ed-b72d-104f122aa965",
      "children": [
        "60fb9d97-4cea-4996-a02d-3f9c1c51c142"
      ]
    },
    "60fb9d97-4cea-4996-a02d-3f9c1c51c142": {
      "id": "60fb9d97-4cea-4996-a02d-3f9c1c51c142",
      "message": {
        "id": "60fb9d97-4cea-4996-a02d-3f9c1c51c142",
        "author": {
          "role": "assistant",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335801.285524,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            "In Rust, spaces within strings are handled without any special escaping. If you are encountering issues, it may not be due to the spaces themselves but potentially how the path is being used or passed around in your code. Here's the correct way to handle a path with spaces in Rust:\n\n```rust\nlet test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n```\n\nHowever, if the issue persists, make sure the path is used correctly in context. For example, if you're using it in a `Command` or file operation, ensure it's properly referenced:\n\n### Example with File Operations\n```rust\nuse std::fs::File;\nuse std::io::prelude::*;\n\nfn main() {\n    let test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n    let mut file = File::open(test_path).expect(\"Unable to open file\");\n    let mut contents = String::new();\n    file.read_to_string(&mut contents).expect(\"Unable to read file\");\n    println!(\"{}\", contents);\n}\n```\n\n### Example with Command Execution\n```rust\nuse std::process::Command;\n\nfn main() {\n    let test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n    let output = Command::new(\"cat\")\n        .arg(test_path)\n        .output()\n        .expect(\"Failed to execute command\");\n\n    println!(\"{}\", String::from_utf8_lossy(&output.stdout));\n}\n```\n\nIf you still encounter issues, please provide more context or the specific error message you're seeing."
          ]
        },
        "status": "finished_successfully",
        "end_turn": true,
        "weight": 1,
        "metadata": {
          "citations": [],
          "gizmo_id": null,
          "finish_details": {
            "type": "stop",
            "stop_tokens": [
              200002
            ]
          },
          "is_complete": true,
          "message_type": null,
          "model_slug": "gpt-4o",
          "default_model_slug": "gpt-4o",
          "pad": "AAAAAAAAAAAA",
          "parent_id": "aaa220ac-5483-47cb-8a88-4006e2d28e0e",
          "request_id": "89f5f912c89d81fd-IAD",
          "timestamp_": "absolute",
          "model_switcher_deny": []
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "aaa220ac-5483-47cb-8a88-4006e2d28e0e",
      "children": [
        "aaa2754b-90be-4185-8a07-1850d1b1ccbf"
      ]
    },
    "aaa2754b-90be-4185-8a07-1850d1b1ccbf": {
      "id": "aaa2754b-90be-4185-8a07-1850d1b1ccbf",
      "message": {
        "id": "aaa2754b-90be-4185-8a07-1850d1b1ccbf",
        "author": {
          "role": "user",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335879.382455,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            "File not found, don't we have to scape spaces in the filename?\n"
          ]
        },
        "status": "finished_successfully",
        "end_turn": null,
        "weight": 1,
        "metadata": {
          "request_id": "89f5fb4a6b9181fd-IAD",
          "message_source": null,
          "timestamp_": "absolute",
          "message_type": null
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "60fb9d97-4cea-4996-a02d-3f9c1c51c142",
      "children": [
        "ff00a280-d6b6-45df-8390-856877f05559"
      ]
    },
    "ff00a280-d6b6-45df-8390-856877f05559": {
      "id": "ff00a280-d6b6-45df-8390-856877f05559",
      "message": {
        "id": "ff00a280-d6b6-45df-8390-856877f05559",
        "author": {
          "role": "assistant",
          "name": null,
          "metadata": {}
        },
        "create_time": 1720335895.383703,
        "update_time": null,
        "content": {
          "content_type": "text",
          "parts": [
            "In Rust, spaces in file paths do not need special escaping within string literals. If you're encountering a \"File not found\" error, it could be due to the path being incorrect or not existing. However, spaces themselves are not the issue. Here's a simple check to see if the path exists:\n\n```rust\nuse std::path::Path;\n\nfn main() {\n    let test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n\n    if Path::new(test_path).exists() {\n        println!(\"The file exists!\");\n    } else {\n        println!(\"The file does not exist.\");\n    }\n}\n```\n\nIf the file does exist but Rust can't find it, there might be a permissions issue or some subtle discrepancy in the path. Make sure the path is correct by copying it directly from the terminal using `pwd` or `ls` commands.\n\n### Checking the File Path in the Terminal\nRun the following in your terminal to verify the path:\n\n```sh\nls \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\"\n```\n\nIf `ls` can't find the file, then Rust won't be able to either. If it can, then double-check for any hidden characters or typos in your Rust code.\n\n### Example with Debugging\nAdd some debugging output to ensure the path is correct:\n\n```rust\nuse std::fs::File;\nuse std::io::prelude::*;\n\nfn main() {\n    let test_path = \"/home/adminlenovo/Data/Programming/rust-srt-extractor/1 - CD13317 GenAI C2 L1 A01 Meet Your Instructor V1 - lang_en-us.srt\";\n\n    println!(\"Trying to open: {}\", test_path);\n\n    match File::open(test_path) {\n        Ok(mut file) => {\n            let mut contents = String::new();\n            file.read_to_string(&mut contents).expect(\"Unable to read file\");\n            println!(\"{}\", contents);\n        }\n        Err(e) => {\n            println!(\"Error opening file: {}\", e);\n        }\n    }\n}\n```\n\nIf the path is correct and accessible, this should work without additional escaping for spaces. If you continue to face issues, it might be helpful to check the file permissions and ensure the running process has access to the specified directory and file."
          ]
        },
        "status": "finished_successfully",
        "end_turn": true,
        "weight": 1,
        "metadata": {
          "citations": [],
          "gizmo_id": null,
          "finish_details": {
            "type": "stop",
            "stop_tokens": [
              200002
            ]
          },
          "is_complete": true,
          "message_type": null,
          "model_slug": "gpt-4o",
          "default_model_slug": "gpt-4o",
          "pad": "AAAAAAAAAAAAAAA",
          "parent_id": "aaa2754b-90be-4185-8a07-1850d1b1ccbf",
          "request_id": "89f5fb4a6b9181fd-IAD",
          "timestamp_": "absolute",
          "model_switcher_deny": []
        },
        "recipient": "all",
        "channel": null
      },
      "parent": "aaa2754b-90be-4185-8a07-1850d1b1ccbf",
      "children": []
    }
  },
  "current_node": "ff00a280-d6b6-45df-8390-856877f05559",
  "conversation_template_id": null,
  "gizmo_id": null,
  "is_archived": false,
  "workspace_id": null,
  "async_status": null,
  "safe_urls": [],
  "moderation_results": [],
  "plugin_ids": null,
  "conversation_id": "ee6a03b5-d17f-42e4-a5b5-65a099c250dc",
  "default_model_slug": "gpt-4o",
  "conversation_origin": null,
  "voice": null
}
    "#;

    // Parse the JSON string into a ConversationData object
    let conversation_data = parse_conversation_data(json_string)?;
    //println!("Parsed ConversationData: {:?}", conversation_data);

    let simplified_conversation = ConversationController::collect_content(&conversation_data);
    println!("{}", simplified_conversation);

    Ok(())
}

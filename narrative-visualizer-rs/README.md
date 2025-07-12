# The Narrative Visualization Engine

> A command-line tool built in Rust that demonstrates a sophisticated multi-modal AI pipeline.

## 🎯 Overview

The Narrative Visualization Engine takes a written narrative—a story, a scene, a description—and automatically generates a visual storyboard, transforming text into a sequence of AI-synthesized images.

The engine's core innovation is its ability to bridge the gap between language and vision. It performs a multi-stage analysis to first understand the narrative structure and then creatively interpret it, generating unique, evocative images that correspond to the text.

## ✨ Key Features

- **Multi-Modal AI Pipeline**: Orchestrates a complex workflow between different AI models: first, a text analysis model to decompose the narrative into scenes, then another to generate artistic prompts, and finally an image generation model to create the visuals.

- **Automated Storyboarding**: Solves the difficult and time-consuming task of visualizing a narrative by automatically generating a storyboard from a single block of text.

- **Sophisticated AI Reasoning**: The engine doesn't just process text; it performs creative interpretation, translating narrative concepts like mood, setting, and action into detailed visual descriptions.

- **Robust & Modular Architecture**: Engineered with a clean separation of concerns. The multi-stage pipeline is broken into distinct, maintainable modules for scene detection, prompt generation, and image synthesis.

- **Self-Contained HTML Output**: Assembles the final result into a single, portable `storyboard.html` file, embedding the generated images directly so it can be easily shared and viewed in any web browser.

## 📁 Project Structure

The codebase is organized to clearly represent the flow of data through the visualization pipeline:

```
narrative-visualizer-rs/
├── .env
├── .gitignore
├── Cargo.toml
├── output/
│   └── (Generated storyboards will be saved here)
├── input/
│   └── sample_story.txt    # An example narrative for processing
└── src/
    ├── main.rs            # Entry point, CLI parsing, and pipeline orchestration
    ├── error.rs           # Unified error handling module
    ├── config.rs          # Manages application configuration
    ├── pipeline/
    │   ├── mod.rs         # The pipeline module definition
    │   ├── stage_1_scene_detection.rs
    │   ├── stage_2_prompt_generation.rs
    │   └── stage_3_image_generation.rs
    ├── output_assembler.rs # Assembles the final storyboard file
    └── openai_client.rs   # A client supporting both text and image generation
```

## 🚀 Setup and Usage

Follow these steps to set up and run the Narrative Visualization Engine:

### 1. Create the Input Directory

In the root of the project, create a directory named `input`:

```bash
mkdir input
```

### 2. Provide an Input Narrative

Inside the `input` directory, create a file named `sample_story.txt` with the following content:

```
The old detective stood on the rain-slicked cobblestone street, the neon sign of "The Blue Dahlia" bar casting a lurid, flickering glow across his tired face. His trench coat was soaked, clinging to his shoulders like a second skin. He stared up at the single lit window on the third floor of the tenement building across the way, a lone silhouette moving behind the tattered curtains. A plume of smoke escaped his lips, mingling with the cold night air as a vintage car rumbled past, its headlights cutting through the dense fog.
```

### 3. Set Up Your API Key

Create a file named `.env` in the root of the project and add your OpenAI API key:

```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
```

### 4. Run the Application

Use `cargo run` to execute the program. You must provide the path to the input file. The `--` separator is crucial to distinguish arguments for Cargo from arguments for your application.

**Example Command:**

```bash
cargo run -- --input-file input/sample_story.txt
```

The tool will then execute the entire pipeline and save the final storyboard to `output/storyboard.html`.

## 📊 Results

### Example Run

Here is an example of the log output you can expect when running the engine with the provided sample story.

**Command:**
```bash
cargo run -- --input-file input/sample_story.txt
```

**Expected Log Output:**
```bash
   Compiling aho-corasick v1.1.3
   Compiling regex-syntax v0.8.5
   Compiling regex-automata v0.4.9
   Compiling regex v1.11.1
   Compiling tracing-subscriber v0.3.19
   Compiling narrative-visualizer-rs v0.1.0 (/Users/davidusta/Desktop/ai-rs/narrative-visualizer-rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.42s
     Running `target/debug/narrative-visualizer-rs --input-file input/sample_story.txt`
 INFO Starting narrative visualization for 'input/sample_story.txt'.
 INFO detect_scenes: Starting scene detection.
 INFO detect_scenes:get_completion: Requesting text completion from OpenAI API.
 INFO detect_scenes:get_completion: Successfully received text completion.
 INFO detect_scenes: Received raw response from API. raw_response=
{
  "scenes": [
    {
      "description": "The old detective stands on a rain-slicked cobblestone street under the neon glow of 'The Blue Dahlia' bar.",
      "originalText": "The old detective stood on the rain-slicked cobblestone street, the neon sign of \"The Blue Dahlia\" bar casting a lurid, flickering glow across his tired face."
    },
    {
      "description": "His soaked trench coat clings to his shoulders as he gazes at a lit window in the tenement building.",
      "originalText": "His trench coat was soaked, clinging to his shoulders like a second skin. He stared up at the single lit window on the third floor of the tenement building across the way, a lone silhouette moving behind the tattered curtains."
    },
    {
      "description": "A plume of smoke escapes his lips and mingles with the cold night air as a vintage car passes by.",
      "originalText": "A plume of smoke escaped his lips, mingling with the cold night air as a vintage car rumbled past, its headlights cutting through the dense fog."
    }
  ]
}

 INFO detect_scenes: Successfully detected 3 scenes.
 INFO generate_visual_prompts: Starting visual prompt generation for 3 scenes.
 INFO generate_visual_prompts: Generating prompt for scene 1/3...
 INFO generate_visual_prompts:get_completion: Requesting text completion from OpenAI API.
 INFO generate_visual_prompts:get_completion: Successfully received text completion.
 INFO generate_visual_prompts: Generating prompt for scene 2/3...
 INFO generate_visual_prompts:get_completion: Requesting text completion from OpenAI API.
 INFO generate_visual_prompts:get_completion: Successfully received text completion.
 INFO generate_visual_prompts: Generating prompt for scene 3/3...
 INFO generate_visual_prompts:get_completion: Requesting text completion from OpenAI API.
 INFO generate_visual_prompts:get_completion: Successfully received text completion.
 INFO generate_visual_prompts: Successfully generated 3 visual prompts.
 INFO generate_images: Starting image generation for 3 prompts.
 INFO generate_images: Generating image for scene 1/3...
 INFO generate_images:generate_image: Requesting image generation from OpenAI API.
 INFO generate_images:generate_image: Successfully received image data.
 INFO generate_images: Generating image for scene 2/3...
 INFO generate_images:generate_image: Requesting image generation from OpenAI API.
 INFO generate_images:generate_image: Successfully received image data.
 INFO generate_images: Generating image for scene 3/3...
 INFO generate_images:generate_image: Requesting image generation from OpenAI API.
 INFO generate_images:generate_image: Successfully received image data.
 INFO generate_images: Successfully generated 3 images.
 INFO Assembling final storyboard HTML at 'output/storyboard.html'...
 INFO Successfully assembled and saved storyboard.
 INFO Successfully generated storyboard at 'output/storyboard.html'
```

### Final Output

After the process completes, a file named `storyboard.html` will be created in the `output` directory. When you open this file in a web browser, you will see a beautifully formatted page displaying each segment of the original story alongside a unique, AI-generated image that visually represents that scene, styled in a cinematic film noir aesthetic as requested by our prompts.

## 🛠️ Prerequisites

- Rust (latest stable version)
- OpenAI API key
- Internet connection for API calls

## 📝 License

This project is licensed under the MIT License.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

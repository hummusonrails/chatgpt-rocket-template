# ChatGPT Template

This is a pre-built Rocket web application template for use with the `create-chatgpt-app` CLI tool. The template provides a starting point for building a ChatGPT powered web app using Rust and Rocket.

## Usage

To create a new ChatGPT web application, use the `create-chatgpt-app` CLI tool:

```sh
cargo install create_chatgpt_app
create_chatgpt_app my-app
```

This will create a new directory called my-app in your current folder with the pre-built ChatGPT template.

## Project Structure

The template project has the following structure:

```sh
my-app
├── Cargo.toml
├── README.md
├── .gitignore
└── src
    ├── main.rs
    ├── chatgpt.rs
└── templates
    ├── index.html.hbs
```

## Running the App

Once you've created a new project using the template, navigate to the project directory and run:

```sh
cargo run
```

This command will compile and run the Rocket web application. By default, it will be available at http://localhost:8000.

## Customizing the App

You can customize the template project to suit your specific needs. For example, you can:

- Rename `.env.sample` to `.env` and add your ChatGPT API key.
- Modify the `src/main.rs` file to handle different endpoints and routes.
- Update the `src/chatgpt.r`s file to implement custom logic for interacting with the ChatGPT API.
- Add new modules or crates to extend the functionality of your web application.

## License

This ChatGPT template project is open-source software, licensed under the [MIT License](LICENSE).

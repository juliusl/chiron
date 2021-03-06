# Chiron

![Chiron preview](assets/chiron_preview.gif)

# Getting Started
- Install Rust, you can go here: [rustup](https://rustup.rs/), to install rust

- If you don't want to clone the repo, then run 
```
cargo install --git https://github.com/juliusl/chiron.git chiron
```

- If you just want to try it out, run 
```
cargo run
```` 

- To install as a command run 
```
cargo install --path .
```
and then the tool will launch with just, 
```sh
chiron
```

## Enable logging 
To enable logging, set `RUST_LOG` env variable. 

Here are some examples:
```
RUST_LOG=lifec=info  chiron
RUST_LOG=lifec=debug chiron
RUST_LOG=lifec=trace chiron

// or 

RUST_LOG=lifec=info  cargo run
RUST_LOG=lifec=debug cargo run
RUST_LOG=lifec=trace cargo run
```

Windows, macos, and linux are supported. 

# Background
In Greek mythology Chiron was called the "wisest and justest of all the centaurs", and was the teacher and mentor to many heroes. His students included Achilles, Ajax, Heracles, Jason, Odysseus, Perseus, and many others. **Project Chiron** is a customizable platform designed to transform new developers into heroes. 

The main goal is to make the process of setting up a developer environment accessible, educational, and productive. This is achieved by introducing two concepts, **components** and **labs**. 

**Components** automate the majority of setting up a developer machine. **Labs** are also components, but are semi-automatic and require some user-interaction. Mentors can use labs as an entry point to include more context and guidance. By completing labs, mentees will gain the skills and knowledge they need to use the lab as a component in their workflow.  

Using the Chiron platform, mentors will be able to create, share, and test new components and labs, custom tailoring them for their own use-cases. Mentors can build on top of a mentee's completed **lab history** and unlock additional components and labs. This allows mentors the ability to guide and monitor mentees on their journey, ensuring they are able to build self-confidence, and realize their full potential to become a dev **legend**.

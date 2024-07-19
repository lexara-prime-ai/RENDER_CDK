# Contributing to Render CDK

Thank you for considering contributing to the Render CDK project! Your contributions help improve the project and benefit the entire community. Below are some guidelines to help you get started.

## Code of Conduct

By participating in this project, you agree to abide by the [Code of Conduct](CODE_OF_CONDUCT.md). Please read it to understand the expected behavior and responsibilities of all contributors.

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please report it by creating a new issue on GitHub. Before doing so, please search the existing issues to ensure that the bug hasn't already been reported. When reporting a bug, please provide:

-   A clear and descriptive title.
-   A detailed description of the problem, including steps to reproduce it.
-   Any relevant logs, screenshots, or code snippets.
-   The version of the crate you are using and any other relevant environment details.

### Suggesting Enhancements

If you have an idea for a new feature or an improvement to an existing feature, please create a new issue on GitHub. When suggesting an enhancement, please include:

-   A clear and descriptive title.
-   A detailed description of the enhancement and why it would be useful.
-   Any relevant examples or use cases.

### Submitting Pull Requests

1.  **Fork the Repository**: Fork the repository to your own GitHub account and clone it to your local machine.
    
```sh
git clone https://github.com/YOUR-USERNAME/render_cdk.git
cd render_cdk
``` 
    
2.  **Create a Branch**: Create a new branch for your work.
    
```sh
git checkout -b feature/your-feature-name
``` 
    
3.  **Make Changes**: Make your changes in the new branch. Ensure your code follows the project's coding style and conventions.
    
4.  **Test Your Changes**: Run the tests to ensure your changes don't break existing functionality. Add new tests if your changes add new features.
    
```sh
cargo test
``` 
    
5.  **Commit Your Changes**: Commit your changes with a clear and descriptive commit message.
    
```sh
git commit -m "Add feature: description of the feature"
``` 
    
6.  **Push Your Changes**: Push your changes to your forked repository.
    
```sh
git push origin feature/your-feature-name
``` 
    
7.  **Create a Pull Request**: Open a pull request from your forked repository to the main repository. In the pull request description, explain the changes you made and why they are necessary.
    

### Code Review

All pull requests will be reviewed by the maintainers. Please be patient, as this process can take some time. The maintainers may request changes or additional tests before merging your contribution.

## Style Guide

-   **Rust Style**: Follow the Rust coding conventions. Use `rustfmt` to format your code.
-   **Documentation**: Write clear and concise documentation for your code. Use doc comments (`///`) for public items.
-   **Commit Messages**: Use clear and descriptive commit messages. Follow the convention: `Add feature: description of the feature`.

## Community

Join the community by participating in discussions, answering questions, and providing feedback. Your involvement helps make the project better for everyone.

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.

Thank you for your contributions!
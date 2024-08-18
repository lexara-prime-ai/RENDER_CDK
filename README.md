[![Cargo Audit](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/audit.yaml/badge.svg)](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/audit.yaml)
[![Run unit tests](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/tests.yml/badge.svg)](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/tests.yml)
[![Code Coverage](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/code-coverage.yml/badge.svg)](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/code-coverage.yml)
[![Publish to crates.io](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/cargo-publish.yml/badge.svg)](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/cargo-publish.yml)
[![Publish to Docker Hub](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/lexara-prime-ai/RENDER_CDK/actions/workflows/docker-publish.yml)


# Render CDK Documentation

- Reference Documentation - [Reference](https://cdk-c1wu.onrender.com/)
- Technical Documentation(Rust) - [Markdown](https://github.com/lexara-prime-ai/RENDER_CDK/blob/master/render_cdk/README.md)
- Technical Documentation(CPP) - _Coming Soon_

![docs](https://github.com/lexara-prime-ai/RENDER_CDK/blob/master/docs/previews/docs.jpeg?raw=true)

## Overview

`render_cdk` provides a streamlined interface for interacting with Render, a platform that allows you to build, deploy, and scale your apps with ease. This crate _abstracts_ Render's API, making it easier to work with Render cloud _**programmatically**_.

### Crate Information

- **Name:** render_cdk
- **Version:** 0.0.19
- **License:** MIT

### Current Features

Work on the resource management module is currently under way. The API supports many of the same actions available from the Render Dashboard. It currently provides endpoints for managing:

- Services
- Deploys
- Custom domains
- Jobs

The CDK will provide an abstraction that will make it easier to work with the Render cloud programmatically.

### To do
_Automate_ tests for deployed services _e.g Connecting and running queries on created database instances etc._

# Render CDK Crate Documentation

The `render_cdk` crate provides a comprehensive toolkit for managing cloud environments, IaaS, resource management, and state management within a Continuous Deployment/Continuous Integration (CDK) environment. This documentation provides an overview of the primary components and usage examples to help you get started.

## Modules

- **environment_management**: Tools and utilities for managing the deployment environment.
- **iaas**: Infrastructure as a Service functionalities.
- **resource_management**: Tools for managing resources, including models and utilities.
- **state_management**: Utilities for managing the state of deployed services.
- **logger**: Logging utilities for debugging and tracking operations.

## Usage

To use `render_cdk`, include the following in your `Cargo.toml`:

```toml
[dependencies]
render_cdk = "0.0.19"
```

* Examples can be found in the:  [Technical Documentation](https://github.com/lexara-prime-ai/RENDER_CDK/blob/master/render_cdk/README.md)

## Contributing

Contributions are welcome! Please see the [repository](CONTRIBUTING.md) for more information on how to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/lexara-prime-ai/MPESA_SDK/blob/master/LICENSE) file for details.

## Contact

For questions, issues, or suggestions, please open an issue on the [repository](https://github.com/lexara-prime-ai/RENDER_CDK).

Thank you for using `render_cdk`! We hope this documentation helps you get started quickly.

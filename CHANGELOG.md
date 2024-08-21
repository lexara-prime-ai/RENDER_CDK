# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- **Redis Management:**
  - `delete_redis_instance`: Deletes a Redis instance by name.
  - `find_redis_instance_by_name`: Finds Redis instances by name with an optional limit.

- **Macros:**
  - `handle_response_data!`: A reusable macro to handle API responses, including success and error logging, JSON parsing, and handling of empty service lists.

### Changed
- Standardized response handling and logging for API interactions, reducing code duplication and improving clarity.

---

*Note: This changelog follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format and adheres to [Semantic Versioning](https://semver.org/).*
# Render CDK - C++ Documentation

A C++ library for `render_cdk` also exists, providing a streamlined interface for interacting with Render's API. The library abstracts Render's API, making it easier to work with Render cloud programmatically in C++.

For more details and sample code, please visit the [repository](https://github.com/lexara-prime-api/RENDER_CDK).

## Building the Project

1. Navigate to your _project_ directory:

```sh
cd librender_cdk
```

2. Create a _build_ directory and navigate into it:

```sh
mkdir build
```

```sh
cd build
```

3. Run **CMake** to generate the build files:

```sh
cmake ..
```

4. Build the project:

```sh
make
```

This should compile the `main.cpp` file and link it against the `curl` and `jsoncpp` libraries, resulting in an executable named `list_services`.

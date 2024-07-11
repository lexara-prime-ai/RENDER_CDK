provider "render" {
    api_key = "rnd_xxxxXXXXxxxxXXXXxxxXX"
}

resource "render_service" "example" {
    name        = "example-service"
    environment = "production"
    replicas    = 3
}

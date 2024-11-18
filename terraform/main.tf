provider "github" {
    token = var.github_token
    owner = var.github_owner
}

resource "github_repository" "RENDER_CDK" {
    name = "RENDER_CDK"
    description = "A simple approach to deploying and managing resources on Render Cloud."
    visibility = "public"
}


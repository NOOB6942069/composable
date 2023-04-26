
data "github_user" "docs" {
  username = "JafarAz"
}


data "github_user" "ops" {
  username = "dzmitry-lahoda"
}

data "github_user" "bot" {
  username = "g-la-d-os"
}

data "github_team" "sre" {
  slug = "sre"
}

data "github_team" "devs" {
  slug = "developers"
}

data "github_team" "product" {
  slug = "product-mgmt"
}

resource "github_repository_collaborators" "roles" {
  repository = "composable"

  user {
    permission = "write"
    username   = data.github_team.devs.name
  }

  user {
    permission = "admin"
    username   = data.github_user.ops.name
  }

  user {
    permission = "write"
    username   = data.github_user.docs.name
  }

  user {
    permission = "admin"
    username   = data.github_user.bot.name
  }
  user {
    permission = "admin"
    username   = data.github_team.product.name
  }

  user {
    permission = "admin"
    username   = data.github_team.sre.name
  }
}

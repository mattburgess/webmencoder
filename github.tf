terraform {
  required_version = "0.13.5"

  required_providers {
    github = {
      source  = "hashicorp/github"
      version = "~> 4.0.0"
    }
  }
}

provider "github" {
  token = var.github_token
}

variable "github_token" {
  type        = string
  description = "The GitHub Personal Access Token (PAT) to use for authentication"
}

resource "github_repository" "webmencoder" {
  name                   = "webmencoder"
  description            = "An opinionated CLI for converting MPEG to WebM"
  has_issues             = true
  has_projects           = true
  has_wiki               = true
  allow_squash_merge     = true
  allow_merge_commit     = false
  allow_rebase_merge     = false
  delete_branch_on_merge = true
  default_branch         = "main"
  vulnerability_alerts   = "true"
}

resource "github_branch_protection" "main" {
  repository_id          = github_repository.webmencoder.node_id
  pattern                = "main"
  enforce_admins         = false
  require_signed_commits = true

  required_status_checks {
    strict   = true
    contexts = ["Lint"]
  }

  required_pull_request_reviews {
    dismiss_stale_reviews = true
  }
}

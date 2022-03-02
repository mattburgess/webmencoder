terraform {
  required_version = "1.1.7"

  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 4.20.0"
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
  #ts:skip=AC_GITHUB_0002 repo is intentionally public
  name                   = "webmencoder"
  description            = "An opinionated CLI for converting MPEG to WebM"
  has_issues             = true
  has_projects           = true
  has_wiki               = true
  allow_squash_merge     = true
  allow_merge_commit     = false
  allow_rebase_merge     = false
  delete_branch_on_merge = true
  vulnerability_alerts   = "true"
}

resource "github_branch_default" "main" {
  repository = github_repository.webmencoder.name
  branch     = "main"
}

resource "github_branch_protection" "main" {
  repository_id          = github_repository.webmencoder.node_id
  pattern                = "main"
  enforce_admins         = false
  require_signed_commits = true

  required_status_checks {
    strict   = true
    contexts = ["Lint", "clippy"]
  }

  required_pull_request_reviews {
    dismiss_stale_reviews = true
  }
}

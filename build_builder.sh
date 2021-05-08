docker build builder -t rbp-rust-builder
docker tag rbp-rust-builder registry.gitlab.com/katlex/github-runner:rbp-rust-builder
docker push registry.gitlab.com/katlex/github-runner:rbp-rust-builder

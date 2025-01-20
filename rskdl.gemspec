# frozen_string_literal: true

require_relative "lib/rskdl/version"

Gem::Specification.new do |spec|
  spec.name = "rskdl"
  spec.version = RsKDL::VERSION
  spec.authors = ["Dani Smith"]
  spec.email = ["code@danini.dev"]

  spec.summary = "Rust parser for kdl-rb"
  spec.description = "Use the kdl-rs parser to create kdl-rb documents"
  spec.homepage = "https://github.com/danini-the-panini/rskdl-rb"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.6.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/danini-the-panini/rskdl-rb"
  spec.metadata["changelog_uri"] = "https://github.com/danini-the-panini/rskdl-rb/releases"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/rskdl/Cargo.toml"]

  spec.add_dependency "kdl", "~> 2.0"
end

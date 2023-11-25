# frozen_string_literal: true

require "kdl"

require_relative "kdl_rust_parser/version"
require_relative "kdl_rust_parser/kdl_rust_parser"

module KdlRustParser
  extend self
end

KDL.singleton_class.prepend(KdlRustParser)
# frozen_string_literal: true

require "kdl"

require_relative "rskdl/version"
require_relative "rskdl/rskdl"

module RsKDL
  extend self
end

KDL.singleton_class.prepend(RsKDL)
